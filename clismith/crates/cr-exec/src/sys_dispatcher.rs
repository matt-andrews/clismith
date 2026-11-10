use std::collections::HashMap;

use serde_json::{Value, to_value};

use crate::{
    invocation::Invocation,
    manifest::{ManifestDisplayModule, ManifestModule},
    utils::get_manifest,
};

const SYS_CMD_MANIFEST_LIST: &str = "SYS_CMD_MANIFEST_LIST";

pub fn dispatch(inv: &Invocation) -> Option<Value> {
    //
    match inv
        .triggered_command
        .target
        .clone()
        .unwrap_or_default()
        .as_str()
    {
        SYS_CMD_MANIFEST_LIST => list_manifest(inv),
        _ => None,
    }
}

fn list_manifest(inv: &Invocation) -> Option<Value> {
    //
    let manifest = get_manifest().unwrap_or_else(|_| panic!("cannot find manifest"));
    let module_filter = inv.str("module");

    if let Some(module_filter) = module_filter {
        //
        let mods: Vec<ManifestDisplayModule> = manifest
            .modules
            .into_iter()
            .filter(|f| f.name == module_filter)
            .map(|m| m.to_display_module())
            .collect();

        to_value(mods).ok()
    } else {
        //
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
}
