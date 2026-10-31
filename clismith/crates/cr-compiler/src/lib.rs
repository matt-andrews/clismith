use std::path::Path;

use crate::project::compile_project;

mod project;

pub fn compile(src: &Path, target: &Path) -> anyhow::Result<()> {
    //
    compile_project(src, target)?;
    Ok(())
}
