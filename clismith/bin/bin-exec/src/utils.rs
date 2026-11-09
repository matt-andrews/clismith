use std::path::Path;

use cr_core::helpers::{deserialize_json_from_path, env_or};

use crate::manifest::Manifest;

pub fn get_manifest(dir: &Path) -> anyhow::Result<Manifest> {
    Ok(deserialize_json_from_path(
        dir,
        &env_or("MANIFEST_NAME", "index.json"),
    )?)
}
