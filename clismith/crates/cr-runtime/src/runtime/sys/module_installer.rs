use cr_core::v1spec::{core::CoreSpec, index::Index};

use crate::{
    installer::module_install,
    runtime::utils::{get_core, get_manifest},
};

pub async fn check_for_required_modules() -> anyhow::Result<()> {
    //
    let manifest: Index = get_manifest()?;
    let core: CoreSpec = get_core(&manifest)?;
    for req in core.required_modules.unwrap_or_default() {
        if manifest.latest_module(&req.module).is_none() {
            install_module(&req.module).await?;
        }
    }

    Ok(())
}

pub async fn install_module(module_name: &str) -> anyhow::Result<()> {
    let manifest: Index = get_manifest()?;
    let core: CoreSpec = get_core(&manifest)?;
    let registry = core
        .registries
        .unwrap_or_default()
        .first()
        .unwrap_or_else(|| panic!("cannot find a registry to download modules from"))
        .clone();

    module_install(registry.index.as_str(), module_name).await?;

    Ok(())
}
