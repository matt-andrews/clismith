use std::{collections::BTreeMap, fs};

use cr_core::{
    dir_helpers::{DirType, get_dir, get_dir_with},
    helpers::env_or,
    io_helpers::deserialize_from_file,
    v1spec::{core::CoreSpec, index::Index, module::ModuleSpec},
};
use oas3::Spec;

use crate::models::LoadResponse;

pub fn load() -> anyhow::Result<LoadResponse> {
    let project: CoreSpec = load_project()?;
    let modules: Vec<ModuleSpec> = load_all_modules(&env_or("MODULE_SUFFIX", ".module.yml"))?;
    let oapi_specs: BTreeMap<String, Spec> = load_all_openapi_docs(&modules);
    let index = load_index(&project.name.clone());

    Ok(LoadResponse {
        project,
        modules,
        oapi_specs,
        index,
    })
}

fn load_project() -> anyhow::Result<CoreSpec> {
    Ok(deserialize_from_file(&get_dir_with(
        DirType::Project,
        &env_or("PROJECT_FILE", "project.yml"),
    ))?)
}

fn load_all_modules(module_suffix: &str) -> anyhow::Result<Vec<ModuleSpec>> {
    //TODO: refactor with walkdir for recursive search
    let mut result: Vec<ModuleSpec> = Vec::new();

    for entry in fs::read_dir(get_dir(DirType::Project))? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let filename = path.file_name().unwrap_or_default().to_string_lossy();

            if filename.ends_with(module_suffix) {
                result.push(load_module(&path.file_name().unwrap().to_string_lossy())?);
            }
        }
    }

    Ok(result)
}

fn load_module(file: &str) -> anyhow::Result<ModuleSpec> {
    Ok(deserialize_from_file(&get_dir_with(
        DirType::Project,
        file,
    ))?)
}

fn load_all_openapi_docs(modules: &[ModuleSpec]) -> BTreeMap<String, Spec> {
    let mut result: BTreeMap<String, Spec> = BTreeMap::new();
    for module in modules {
        if let Some(spec) = load_openapi_doc(module) {
            result.insert(spec.0, spec.1);
        }
    }
    result
}

fn load_openapi_doc(module: &ModuleSpec) -> Option<(String, Spec)> {
    if let Some(spec_path) = &module.openapi_spec {
        let yaml = std::fs::read_to_string(get_dir(DirType::Project).join(spec_path)).unwrap();

        return match oas3::from_yaml(yaml) {
            Ok(spec) => Some((module.name.to_string(), spec)),
            Err(err) => {
                println!("error: {}", err);
                None
            }
        };
    }

    None
}

fn load_index(project_name: &str) -> Index {
    match does_index_exist() {
        true => match deserialize_from_file(&get_dir_with(DirType::Target, "index.json")) {
            Ok(v) => v,
            _ => Index::new(project_name, ""),
        },
        false => Index::new(project_name, ""),
    }
}

fn does_index_exist() -> bool {
    let target = get_dir_with(DirType::Target, "index.json");
    target.try_exists().unwrap_or_default()
}
