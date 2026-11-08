use std::path::Path;

use crate::emit::emit;
use crate::load::load;
use crate::merge::merge;
use crate::models::{LoadResponse, MergeResponse};
use crate::validate::validate;

mod emit;
mod load;
mod merge;
mod models;
mod validate;

pub fn compile(root_dir: &Path, target_dir: &Path) -> anyhow::Result<()> {
    //
    let load: LoadResponse = load(root_dir)?;
    let merge: MergeResponse = merge(&load)?;
    validate(&merge)?;
    emit(&merge, target_dir)?;

    Ok(())
}
