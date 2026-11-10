use std::{
    fs::{self},
    path::PathBuf,
};

use cr_core::{
    dir_helpers::{DirType, get_dir, get_dir_with},
    io_helpers::{deserialize_from_file, serialize_to_file},
    v1spec::index::{Index, IndexModule},
};

use crate::installer::protocol_selector::ProtocolShape;

pub struct FileProtocol {
    pub loc: String,
}

impl ProtocolShape for FileProtocol {
    async fn download_module(&self, module_name: &str) -> anyhow::Result<()> {
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

        let mut download_dir = get_dir(DirType::Exec);
        create_or_update_manifest(&index.name, module)?;
        download_dir.push(module.target.clone());

        fs::write(download_dir, old)?;

        Ok(())
    }

    async fn download_app(&self) -> anyhow::Result<()> {
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

        let mut download_dir = get_dir(DirType::Exec);
        fs::create_dir_all(download_dir.clone())?;
        create_or_update_manifest(&index.name, module)?;

        download_dir.push(index.core.clone());
        fs::write(download_dir.clone(), old)?;

        Ok(())
    }
}

fn create_or_update_manifest(project_name: &str, index_module: &IndexModule) -> anyhow::Result<()> {
    let mut manifest: Index = match does_manifest_exist() {
        true => match deserialize_from_file(&get_dir_with(DirType::Exec, "manifest.json")) {
            Ok(v) => v,
            _ => Index::new(project_name, ""),
        },
        false => Index::new(project_name, ""),
    };
    if index_module.name == "core" {
        manifest.core = index_module.target.clone();
    }
    manifest.modules.push(index_module.clone());

    serialize_to_file(&manifest, &get_dir_with(DirType::Exec, "manifest.json"))?;
    Ok(())
}

fn does_manifest_exist() -> bool {
    let target = get_dir_with(DirType::Exec, "manifest.json");
    target.try_exists().unwrap_or_default()
}
