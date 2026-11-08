use std::{collections::BTreeMap, path::PathBuf, process};

use cr_core::{
    helpers::env_or,
    v1spec::{compiled::param::ParamCompiled, param_where::ParamWhere},
};
use cr_outputs::{AnyOutput, OutputShape, output_for};
use cr_req_dispatch::{
    http_dispatch,
    request::{HttpMethod, HttpRequestMessage},
};
use noyalib::cst::format;

use crate::{invocation::Invocation, tree::CommandTree};

mod errors;
mod invocation;
mod pattern;
mod tree;

#[tokio::main]
async fn main() -> process::ExitCode {
    let build_path = PathBuf::from(env_or(
        "PROJECT_LOCATION",
        "../examples/billing_api/openapi/target",
    ));
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
        base_uri: inv.base_uri.clone(),
        route: route_resolver(inv),
        body: None,               //TODO
        headers: BTreeMap::new(), //TODO
        method: match inv
            .triggered_command
            .clone()
            .method
            .unwrap_or("GET".to_string())
            .to_uppercase()
            .as_str()
        {
            "PUT" => HttpMethod::Put,
            "POST" => HttpMethod::Post,
            "PATCH" => HttpMethod::Patch,
            "DELETE" => HttpMethod::Delete,
            "OPTIONS" => HttpMethod::Options,
            _ => HttpMethod::Get,
        },
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

fn route_resolver(inv: &Invocation) -> String {
    //
    let mut result = inv.triggered_command.clone().route.unwrap_or_default();

    for route_param in inv
        .triggered_command
        .args
        .iter()
        .filter(|f| f.param_where == ParamWhere::Route)
    {
        let pattern = &format!("{{{}}}", route_param.target.clone().unwrap_or_default());
        result = result.replace(pattern, inv.str(&route_param.long).unwrap_or_default());
    }

    let mut qry: String = String::new();

    for qry_param in inv
        .triggered_command
        .args
        .iter()
        .filter(|f| f.param_where == ParamWhere::Query)
    {
        qry = format!(
            "{}&{}={}",
            qry,
            qry_param.long,
            inv.str(&qry_param.long).unwrap_or_default()
        );
    }

    if !qry.is_empty() {
        result = format!("{}?{}", result, qry.trim_start_matches('&'));
    }

    result
}

fn get_exit(exit: ExitCode) -> process::ExitCode {
    process::ExitCode::from(exit as u8)
}

#[repr(i32)]
enum ExitCode {
    Success = 0,
    Failed = 1,
}
