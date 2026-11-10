use std::collections::HashMap;

use cr_core::v1spec::index::{IndexDisplayModule, IndexModule};
use serde_json::{Value, to_value};

use crate::{invocation::Invocation, utils::get_manifest};

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
