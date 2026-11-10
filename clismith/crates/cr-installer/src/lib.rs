use crate::protocol_selector::ProtocolShape;
use crate::protocol_selector::protocol_for;

mod protocol_selector;
pub async fn module_install(index: &str, module_name: &str) -> anyhow::Result<()> {
    //
    let downloader = protocol_for(index);
    downloader.download_module(module_name).await?;

    Ok(())
}

pub async fn app_install(index: &str) -> anyhow::Result<()> {
    //
    let downloader = protocol_for(index);
    downloader.download_app().await?;

    Ok(())
}
