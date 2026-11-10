use cr_core::{
    dir_helpers::{DirType, get_dir_with},
    helpers::env_or,
    io_helpers::deserialize_from_file,
    v1spec::{
        core::CoreSpec,
        index::{Index, IndexModule},
        module::ModuleSpec,
    },
};

pub fn get_manifest() -> anyhow::Result<Index> {
    Ok(deserialize_from_file(&get_dir_with(
        DirType::Exec,
        &env_or("MANIFEST_NAME", "manifest.json"),
    ))?)
}

pub fn get_core(manifest: &Index) -> anyhow::Result<CoreSpec> {
    Ok(deserialize_from_file(&get_dir_with(
        DirType::Exec,
        &manifest.core,
    ))?)
}

pub fn get_module(module: &IndexModule) -> anyhow::Result<ModuleSpec> {
    Ok(deserialize_from_file(&get_dir_with(
        DirType::Exec,
        &module.target,
    ))?)
}
