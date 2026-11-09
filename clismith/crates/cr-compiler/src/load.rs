use std::{collections::BTreeMap, fs, path::Path};

use cr_core::{
    helpers::{deserialize_json_from_path, deserialize_yaml_from_path, env_or},
    v1spec::{
        authored::{core::CoreSpec, module::ModuleSpec},
        index::Index,
    },
};
use oas3::Spec;

use crate::models::LoadResponse;

pub fn load(root_dir: &Path, target_dir: &Path) -> anyhow::Result<LoadResponse> {
    let project: CoreSpec = load_project(root_dir)?;
    let modules: Vec<ModuleSpec> =
        load_all_modules(root_dir, &env_or("MODULE_SUFFIX", ".module.yml"))?;
    let oapi_specs: BTreeMap<String, Spec> = load_all_openapi_docs(root_dir, &modules);
    let index = load_index(target_dir, &project.name.clone());

    Ok(LoadResponse {
        project,
        modules,
        oapi_specs,
        index,
    })
}

fn load_project(root_dir: &Path) -> anyhow::Result<CoreSpec> {
    let project: CoreSpec =
        deserialize_yaml_from_path(root_dir, &env_or("PROJECT_FILE", "project.yml").to_string())?;
    Ok(project)
}

fn load_all_modules(root_dir: &Path, module_suffix: &str) -> anyhow::Result<Vec<ModuleSpec>> {
    //TODO: refactor with walkdir for recursive search
    let mut result: Vec<ModuleSpec> = Vec::new();

    for entry in fs::read_dir(root_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let filename = path.file_name().unwrap_or_default().to_string_lossy();

            if filename.ends_with(module_suffix) {
                result.push(load_module(
                    root_dir,
                    &path.file_name().unwrap().to_string_lossy(),
                )?);
            }
        }
    }

    Ok(result)
}

fn load_module(root_dir: &Path, file: &str) -> anyhow::Result<ModuleSpec> {
    let module: ModuleSpec = deserialize_yaml_from_path(root_dir, file)?;
    Ok(module)
}

fn load_all_openapi_docs(root_dir: &Path, modules: &[ModuleSpec]) -> BTreeMap<String, Spec> {
    let mut result: BTreeMap<String, Spec> = BTreeMap::new();
    for module in modules {
        if let Some(spec) = load_openapi_doc(root_dir, module) {
            result.insert(spec.0, spec.1);
        }
    }
    result
}

fn load_openapi_doc(root_dir: &Path, module: &ModuleSpec) -> Option<(String, Spec)> {
    if let Some(spec_path) = &module.openapi_spec {
        let yaml = std::fs::read_to_string(root_dir.join(spec_path)).unwrap();

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

fn load_index(target_dir: &Path, project_name: &str) -> Index {
    match does_index_exist(target_dir) {
        true => match deserialize_json_from_path(target_dir, "index.json") {
            Ok(v) => v,
            _ => Index::new(project_name, ""),
        },
        false => Index::new(project_name, ""),
    }
}

fn does_index_exist(target_dir: &Path) -> bool {
    let mut target = target_dir.to_path_buf();
    target.push("index.json");
    target.try_exists().unwrap_or_default()
}
