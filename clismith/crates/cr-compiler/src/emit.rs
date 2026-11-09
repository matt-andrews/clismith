use std::path::Path;

use cr_core::{
    helpers::{deserialize_json_from_path, serialize_to_json},
    v1spec::{
        authored::{core::CoreSpec, module::ModuleSpec},
        compiled::{core::CoreCompiled, module::ModuleCompiled},
        diff::{Bump, Change, Diff},
        index::{Index, IndexModule},
    },
};
use semver::Version;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::models::MergeResponse;

pub fn emit(context: &mut MergeResponse, target_dir: &Path) -> anyhow::Result<()> {
    emit_project(&context.project, target_dir, &mut context.index)?;
    emit_modules(&context.modules, target_dir, &mut context.index)?;

    Ok(())
}

pub fn emit_index(context: &MergeResponse, target_dir: &Path) -> anyhow::Result<()> {
    let mut target = target_dir.to_path_buf();
    target.push("index.json");
    serialize_to_json(&context.index, &target)?;
    Ok(())
}

fn emit_project(project: &CoreSpec, target_dir: &Path, index: &mut Index) -> anyhow::Result<()> {
    let mut target = target_dir.to_path_buf();
    let module_name = "core";
    let index_module = index.latest_module(module_name);
    let compiled_project: CoreCompiled = project.to_compiled();

    if let Some(index_module) = index_module {
        //
        let mut version = Version::parse(&index_module.version)?;
        let sha = calculate_sha256_hash(&compiled_project)?;
        if index_module.sha256.eq(&sha) {
            return Ok(());
        }
        let old_proj: CoreCompiled = deserialize_json_from_path(target_dir, &index_module.target)?;
        let mut bump_list: Vec<Change> = Vec::new();
        old_proj.diff(&compiled_project, "core", &mut bump_list);
        let bump = bump_list.iter().map(|c| c.bump).max().unwrap_or(Bump::None);
        if bump == Bump::Major {
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
        } else if bump == Bump::Minor {
            version.minor += 1;
            version.patch = 0;
        } else if bump == Bump::Patch {
            version.patch += 1;
        }
        let module_file_name = format!("{}.v{}.json", module_name, version);
        index.core = module_file_name.clone();
        let index_module = IndexModule {
            name: module_name.to_string(),
            version: version.to_string(),
            target: module_file_name.clone(),
            sha256: sha,
        };
        index.modules.push(index_module);

        target.push(module_file_name);
        serialize_to_json(&compiled_project, &target)?;
    } else {
        let version = "1.0.0";
        let module_file_name = format!("{}.v{}.json", module_name, version);
        index.core = module_file_name.clone();
        let index_module = IndexModule {
            name: module_name.to_string(),
            version: version.to_string(),
            target: module_file_name.clone(),
            sha256: calculate_sha256_hash(&compiled_project)?,
        };
        index.modules.push(index_module);

        target.push(module_file_name);
        serialize_to_json(&compiled_project, &target)?;
    }

    Ok(())
}

fn emit_modules(
    modules: &[ModuleSpec],
    target_dir: &Path,
    index: &mut Index,
) -> anyhow::Result<()> {
    for module in modules {
        emit_module(module, target_dir, index)?
    }
    Ok(())
}

pub fn emit_module(
    module: &ModuleSpec,
    target_dir: &Path,
    index: &mut Index,
) -> anyhow::Result<()> {
    let mut target = target_dir.to_path_buf();
    let module_name = &module.name;
    let index_module = index.latest_module(module_name);
    let compiled: ModuleCompiled = module.to_compiled();

    if let Some(index_module) = index_module {
        let mut version = Version::parse(&index_module.version)?;
        let sha = calculate_sha256_hash(&compiled)?;
        if index_module.sha256.eq(&sha) {
            return Ok(());
        }
        let old_module: ModuleCompiled =
            deserialize_json_from_path(target_dir, &index_module.target)?;
        let mut bump_list: Vec<Change> = Vec::new();
        old_module.diff(&compiled, module_name, &mut bump_list);
        let bump = bump_list.iter().map(|c| c.bump).max().unwrap_or(Bump::None);
        if bump == Bump::Major {
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
        } else if bump == Bump::Minor {
            version.minor += 1;
            version.patch = 0;
        } else if bump == Bump::Patch {
            version.patch += 1;
        }
        let module_file_name = format!("{}.v{}.json", module_name, version);
        let index_module = IndexModule {
            name: module_name.to_string(),
            version: version.to_string(),
            target: module_file_name.clone(),
            sha256: calculate_sha256_hash(&compiled)?,
        };
        index.modules.push(index_module);
        target.push(module_file_name.clone());
        serialize_to_json(&module.to_compiled(), &target)?;
    } else {
        let version = "1.0.0";
        let module_file_name = format!("{}.v{}.json", module_name, version);
        let index_module = IndexModule {
            name: module_name.to_string(),
            version: version.to_string(),
            target: module_file_name.clone(),
            sha256: calculate_sha256_hash(&compiled)?,
        };
        index.modules.push(index_module);
        target.push(module_file_name.clone());
        serialize_to_json(&module.to_compiled(), &target)?;
    }

    Ok(())
}

fn calculate_sha256_hash<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
    let bytes = serde_json::to_vec(data)?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}
