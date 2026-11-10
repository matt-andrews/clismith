use std::collections::BTreeSet;

use clap::{Arg, Command};
use cr_core::v1spec::{core::CoreSpec, index::Index};

use crate::runtime::{
    errors::BuildError,
    pattern::{Pattern, Segment},
    utils::param_to_arg,
};
//TODO: we no longer need a complex top level selection; since sys has moved to its own built-in app
pub enum Selection {
    Module(String),
}

fn branch(cmd: Command) -> Command {
    cmd.subcommand_required(true).arg_required_else_help(true)
}

fn stub(name: &str) -> Command {
    Command::new(name.to_string())
        .disable_help_flag(true)
        .disable_help_subcommand(true)
        .arg(
            Arg::new("rest")
                .num_args(0..)
                .trailing_var_arg(true)
                .allow_hyphen_values(true)
                .hide(true),
        )
}

pub fn skeleton(core: &CoreSpec, manifest: &Index) -> Result<Command, BuildError> {
    let pattern = Pattern::parse(&core.grammar.pattern)?;

    let mut names = vec![core.name.clone()];
    for seg in pattern.segments() {
        match seg {
            Segment::Literal(word) => names.push(word.clone()),
            Segment::Slot { name, .. } if name == "module" => break,
            Segment::Slot { .. } => todo!("a slot precedes module"),
        }
    }

    let modules: BTreeSet<&str> = manifest.modules.iter().map(|m| m.name.as_str()).collect();
    let mut cmd =
        branch(Command::new(names.pop().unwrap())).subcommands(modules.into_iter().map(stub));

    while let Some(name) = names.pop() {
        cmd = branch(Command::new(name)).subcommand(cmd);
    }

    for g in &core.global_flags {
        cmd = cmd.arg(param_to_arg(g, false).global(true));
    }

    Ok(cmd
        .bin_name(core.name.clone())
        .subcommand(Command::new("app-install").arg(Arg::new("index").required(true))))
}

pub fn select(skeleton: Command) -> Result<Selection, clap::Error> {
    let matches = skeleton.try_get_matches_from(std::env::args_os())?;
    let mut m = &matches;
    let mut last = "";
    while let Some((word, sub)) = m.subcommand() {
        last = word;
        m = sub;
    }
    Ok({
        let module = last;
        Selection::Module(module.to_string())
    })
}
