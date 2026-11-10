use std::collections::BTreeMap;

use cr_core::{
    helpers::env_or,
    v1spec::{
        module::{CommandSpec, ModuleSpec},
        param::ParamSpec,
    },
};

use crate::{emit::emit_module, models::MergeResponse};

pub fn emit_core_modules(context: &mut MergeResponse) -> anyhow::Result<()> {
    //
    emit_module(&sys_manifest(), &mut context.index)?;
    Ok(())
}

fn sys_manifest() -> ModuleSpec {
    let module: &str = &env_or("MANIEFEST_MODULE_NAME", "manifest");
    ModuleSpec {
        name: module.to_string(),
        about: Some("Commands for viewing and updating the manifest".to_string()),
        commands: vec![CommandSpec {
            target: Some("SYS_CMD_MANIFEST_LIST".to_string()),
            about: Some("List the available modules in the manifest".to_string()),
            args: vec![ParamSpec {
                target: Some("module".to_string()),
                long: "module".to_string(),
                about: Some("List out all versions for the specified module".to_string()),
                ..Default::default()
            }],
            bindings: BTreeMap::from([
                ("verb".to_string(), "list".to_string()),
                ("module".to_string(), module.to_string()),
            ]),
            ..Default::default()
        }],
        ..Default::default()
    }
}
