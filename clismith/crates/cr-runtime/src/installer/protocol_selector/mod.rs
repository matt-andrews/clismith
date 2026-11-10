use enum_dispatch::enum_dispatch;

use crate::installer::protocol_selector::file_protocol::FileProtocol;

mod file_protocol;

#[enum_dispatch]
pub trait ProtocolShape {
    async fn download_module(&self, module_name: &str) -> anyhow::Result<()>;
    async fn download_app(&self) -> anyhow::Result<()>;
}

#[enum_dispatch(ProtocolShape)]
pub enum AnyProtocol {
    FileProtocol,
}

pub fn protocol_for(loc: &str) -> AnyProtocol {
    FileProtocol {
        loc: loc.trim_start_matches("lfs://").to_string(),
    }
    .into()
}
