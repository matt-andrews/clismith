use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ffi::OsString,
    path::{Path, PathBuf},
};

use clap::{Arg, ArgAction, ArgMatches, Command, builder::PossibleValuesParser, value_parser};
use cr_core::{
    helpers::deserialize_yaml_from_path,
    v1spec::{
        compiled::{
            core::CoreCompiled,
            module::{CommandCompiled, ModuleCompiled},
            param::ParamCompiled,
        },
        value_kind::ValueKind,
    },
};

use crate::{
    errors::BuildError,
    invocation::{Invocation, Value},
    pattern::{Pattern, Segment},
};

#[derive(Debug, Default)]
struct Node {
    children: BTreeMap<String, Edge>,
    leaf: Option<usize>,
}

#[derive(Debug)]
struct Edge {
    slot: Option<String>,
    first: usize,
    node: Node,
}

fn meaning(slot: &Option<String>) -> String {
    match slot {
        Some(s) => format!("`{s}`"),
        None => "fixed word".into(),
    }
}

#[derive(Debug)]
pub struct CommandTree {
    pattern: Pattern,
    globals: Vec<ParamCompiled>,
    specs: Vec<CommandCompiled>,
    root: Node,
    clap: Command,
    triggered_command_spec: Option<CommandCompiled>,
}

#[derive(Debug)]
pub struct Compilation {
    pub core: CoreCompiled,
    pub module: ModuleCompiled,
}

impl CommandTree {
    pub fn from_yaml(dir: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config: Compilation = Compilation {
            core: deserialize_yaml_from_path(dir, "core.yml")?,
            module: deserialize_yaml_from_path(dir, "billing-module.yml")?,
        };
        Ok(Self::compile(config)?)
    }

    pub fn compile(config: Compilation) -> Result<Self, BuildError> {
        let pattern = Pattern::parse(&config.core.grammar.pattern)?;
        let slots: Vec<&str> = pattern.slots().collect();

        validate_params("global flags", &config.core.global_flags, &[], false)?;

        let mut root = Node::default();
        let mut seen: HashMap<Vec<String>, usize> = HashMap::new();
        let mut paths: Vec<String> = Vec::with_capacity(config.module.commands.len());
        let mut triggered_spec: Option<CommandCompiled> = None;

        for (index, spec) in config.module.commands.iter().enumerate() {
            let desc = spec.describe();
            for key in spec.bindings.keys() {
                if !slots.contains(&key.as_str()) {
                    return Err(BuildError::UnknownSlot {
                        index,
                        desc,
                        key: key.clone(),
                    });
                }
            }

            let mut path: Vec<(String, Option<String>)> =
                Vec::with_capacity(pattern.segments().len());
            for seg in pattern.segments() {
                match seg {
                    Segment::Literal(word) => path.push((word.clone(), None)),
                    Segment::Slot { name, optional } => {
                        let Some(value) = spec.bindings.get(name) else {
                            if *optional {
                                continue;
                            }
                            return Err(BuildError::MissingSlot {
                                index,
                                desc,
                                slot: name.clone(),
                            });
                        };
                        if let Some(reason) = invalid_word(value) {
                            return Err(BuildError::InvalidValue {
                                index,
                                slot: name.clone(),
                                value: value.clone(),
                                reason,
                            });
                        }
                        path.push((value.clone(), Some(name.clone())));
                    }
                }
            }
            if path.is_empty() {
                return Err(BuildError::EmptyPath { index, desc });
            }

            let words: Vec<String> = path.iter().map(|(w, _)| w.clone()).collect();
            let joined = words.join(" ");
            if let Some(&first) = seen.get(&words) {
                return Err(BuildError::Duplicate {
                    first,
                    second: index,
                    path: joined,
                });
            }
            validate_params(&joined, &spec.args, &config.core.global_flags, true)?;
            validate_params(
                &joined,
                &spec.flags,
                &[config.core.global_flags.as_slice(), spec.args.as_slice()].concat(),
                false,
            )?;

            let mut node = &mut root;
            for (depth, (word, slot)) in path.iter().enumerate() {
                let edge = node.children.entry(word.clone()).or_insert_with(|| Edge {
                    slot: slot.clone(),
                    first: index,
                    node: Node::default(),
                });
                if edge.slot != *slot {
                    return Err(BuildError::AmbiguousWord {
                        path: words[..=depth].join(" "),
                        word: word.clone(),
                        first: edge.first,
                        first_meaning: meaning(&edge.slot),
                        second: index,
                        second_meaning: meaning(slot),
                    });
                }
                node = &mut edge.node;
                if depth + 1 < path.len()
                    && let Some(leaf) = node.leaf
                {
                    return Err(BuildError::LeafAndBranch {
                        leaf,
                        leaf_path: paths[leaf].clone(),
                        branch: index,
                        branch_path: joined,
                    });
                }
            }
            if let Some(child) = node.children.values().next() {
                return Err(BuildError::LeafAndBranch {
                    leaf: index,
                    leaf_path: joined,
                    branch: child.first,
                    branch_path: paths[child.first].clone(),
                });
            }
            node.leaf = Some(index);
            seen.insert(words, index);
            paths.push(joined);
            triggered_spec = Some(spec.clone());
        }

        let mut tree = Self {
            pattern,
            globals: config.core.global_flags,
            specs: config.module.commands,
            root,
            clap: Command::new(""),
            triggered_command_spec: triggered_spec,
        };
        tree.clap = tree.build_clap(&config.core.name, config.core.about.as_deref());
        Ok(tree)
    }

