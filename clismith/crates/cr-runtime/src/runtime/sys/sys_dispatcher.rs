use std::{collections::HashMap, path::PathBuf};

use cr_compiler::compile;
use cr_core::{
    dir_helpers::{DirType, get_dir_with, set_dir},
    v1spec::index::{IndexDisplayModule, IndexModule},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, to_value};

use crate::{
    installer::app_install,
    runtime::{
        invocation::Invocation,
        sys::{
            SYS_CMD_COMPILE_APP, SYS_CMD_COMPILE_VERSION, SYS_CMD_INSTALL_APP,
            SYS_CMD_MANIFEST_LIST,
        },
        utils::get_manifest,
    },
};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SimpleResult {
    pub result: String,
}

pub async fn dispatch(inv: &Invocation) -> Option<Value> {
    //
    match inv
        .triggered_command
        .target
        .clone()
        .unwrap_or_default()
        .as_str()
    {
        SYS_CMD_MANIFEST_LIST => list_manifest(inv),
        SYS_CMD_INSTALL_APP => install_app(inv).await,
        SYS_CMD_COMPILE_APP => compile_app(inv),
        SYS_CMD_COMPILE_VERSION => compiler_version(inv),
        _ => None,
    }
}

fn list_manifest(inv: &Invocation) -> Option<Value> {
    //
    let manifest = get_manifest().unwrap_or_else(|_| panic!("cannot find manifest"));
    let module_filter = inv.str("module");

    if let Some(module_filter) = module_filter {
        //
        let mods: Vec<IndexDisplayModule> = manifest
            .modules
            .into_iter()
            .filter(|f| f.name == module_filter)
            .map(IndexDisplayModule::from)
            .collect();

        to_value(mods).ok()
    } else {
        //
        let deduped_mods: HashMap<String, IndexModule> = manifest
            .modules
            .into_iter()
            .map(|m| (m.name.clone(), m))
            .collect();
        let flat_result: Vec<IndexDisplayModule> = deduped_mods
            .iter()
            .map(|m| IndexDisplayModule::from(m.1.to_owned()))
            .collect();

        to_value(flat_result).ok()
    }
}

async fn install_app(inv: &Invocation) -> Option<Value> {
    //
    let name_arg = inv
        .str("name")
        .unwrap_or_else(|| panic!("name arg is required"));

    //we set the dir here because we're currently in a system app.
    //once we have the full index we can move to the end-result dir
    set_dir(DirType::Exec, &get_dir_with(DirType::ExecAppData, name_arg));

    let index_arg = inv
        .str("index")
        .unwrap_or_else(|| panic!("index arg is required"));
    if let Err(err) = app_install(index_arg).await {
        Some(Value::from(format!("failed to install app: {}", err)))
    } else {
        Some(
            serde_json::to_value(SimpleResult {
                result: "Application successfully installed".to_string(),
            })
            .unwrap_or_default(),
        )
    }
}

fn compile_app(inv: &Invocation) -> Option<Value> {
    let src_arg = inv.str("src");
    let target_arg = inv.str("target");

    if let Some(src_arg) = src_arg {
        set_dir(DirType::Project, &PathBuf::from(src_arg));
    }

    if let Some(target_arg) = target_arg {
        set_dir(DirType::Target, &PathBuf::from(target_arg));
    }

    match compile() {
        Ok(_) => Some(
            serde_json::to_value(SimpleResult {
                result: "Application successfully compiled".to_string(),
            })
            .unwrap_or_default(),
        ),
        Err(e) => Some(Value::from(e.to_string())),
    }
}

fn compiler_version(_: &Invocation) -> Option<Value> {
    Some(
        serde_json::to_value(SimpleResult {
            result: env!("CARGO_PKG_VERSION").to_string(),
        })
        .unwrap_or_default(),
    )
}
