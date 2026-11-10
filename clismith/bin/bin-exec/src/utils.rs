use cr_core::{
    dir_helpers::{DirType, get_dir_with},
    helpers::env_or,
    io_helpers::deserialize_from_file,
    v1spec::compiled::{core::CoreCompiled, module::ModuleCompiled},
};

use crate::manifest::{Manifest, ManifestModule};

pub fn get_manifest() -> anyhow::Result<Manifest> {
    Ok(deserialize_from_file(&get_dir_with(
        DirType::Exec,
        &env_or("MANIFEST_NAME", "manifest.json"),
    ))?)
}

pub fn get_core(manifest: &Manifest) -> anyhow::Result<CoreCompiled> {
    Ok(deserialize_from_file(&get_dir_with(
        DirType::Exec,
        &manifest.core,
    ))?)
}

pub fn get_module(module: &ManifestModule) -> anyhow::Result<ModuleCompiled> {
    Ok(deserialize_from_file(&get_dir_with(
        DirType::Exec,
        &module.target,
    ))?)
}
