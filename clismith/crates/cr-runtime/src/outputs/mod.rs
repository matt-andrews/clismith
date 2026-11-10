use enum_dispatch::enum_dispatch;
use serde_json::Value;

use crate::outputs::{json_output::JsonOutput, none_output::NoneOutput, table_output::TableOutput};

mod json_output;
mod none_output;
mod table_output;

#[enum_dispatch]
pub trait OutputShape {
    fn print_output(&self, content: Value);
}

#[allow(clippy::enum_variant_names)]
#[enum_dispatch(OutputShape)]
pub enum AnyOutput {
    JsonOutput,
    TableOutput,
    NoneOutput,
}

pub fn output_for(kind: &str) -> AnyOutput {
    match kind {
        "table" => TableOutput {}.into(),
        "none" => NoneOutput {}.into(),
        _ => JsonOutput {}.into(),
    }
}
