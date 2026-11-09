use std::{collections::BTreeMap, path::Path};

use cr_core::{
    helpers::env_or,
    v1spec::{
        authored::{
            module::{CommandSpec, ModuleSpec},
            param::ParamSpec,
        },
        param_where::ParamWhere,
        value_kind::ValueKind,
    },
};

use crate::{emit::emit_module, models::MergeResponse};

pub fn emit_core_modules(context: &mut MergeResponse, target_dir: &Path) -> anyhow::Result<()> {
    //
    emit_module(&sys_manifest(), target_dir, &mut context.index)?;
    Ok(())
}

fn sys_manifest() -> ModuleSpec {
    let module: &str = &env_or("MANIEFEST_MODULE_NAME", "manifest");
    ModuleSpec {
        name: module.to_string(),
        about: Some("Commands for viewing and updating the manifest".to_string()),
        base_uri: None,
        commands: vec![CommandSpec {
            target: Some("SYS_CMD_MANIFEST_LIST".to_string()),
            about: Some("List the available modules in the manifest".to_string()),
            args: vec![ParamSpec {
                target: Some("module".to_string()),
                long: "module".to_string(),
                short: None,
                about: Some("List out all versions for the specified module".to_string()),
                choices: None,
                default: None,
                kind: ValueKind::String,
                multiple: false,
                param_where: ParamWhere::Query,
                required: false,
                allow_hyphen_values: None,
            }],
            flags: Vec::new(),
            bindings: BTreeMap::from([
                ("verb".to_string(), "list".to_string()),
                ("module".to_string(), module.to_string()),
            ]),
            method: None,
            route: None,
            result: None,
        }],
        openapi_spec: None,
    }
}
