use std::{collections::BTreeMap, process::ExitCode};

use cr_core::v1spec::{core::CoreSpec, index::Index, param_where::ParamWhere};

use crate::{
    outputs::{AnyOutput, OutputShape, output_for},
    req_dispatch::{
        http_dispatch,
        request::{HttpMethod, HttpRequestMessage},
    },
    runtime::{
        invocation::Invocation,
        module_selector::{Selection, select, skeleton},
        preprocessor::result_preprocess,
        sys::{module_installer::check_for_required_modules, sys_dispatcher},
        tree::CommandTree,
        utils::{get_core, get_manifest},
    },
};

mod installer;
mod outputs;
mod req_dispatch;
mod runtime;

pub async fn exec() -> anyhow::Result<ExitCode> {
    //
    if let Err(err) = check_for_required_modules().await {
        eprintln!("failed to install required modules: {}", err);
        return Ok(ExitCode::FAILURE);
    }

    let manifest: Index = get_manifest()?;
    let core: CoreSpec = get_core(&manifest)?;

    match select(skeleton(&core, &manifest)?).unwrap_or_else(|e| e.exit()) {
        Selection::Module(name) => {
            let tree = CommandTree::from_yaml(&name, &manifest, &core);

            if let Err(e) = tree {
                eprintln!("error: invalid CLI config: {e}");
                return Ok(ExitCode::FAILURE);
            }

            let inv = tree.unwrap().parse().unwrap_or_else(|e| e.exit());

            Ok(dispatch(&inv).await)
        }
    }
}

async fn dispatch(inv: &Invocation) -> ExitCode {
    if inv.is_verbose() {
        println!(
            "[parsed] path={:?}\n[parsed] bindings={:?}\n[parsed] params={:?}",
            inv.path, inv.bindings, inv.params
        );
    }

    if let Some(sys_dispatch) = sys_dispatcher::dispatch(inv).await {
        let output_printer: AnyOutput = output_for(inv.get_output().unwrap_or_default());
        let mut output = sys_dispatch;
        result_preprocess(&mut output, inv);
        output_printer.print_output(output);
        return ExitCode::SUCCESS;
    }

    match http_dispatch(HttpRequestMessage {
        base_uri: inv.base_uri.clone(),
        route: route_resolver(inv),
        body: inv.request_body(),
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
            let mut output = result.content;
            result_preprocess(&mut output, inv);
            output_printer.print_output(output);
            ExitCode::SUCCESS
        }
        _ => ExitCode::FAILURE,
    }
}

fn route_resolver(inv: &Invocation) -> String {
    //
    let mut result = inv.triggered_command.clone().route.unwrap_or_default();
    let args_and_flags = &inv.args_and_flags();

    for route_param in args_and_flags
        .iter()
        .filter(|f| f.param_where == ParamWhere::Route)
    {
        let pattern = &format!(
            "{{{}}}",
            route_param
                .target
                .clone()
                .unwrap_or(route_param.long.clone())
        );
        result = result.replace(pattern, inv.str(&route_param.long).unwrap_or_default());
    }

    let mut qry: String = String::new();

    for qry_param in args_and_flags
        .iter()
        .filter(|f| f.param_where == ParamWhere::Query)
    {
        if let Some(arg) = inv.str(&qry_param.target.clone().unwrap_or(qry_param.long.clone())) {
            qry = format!(
                "{}&{}={}",
                qry,
                qry_param.target.clone().unwrap_or(qry_param.long.clone()),
                arg
            );
        }
    }

    if !qry.is_empty() {
        result = format!("{}?{}", result, qry.trim_start_matches('&'));
    }

    result
}
