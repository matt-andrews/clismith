use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::v1spec::{
    diff::{Bump, Diff, cmp_field, diff_by_key, diff_opt_by_key},
    param::ParamSpec,
    value_kind::ValueKind,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CoreSpec {
    pub name: String,
    #[serde(default = "vspec")]
    pub vspec: String,
    pub about: Option<String>,
    pub grammar: Grammar,

    #[serde(default)]
    pub required_modules: Option<Vec<RequiredModule>>,

    #[serde(default)]
    pub is_singleton: Option<bool>,

    #[serde(default)]
    pub registries: Option<Vec<Registry>>,

    #[serde(default)]
    pub global_flags: Vec<ParamSpec>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Grammar {
    pub pattern: String,
    pub naming: Naming,
    pub verbs: Vec<VerbDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Naming {
    pub case: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerbDefinition {
    pub method: String,
    pub target: String,
    pub verb: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequiredModule {
    pub module: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Registry {
    pub index: String,
    pub secure: bool,
}

impl CoreSpec {
    pub fn init_default_template(name: &str, about: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            about,
            vspec: "v1".to_string(),
            grammar: Grammar {
                naming: Naming {
                    case: "kebab".to_string(),
                },
                pattern: "{ module } { resource? } { verb }".to_string(),
                verbs: vec![
                    VerbDefinition {
                        method: "GET".to_string(),
                        target: "item".to_string(),
                        verb: "get".to_string(),
                    },
                    VerbDefinition {
                        method: "GET".to_string(),
                        target: "collection".to_string(),
                        verb: "list".to_string(),
                    },
                    VerbDefinition {
                        method: "POST".to_string(),
                        target: "collection".to_string(),
                        verb: "create".to_string(),
                    },
                    VerbDefinition {
                        method: "POST".to_string(),
                        target: "item".to_string(),
                        verb: "create".to_string(),
                    },
                    VerbDefinition {
                        method: "PUT".to_string(),
                        target: "item".to_string(),
                        verb: "update".to_string(),
                    },
                    VerbDefinition {
                        method: "PATCH".to_string(),
                        target: "item".to_string(),
                        verb: "update".to_string(),
                    },
                    VerbDefinition {
                        method: "DELETE".to_string(),
                        target: "item".to_string(),
                        verb: "delete".to_string(),
                    },
                ],
            },
            required_modules: Some(vec![RequiredModule {
                module: "manifest".to_string(),
                version: "*".to_string(),
            }]),
            global_flags: vec![
            ParamSpec {
                long: "verbose".to_string(),
                short: Some('v'),
                about: Some("Enable verbose logging".to_string()),
                kind: ValueKind::Bool,
                target: Some("SYS_FLAG_VERBOSE".to_string()),
                ..Default::default()
            },
            ParamSpec {
                long: "head".to_string(),
                short: Some('I'),
                about: Some(
                    "Force the output to be request headers instead of content".to_string(),
                ),
                target: Some("SYS_FLAG_HEAD".to_string()),
                ..Default::default()
            },
            ParamSpec {
                long: "output".to_string(),
                short: Some('o'),
                about: Some("Set the printed structure".to_string()),
                target: Some("SYS_FLAG_OUTPUT".to_string()),
                default: Some("json".to_string()),
                choices: Some(vec![
                    "json".to_string(),
                    "table".to_string(),
                    "none".to_string(),
                ]),
                ..Default::default()
            },
            ParamSpec {
                long: "output-prop".to_string(),
                about: Some("Select which properties are output".to_string()),
                target: Some("SYS_FLAG_OUTPUT_PROP".to_string()),
                multiple: true,
                ..Default::default()
            },
            ParamSpec {
                long: "output-sort".to_string(),
                about: Some(
                    "Sort output by given property. Prefix the property with `-` for descending"
                        .to_string(),
                ),
                target: Some("SYS_FLAG_OUTPUT_SORT".to_string()),
                multiple: true,
                allow_hyphen_values: Some(true),
                ..Default::default()
            },
        ],
            ..Default::default()
        }
    }
}

impl Diff for CoreSpec {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(out, path, "name", &self.name, &new.name, Bump::Major);
        cmp_field(out, path, "about", &self.about, &new.about, Bump::Minor);

        self.grammar
            .diff(&new.grammar, &format!("{path}.grammar"), out);

        diff_opt_by_key(
            &self.required_modules,
            &new.required_modules,
            &format!("{path}.required_modules"),
            out,
            |key| &key.module,
        );

        diff_opt_by_key(
            &self.registries,
            &new.registries,
            &format!("{path}.registries"),
            out,
            |key| &key.index,
        );

        diff_by_key(
            &self.global_flags,
            &new.global_flags,
            &format!("{path}.global_flags"),
            out,
            |key| &key.long,
        );
    }
}

impl Diff for Grammar {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(
            out,
            path,
            "pattern",
            &self.pattern,
            &new.pattern,
            Bump::Major,
        );

        self.naming
            .diff(&new.naming, &format!("{path}.naming"), out);

        diff_by_key(
            &self.verbs,
            &new.verbs,
            &format!("{path}.verbs"),
            out,
            |key| (&key.method, &key.target, &key.verb),
        );
    }
}

impl Diff for Naming {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(out, path, "case", &self.case, &new.case, Bump::Major);
    }
}

impl Diff for VerbDefinition {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(out, path, "method", &self.method, &new.method, Bump::Major);
        cmp_field(out, path, "target", &self.target, &new.target, Bump::Major);
        cmp_field(out, path, "verb", &self.verb, &new.verb, Bump::Major);
    }
}

impl Diff for RequiredModule {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(out, path, "module", &self.module, &new.module, Bump::Major);
        cmp_field(
            out,
            path,
            "version",
            &self.version,
            &new.version,
            Bump::Major,
        );
    }
}

impl Diff for Registry {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(out, path, "index", &self.index, &new.index, Bump::Major);
        cmp_field(out, path, "secure", &self.secure, &new.secure, Bump::Major);
    }
}

fn vspec() -> String {
    "v1".to_string()
}