    // generated clap command
    pub fn command(&self) -> Command {
        self.clap.clone()
    }

    pub fn parse(&self) -> Result<Invocation, clap::Error> {
        self.parse_from(std::env::args_os())
    }

    pub fn parse_from<I, T>(&self, argv: I) -> Result<Invocation, clap::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let matches = self.command().try_get_matches_from(argv)?;

        let mut m = &matches;
        let mut node = &self.root;
        let mut path = Vec::new();
        while let Some((word, sub)) = m.subcommand() {
            path.push(word.to_string());
            node = &node.children[word].node;
            m = sub;
        }
        let index = node
            .leaf
            .expect("clap requires a subcommand at every non-leaf level");
        let spec = &self.specs[index];

        let mut params = BTreeMap::new();
        for p in self.globals.iter().chain(&spec.args).chain(&spec.flags) {
            if let Some(v) = extract(p, m) {
                if let Some(target) = &p.target {
                    params.insert(target.clone(), v);
                } else {
                    params.insert(p.long.clone(), v);
                }
            }
        }
        Ok(Invocation {
            path,
            bindings: spec.bindings.clone(),
            params,
            triggered_command: self.triggered_command_spec.clone(),
        })
    }

    fn build_clap(&self, name: &str, about: Option<&str>) -> Command {
        let mut root = Command::new(name.to_string()).bin_name(name.to_string());
        if let Some(a) = about {
            root = root.about(a.to_string());
        }
        for g in &self.globals {
            root = root.arg(to_arg(g, false).global(true));
        }
        self.build_level(root, &self.root)
    }

    fn build_level(&self, mut cmd: Command, node: &Node) -> Command {
        if let Some(index) = node.leaf {
            let spec = &self.specs[index];
            for a in &spec.args {
                cmd = cmd.arg(to_arg(a, true));
            }
            for f in &spec.flags {
                cmd = cmd.arg(to_arg(f, false));
            }
            return cmd;
        }

        cmd = cmd.subcommand_required(true).arg_required_else_help(true);

        let filled: Vec<&str> = self
            .pattern
            .slots()
            .filter(|s| {
                node.children
                    .values()
                    .any(|e| e.slot.as_deref() == Some(*s))
            })
            .collect();
        if !filled.is_empty() {
            let value_name = filled
                .iter()
                .map(|s| s.to_uppercase())
                .collect::<Vec<_>>()
                .join("|");
            let heading = filled
                .iter()
                .map(|s| heading(s))
                .collect::<Vec<_>>()
                .join(" / ");
            cmd = cmd
                .subcommand_value_name(value_name)
                .subcommand_help_heading(heading);
        }

        for (word, edge) in &node.children {
            let mut sub = Command::new(word.clone());
            let about = edge.node.leaf.and_then(|i| self.specs[i].about.clone());
            if let Some(a) = about {
                sub = sub.about(a);
            }
            cmd = cmd.subcommand(self.build_level(sub, &edge.node));
        }
        cmd
    }
}

fn heading(slot: &str) -> String {
    let mut c = slot.chars();
    let first = c.next().map(|f| f.to_ascii_uppercase()).unwrap_or_default();
    format!("{first}{}s", c.as_str())
}

fn invalid_word(w: &str) -> Option<&'static str> {
    if w.is_empty() {
        Some("empty")
    } else if w.starts_with('-') {
        Some("would be parsed as a flag")
    } else if w.chars().any(char::is_whitespace) {
        Some("contains whitespace")
    } else if w == "help" {
        Some("`help` is reserved")
    } else {
        None
    }
}

