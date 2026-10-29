use std::{collections::BTreeMap, path::PathBuf, process};

use cr_outputs::{AnyOutput, OutputShape, output_for};
use cr_req_dispatch::{
    http_dispatch,
    request::{HttpMethod, HttpRequestMessage},
};

use crate::{invocation::Invocation, tree::CommandTree};

mod errors;
mod invocation;
mod pattern;
mod tree;

#[tokio::main]
async fn main() -> process::ExitCode {
    let build_path = PathBuf::from("./");
    let tree = CommandTree::from_yaml(&build_path);

    if let Err(e) = tree {
        eprintln!("error: invalid CLI config: {e}");
        return get_exit(ExitCode::Failed);
    }

    let inv = tree.unwrap().parse().unwrap_or_else(|e| e.exit());

    get_exit(dispatch(&inv).await)
}

async fn dispatch(inv: &Invocation) -> ExitCode {
    if inv.is_verbose() {
        eprintln!(
            "[parsed] path={:?}\n[parsed] bindings={:?}\n[parsed] params={:?}",
            inv.path, inv.bindings, inv.params
        );
    }

    match http_dispatch(HttpRequestMessage {
        base_uri: "http://localhost:5080".to_string(),
        route: inv.triggered_command.clone().unwrap().route.unwrap(),
        body: None,
        headers: BTreeMap::new(),
        method: HttpMethod::Get,
        verbose: inv.is_verbose(),
        output_type_headers: inv.is_head(),
    })
    .await
    {
        Ok(result) => {
            let output_printer: AnyOutput = output_for(inv.get_output().unwrap_or_default());
            output_printer.print_output(result.content);
            ExitCode::Success
        }
        _ => ExitCode::Failed,
    }
}

fn get_exit(exit: ExitCode) -> process::ExitCode {
    process::ExitCode::from(exit as u8)
}

#[repr(i32)]
enum ExitCode {
    Success = 0,
    Failed = 1,
}
