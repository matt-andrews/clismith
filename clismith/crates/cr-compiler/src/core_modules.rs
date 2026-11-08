use std::{collections::BTreeMap, path::Path};

use cr_core::{
    helpers::{env_or, serialize_to_json},
    v1spec::compiled::module::{CommandCompiled, ModuleCompiled},
};

pub fn emit_core_modules(target_dir: &Path) -> anyhow::Result<()> {
    //
    serialize_to_json(&sys_manifest(), &target_dir.join("sys-manifest.json"))?;
    Ok(())
}

fn sys_manifest() -> ModuleCompiled {
    let module: &str = &env_or("MANIEFEST_MODULE_NAME", "manifest");
    ModuleCompiled {
        name: module.to_string(),
        about: Some("Commands for viewing and updating the manifest".to_string()),
        base_uri: None,
        commands: vec![CommandCompiled {
            target: Some("SYS_CMD_MANIFEST_LIST".to_string()),
            about: Some("List the available modules in the manifest".to_string()),
            args: Vec::new(),
            flags: Vec::new(),
            bindings: BTreeMap::from([
                ("verb".to_string(), "list".to_string()),
                ("module".to_string(), module.to_string()),
            ]),
            method: None,
            route: None,
        }],
    }
}
