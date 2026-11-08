use std::{collections::HashMap, path::Path};

use serde_json::{Value, to_value};

use crate::{
    invocation::Invocation,
    manifest::{ManifestDisplayModule, ManifestModule},
    utils::get_manifest,
};

const SYS_CMD_MANIFEST_LIST: &str = "SYS_CMD_MANIFEST_LIST";

pub fn dispatch(inv: &Invocation, dir: &Path) -> Option<Value> {
    //
    match inv
        .triggered_command
        .target
        .clone()
        .unwrap_or_default()
        .as_str()
    {
        SYS_CMD_MANIFEST_LIST => list_manifest(dir),
        _ => None,
    }
}

fn list_manifest(dir: &Path) -> Option<Value> {
    //
    let manifest = get_manifest(dir).unwrap_or_else(|_| panic!("cannot find manifest"));

    let deduped_mods: HashMap<String, ManifestModule> = manifest
        .modules
        .into_iter()
        .map(|m| (m.name.clone(), m))
        .collect();

    let flat_result: Vec<ManifestDisplayModule> = deduped_mods
        .iter()
        .map(|m| m.1.to_display_module())
        .collect();

    to_value(flat_result).ok()
}
