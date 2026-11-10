use std::process::ExitCode;

use cr_exec::exec;

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    //
    exec().await
}
