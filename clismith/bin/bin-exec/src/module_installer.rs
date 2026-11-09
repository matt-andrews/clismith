use std::path::Path;

use cr_core::{helpers::deserialize_json_from_path, v1spec::compiled::core::CoreCompiled};
use cr_installer::{app_install, module_install};

use crate::manifest::Manifest;

pub async fn check_for_app_installer(app_dir: &Path) -> Option<()> {
    let mut args = std::env::args_os();
    args.next();
    let is_installer = args
        .next()
        .unwrap_or_else(|| panic!("failed to identify"))
        .to_str()
        .unwrap()
        .to_string();

    if is_installer != "app-install" {
        return None;
    }

    let arg = args
        .next()
        .unwrap_or_else(|| panic!("no app installer was specified"))
        .to_str()
        .unwrap()
        .to_string();

    if let Err(failed) = app_install(app_dir, &arg).await {
        eprintln!("failed to install app: {}", failed);
    }

    Some(())
}

pub async fn check_for_required_modules(app_dir: &Path) -> anyhow::Result<()> {
    //
    let manifest: Manifest = deserialize_json_from_path(app_dir, "manifest.json")?;
    let core: CoreCompiled = deserialize_json_from_path(app_dir, &manifest.core)?;
    for req in core.required_modules.unwrap_or_default() {
        if manifest.latest_module(&req.module).is_none() {
            install_module(app_dir, &req.module).await?;
        }
    }

    Ok(())
}

pub async fn install_module(app_dir: &Path, module_name: &str) -> anyhow::Result<()> {
    let manifest: Manifest = deserialize_json_from_path(app_dir, "manifest.json")?;
    let core: CoreCompiled = deserialize_json_from_path(app_dir, &manifest.core)?;
    let registry = core
        .registries
        .unwrap_or_default()
        .first()
        .unwrap_or_else(|| panic!("cannot find a registry to download modules from"))
        .clone();

    module_install(app_dir, registry.index.as_str(), module_name).await?;

    Ok(())
}