fn validate_params(
    context: &str,
    params: &[ParamCompiled],
    existing: &[ParamCompiled],
    positional: bool,
) -> Result<(), BuildError> {
    let err = |p: &ParamCompiled, reason: &str| BuildError::InvalidParam {
        context: context.to_string(),
        name: p.long.clone(),
        reason: reason.to_string(),
    };
    let mut names: HashSet<&str> = existing.iter().map(|p| p.long.as_str()).collect();
    let mut shorts: HashSet<char> = existing.iter().filter_map(|p| p.short).collect();

    for (i, p) in params.iter().enumerate() {
        let valid_name = p
            .long
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic())
            && p.long
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');

        if !valid_name {
            return Err(err(p, "has an invalid name"));
        }
        if p.long == "help" || p.short == Some('h') {
            return Err(err(p, "clashes with the built-in --help/-h"));
        }
        if !names.insert(&p.long) {
            return Err(err(p, "is declared more than once"));
        }
        if let Some(s) = p.short {
            if positional {
                return Err(err(p, "is positional and can't have a short flag"));
            }
            if !s.is_ascii_alphanumeric() || !shorts.insert(s) {
                return Err(err(p, "has an invalid or duplicate short flag"));
            }
        }
        if p.kind == ValueKind::Bool {
            if positional {
                return Err(err(p, "is positional and can't be a bool"));
            }
            if p.multiple || p.default.is_some() || p.choices.is_some() || p.required {
                return Err(err(
                    p,
                    "is a bool flag and can't be multiple, required, or have a default or choices",
                ));
            }
        }
        if let Some(choices) = &p.choices {
            if p.kind != ValueKind::String {
                return Err(err(
                    p,
                    "has choices, which are only supported for string values",
                ));
            }
            if let Some(d) = &p.default
                && !choices.contains(d)
            {
                return Err(err(
                    p,
                    &format!("has default {d:?}, which isn't one of its choices"),
                ));
            }
        }
        if positional && p.multiple && i + 1 != params.len() {
            return Err(err(
                p,
                "takes multiple values, so it must be the last positional",
            ));
        }
    }
    Ok(())
}

fn to_arg(p: &ParamCompiled, positional: bool) -> Arg {
    let mut a = Arg::new(p.long.clone());
    if let Some(h) = &p.about {
        a = a.help(h.clone());
    }
    if !positional {
        a = a.long(p.long.clone());
        if let Some(s) = p.short {
            a = a.short(s);
        }
    }
    if p.kind == ValueKind::Bool {
        return a.action(ArgAction::SetTrue);
    }

    a = match (&p.choices, p.kind) {
        (Some(c), _) => a.value_parser(PossibleValuesParser::new(c.clone())),
        (None, ValueKind::Int) => a.value_parser(value_parser!(i64)),
        (None, ValueKind::Float) => a.value_parser(value_parser!(f64)),
        (None, ValueKind::Path) => a.value_parser(value_parser!(PathBuf)),
        (None, _) => a.value_parser(value_parser!(String)),
    };
    a = match (p.multiple, positional) {
        (true, true) => a.num_args(1..).action(ArgAction::Append),
        (true, false) => a.action(ArgAction::Append),
        (false, _) => a.action(ArgAction::Set),
    };
    if let Some(d) = &p.default {
        a = a.default_value(d.clone());
    }

    let required = p.required && p.default.is_none();
    a.required(required)
}

fn extract(p: &ParamCompiled, m: &ArgMatches) -> Option<Value> {
    fn grab<T: Clone + Send + Sync + 'static>(
        m: &ArgMatches,
        name: &str,
        multiple: bool,
        wrap: fn(T) -> Value,
    ) -> Option<Value> {
        if multiple {
            m.get_many::<T>(name)
                .map(|vs| Value::List(vs.cloned().map(wrap).collect()))
        } else {
            m.get_one::<T>(name).cloned().map(wrap)
        }
    }

    let n = p.long.as_str();
    match p.kind {
        ValueKind::Bool => Some(Value::Bool(m.get_flag(n))),
        ValueKind::Int => grab(m, n, p.multiple, Value::Int),
        ValueKind::Float => grab(m, n, p.multiple, Value::Float),
        ValueKind::Path => grab(m, n, p.multiple, Value::Path),
        ValueKind::String => grab(m, n, p.multiple, Value::Str),
    }
}
