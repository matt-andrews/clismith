use std::path::Path;

use cr_core::{
    helpers::{deserialize_yaml_from_path, env_or, serialize_to_json},
    v1spec::authored::core::CoreSpec,
};

pub fn compile_project(src: &Path, target: &Path) -> anyhow::Result<CoreSpec> {
    let project: CoreSpec =
        deserialize_yaml_from_path(src, &env_or("PROJECT_FILE", "project.yml").to_string())?;
    let mut target = target.to_path_buf();
    target.push(env_or("CORE_OUTPUT_FILE", "core.json"));
    serialize_to_json(&project.to_compiled(), &target)?;
    Ok(project)
}
