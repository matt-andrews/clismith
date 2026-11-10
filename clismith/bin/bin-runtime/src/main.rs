use std::process::ExitCode;

use cr_runtime::exec;

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    //
    exec().await
}
