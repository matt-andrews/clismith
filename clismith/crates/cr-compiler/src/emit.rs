use cr_core::{
    dir_helpers::{DirType, get_dir_with},
    io_helpers::{deserialize_from_file, serialize_to_file},
    v1spec::{
        core::CoreSpec,
        diff::{Bump, Change, Diff},
        index::{Index, IndexModule},
        module::ModuleSpec,
    },
};
use semver::Version;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::models::MergeResponse;

pub fn emit(context: &mut MergeResponse) -> anyhow::Result<()> {
    emit_project(&context.project, &mut context.index)?;
    emit_modules(&context.modules, &mut context.index)?;

    Ok(())
}

pub fn emit_index(context: &MergeResponse) -> anyhow::Result<()> {
    Ok(serialize_to_file(
        &context.index,
        &get_dir_with(DirType::Target, "index.json"),
    )?)
}

pub fn emit_module(module: &ModuleSpec, index: &mut Index) -> anyhow::Result<()> {
    let module_name = &module.name;
    let index_imut = index.clone();
    let index_module = index_imut.latest_module(module_name);
    let compiled: ModuleSpec = module.clone();
    emit_obj(index, module, &compiled, index_module, module_name, |v| {
        deserialize_from_file(&get_dir_with(DirType::Target, v))
            .expect("expected to find an older module to compare")
    })?;
    Ok(())
}

fn emit_project(module: &CoreSpec, index: &mut Index) -> anyhow::Result<()> {
    let module_name = "core";
    let index_imut = index.clone();
    let index_module = index_imut.latest_module(module_name);
    let compiled: CoreSpec = module.clone();
    let module_name = emit_obj(index, module, &compiled, index_module, module_name, |v| {
        deserialize_from_file(&get_dir_with(DirType::Target, v))
            .expect("expected to find an older module to compare")
    })?;
    index.core = module_name;
    Ok(())
}

fn emit_modules(modules: &[ModuleSpec], index: &mut Index) -> anyhow::Result<()> {
    for module in modules {
        emit_module(module, index)?
    }
    Ok(())
}

///emit either a ModuleCompiled or CoreCompiled to the target dir. Handles sha check and version bump
fn emit_obj<T>(
    index: &mut Index,
    module: &T,
    compiled: &T,
    index_module: Option<&IndexModule>,
    module_name: &str,
    old_module: fn(&str) -> T,
) -> anyhow::Result<String>
where
    T: Serialize,
    T: Diff,
{
    if let Some(index_module) = index_module {
        let mut version = Version::parse(&index_module.version)?;
        let sha = calculate_sha256_hash(&compiled)?;
        if index_module.sha256.eq(&sha) {
            return Ok(index_module.target.clone());
        }
        let mut bump_list: Vec<Change> = Vec::new();
        let old_module = old_module(&index_module.target);
        old_module.diff(compiled, module_name, &mut bump_list);
        if !bump_version(&mut version, bump_list) {
            return Ok(index_module.target.clone());
        }
        serialize_module(index, module, module_name, &version.to_string(), sha)
    } else {
        let version = "1.0.0";
        serialize_module(
            index,
            module,
            module_name,
            version,
            calculate_sha256_hash(&compiled)?,
        )
    }
}

///handles the serilization of the module and adds the module to the index
fn serialize_module<T>(
    index: &mut Index,
    module: &T,
    module_name: &str,
    version: &str,
    sha: String,
) -> anyhow::Result<String>
where
    T: Serialize,
    T: Diff,
{
    let module_file_name = format!("{}.v{}.json", module_name, version);
    let index_module = IndexModule {
        name: module_name.to_string(),
        version: version.to_string(),
        target: module_file_name.clone(),
        sha256: sha,
    };
    index.modules.push(index_module);
    serialize_to_file(
        &module,
        &get_dir_with(DirType::Target, &module_file_name.clone()),
    )?;
    Ok(module_file_name)
}

///bumps the version based on the given bump values; returns true if version bumped
fn bump_version(version: &mut Version, bump_list: Vec<Change>) -> bool {
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
    } else {
        return false;
    }
    true
}

fn calculate_sha256_hash<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
    let bytes = serde_json::to_vec(data)?;
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let result = hasher.finalize();
    Ok(hex::encode(result))
}
