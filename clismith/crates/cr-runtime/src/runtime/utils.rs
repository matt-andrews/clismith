use std::path::PathBuf;

use clap::{Arg, ArgAction, builder::PossibleValuesParser, value_parser};
use cr_core::{
    dir_helpers::{DirType, get_dir_with, project_is_system},
    helpers::env_or,
    io_helpers::deserialize_from_file,
    v1spec::{
        core::CoreSpec,
        index::{Index, IndexModule},
        module::ModuleSpec,
        param::ParamSpec,
        value_kind::ValueKind,
    },
};

use crate::runtime::sys::sys_spec::{get_sys_core, get_sys_manifest, get_sys_module};

pub fn get_manifest() -> anyhow::Result<Index> {
    if project_is_system() {
        Ok(get_sys_manifest())
    } else {
        Ok(deserialize_from_file(&get_dir_with(
            DirType::Exec,
            &env_or("MANIFEST_NAME", "manifest.json"),
        ))?)
    }
}

pub fn get_core(manifest: &Index) -> anyhow::Result<CoreSpec> {
    if project_is_system() {
        Ok(get_sys_core())
    } else {
        Ok(deserialize_from_file(&get_dir_with(
            DirType::Exec,
            &manifest.core,
        ))?)
    }
}

pub fn get_module(module: &IndexModule) -> anyhow::Result<ModuleSpec> {
    if project_is_system() {
        match get_sys_module(&module.name) {
            Some(v) => Ok(v),
            None => Err(anyhow::anyhow!(
                "Specified module could not be found: {}",
                module.name
            )),
        }
    } else {
        Ok(deserialize_from_file(&get_dir_with(
            DirType::Exec,
            &module.target,
        ))?)
    }
}

pub fn param_to_arg(p: &ParamSpec, positional: bool) -> Arg {
    let mut a = Arg::new(p.long.clone());

    if p.allow_hyphen_values.unwrap_or_default() {
        a = a.allow_hyphen_values(true);
    }

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
