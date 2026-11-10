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

pub fn compile() -> anyhow::Result<()> {
    //
    let load: LoadResponse = load()?;
    let mut merge: MergeResponse = merge(&load)?;
    validate(&merge)?;
    emit(&mut merge)?;
    emit_core_modules(&mut merge)?;

    emit_index(&merge)?;
    Ok(())
}
