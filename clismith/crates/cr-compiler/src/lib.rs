use std::fs;
use std::path::Path;

use crate::core_modules::emit_core_modules;
use crate::emit::{emit, emit_index};
use crate::load::load;
use crate::merge::merge;
use crate::models::{LoadResponse, MergeResponse};
use crate::validate::validate;

mod core_modules;
mod emit;
mod load;
mod merge;
mod models;
mod validate;

pub fn compile(root_dir: &Path, target_dir: &Path) -> anyhow::Result<()> {
    //
    fs::create_dir_all(target_dir)?;
    let load: LoadResponse = load(root_dir, target_dir)?;
    let mut merge: MergeResponse = merge(&load)?;
    validate(&merge)?;
    emit(&mut merge, target_dir)?;
    emit_core_modules(&mut merge, target_dir)?;

    emit_index(&merge, target_dir)?;
    Ok(())
}
