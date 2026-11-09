use std::collections::BTreeMap;

use cr_core::{
    helpers::deconstruct_case,
    v1spec::{
        authored::{
            core::CoreSpec,
            module::{CommandSpec, ModuleSpec},
            param::ParamSpec,
        },
        param_where::ParamWhere,
        value_kind::ValueKind,
    },
};
use oas3::{
    Spec,
    spec::{ObjectOrReference, Operation, Schema, SchemaType},
};

use crate::models::{LoadResponse, MergeResponse};

pub fn merge(context: &LoadResponse) -> anyhow::Result<MergeResponse> {
    //
    let mut merged_modules: Vec<ModuleSpec> = Vec::new();

    for module in &context.modules {
        let spec = context.oapi_specs.get(&module.name);
        merged_modules.push(merge_module(&context.project, module, spec));
    }

    Ok(MergeResponse {
        project: context.project.clone(),
        modules: merged_modules,
        index: context.index.clone(),
    })
}

fn merge_module(project: &CoreSpec, module: &ModuleSpec, spec: Option<&Spec>) -> ModuleSpec {
    //TODO: merge will need to account for overriding api specs from the module definition
    let mut commands: Vec<CommandSpec> = module.commands.clone();

    if let Some(spec) = spec {
        for (path, item) in &spec.paths.clone().unwrap_or_default() {
            //
            let grammar_parts = get_grammar_route_parts(project, path);
            for (method, op) in item.methods() {
                commands.push(CommandSpec {
                    about: op.description
                        .as_deref()
                        .filter(|s| !s.is_empty())
                        .or(op.summary.as_deref().filter(|s| !s.is_empty()))
                        .map(str::to_owned),
                    target: Some(path.to_string()),
                    route: Some(path.to_string()),
                    method: Some(method.to_string().to_uppercase()),
                    result: None,
                    bindings: BTreeMap::from([
                        ("module".to_string(), module.name.clone()),
                        (
                            "resource".to_string(),
                            grammar_parts.first().unwrap_or(&String::new()).clone(),
                        ),
                        (
                            "verb".to_string(),
                            match grammar_parts.get(1) {
                                Some(v) => v.clone(),
                                None => project
                                    .grammar
                                    .verbs
                                    .iter()
                                    .filter(|f| f.method == method.as_str())
                                    .find(|f| f.target == get_verb_target_from_op(op, spec))
                                    .map(|m| m.verb.to_string())
                                    .unwrap_or_else(|| panic!("cannot find a verb for the given method and target {} - {}", method, get_verb_target_from_op(op, spec))),
                            },
                        ),
                    ]),
                    flags: Vec::new(),
                    args: op
                        .clone()
                        .parameters
                        .into_iter()
                        .filter_map(|m| match m.resolve(spec) {
                            Ok(v) => Some(ParamSpec {
                                about: v.description,
                                long: normalize_name(project, &v.name),
                                target: Some(v.name),
                                short: None,
                                kind: ValueKind::String,
                                default: None,
                                multiple: false,
                                required: v.required.unwrap_or_default(),
                                choices: None,
                                param_where: match v.location {
                                    oas3::spec::ParameterIn::Path => ParamWhere::Route,
                                    oas3::spec::ParameterIn::Query => ParamWhere::Query,
                                    oas3::spec::ParameterIn::Header => ParamWhere::Header,
                                    oas3::spec::ParameterIn::Cookie => ParamWhere::Cookie,
                                },
                                allow_hyphen_values: None
                            }),
                            Err(_) => None,
                        })
                        .collect(),
                });
            }
        }
    }

    ModuleSpec {
        name: module.name.clone(),
        about: Some(module.about.clone().unwrap_or(match spec {
            Some(v) => v.info.description.clone().unwrap_or_default(),
            None => String::new(),
        })),
        openapi_spec: module.openapi_spec.clone(),
        commands,
        base_uri: module.base_uri.clone(),
    }
}

fn get_grammar_route_parts(project: &CoreSpec, path: &str) -> Vec<String> {
    //
    path.split('/')
        .filter(|f| !f.contains('{') && !f.contains('}'))
        .filter(|f| !f.trim().is_empty())
        .map(|m| normalize_name(project, m))
        .collect()
    //
}

fn get_verb_target_from_op(op: &Operation, spec: &Spec) -> String {
    let is_array = (|| {
        let response = op.responses.as_ref()?.get("200")?.resolve(spec).ok()?;
        let schema = response.content.get("application/json")?.schema.as_ref()?;
        let Schema::Object(obj) = schema.resolve(spec).ok()? else {
            return None;
        };
        let ObjectOrReference::Object(obj) = *obj else {
            return None;
        };
        Some(obj.schema_type?.contains(SchemaType::Array))
    })()
    .unwrap_or(false);

    if is_array { "collection" } else { "item" }.to_string()
}

fn normalize_name(_: &CoreSpec, current_name: &str) -> String {
    // only kebab supported atm so no match needed
    let deconstructed = deconstruct_case(current_name);
    deconstructed.1.join("-")
}
