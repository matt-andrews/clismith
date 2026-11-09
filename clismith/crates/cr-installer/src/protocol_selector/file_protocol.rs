use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use cr_core::{
    helpers::{deserialize_json_from_path, serialize_to_json},
    v1spec::index::{Index, IndexModule},
};

use crate::protocol_selector::ProtocolShape;

pub struct FileProtocol {
    pub loc: String,
}

impl ProtocolShape for FileProtocol {
    async fn download_module(&self, module_name: &str, download_dir: &Path) -> anyhow::Result<()> {
        let src = std::fs::read_to_string(self.loc.clone())?;
        let index: Index = serde_json::from_str(&src)?;
        let mut dir = PathBuf::from(self.loc.clone())
            .parent()
            .unwrap_or(&PathBuf::from("/"))
            .to_path_buf();
        let module = index
            .latest_module(module_name)
            .unwrap_or_else(|| panic!("cannot find specified module to install"));
        dir.push(module.target.clone());

        let old = std::fs::read_to_string(dir)?;

        let mut download_dir = download_dir.to_path_buf();
        create_or_update_manifest(&download_dir, &index.name, module)?;
        download_dir.push(module.target.clone());

        fs::write(download_dir, old)?;

        Ok(())
    }

    async fn download_app(&self, download_dir: &Path) -> anyhow::Result<()> {
        let src = std::fs::read_to_string(self.loc.clone())?;
        let index: Index = serde_json::from_str(&src)?;
        let module = index
            .latest_module("core")
            .unwrap_or_else(|| panic!("cannot find specified module to install"));

        let mut dir = PathBuf::from(self.loc.clone())
            .parent()
            .unwrap_or(&PathBuf::from("/"))
            .to_path_buf();
        dir.push(index.core.clone());

        let old = std::fs::read_to_string(dir)?;

        let mut download_dir = download_dir.to_path_buf();
        download_dir.push(index.name.clone()); //app needs to add app name to dir
        fs::create_dir_all(download_dir.clone())?;
        create_or_update_manifest(&download_dir, &index.name, module)?;

        download_dir.push(index.core.clone());
        fs::write(download_dir.clone(), old)?;

        Ok(())
    }
}

fn create_or_update_manifest(
    download_dir: &Path,
    project_name: &str,
    index_module: &IndexModule,
) -> anyhow::Result<()> {
    let mut manifest: Index = match does_manifest_exist(download_dir) {
        true => match deserialize_json_from_path(download_dir, "manifest.json") {
            Ok(v) => v,
            _ => Index::new(project_name, ""),
        },
        false => Index::new(project_name, ""),
    };
    if index_module.name == "core" {
        manifest.core = index_module.target.clone();
    }
    manifest.modules.push(index_module.clone());

    let mut target = download_dir.to_path_buf();
    target.push("manifest.json");
    serialize_to_json(&manifest, &target)?;
    Ok(())
}

fn does_manifest_exist(download_dir: &Path) -> bool {
    let mut target = download_dir.to_path_buf();
    target.push("manifest.json");
    target.try_exists().unwrap_or_default()
}
