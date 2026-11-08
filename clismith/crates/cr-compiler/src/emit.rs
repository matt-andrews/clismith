use std::path::Path;

use cr_core::{
    helpers::{env_or, serialize_to_json},
    v1spec::authored::{core::CoreSpec, module::ModuleSpec},
};

use crate::models::MergeResponse;

pub fn emit(context: &MergeResponse, target_dir: &Path) -> anyhow::Result<()> {
    emit_project(&context.project, target_dir)?;
    emit_modules(&context.modules, target_dir)?;

    Ok(())
}

fn emit_project(project: &CoreSpec, target_dir: &Path) -> anyhow::Result<()> {
    let mut target = target_dir.to_path_buf();
    target.push(env_or("CORE_OUTPUT_FILE", "core.json"));
    serialize_to_json(&project.to_compiled(), &target)?;
    Ok(())
}

fn emit_modules(modules: &[ModuleSpec], target_dir: &Path) -> anyhow::Result<()> {
    for module in modules {
        emit_module(module, target_dir)?
    }
    Ok(())
}

fn emit_module(module: &ModuleSpec, target_dir: &Path) -> anyhow::Result<()> {
    let mut target = target_dir.to_path_buf();
    //TODO - need version in filename for compat target?
    let file_name = format!("{}.json", module.name);
    target.push(file_name.clone());
    serialize_to_json(&module.to_compiled(), &target)?;
    Ok(())
}
