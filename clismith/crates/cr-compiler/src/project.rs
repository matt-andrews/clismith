use std::path::Path;

use cr_core::{
    helpers::{deserialize_yaml_from_path, serialize_to_json},
    v1spec::compiled::core::CoreCompiled,
};

pub fn compile_project(src: &Path, target: &Path) -> anyhow::Result<CoreCompiled> {
    let project: CoreCompiled = deserialize_yaml_from_path(src, "project.yml")?;
    let mut target = target.to_path_buf();
    target.push("core.json");
    serialize_to_json(&project, &target)?;
    Ok(project)
}
