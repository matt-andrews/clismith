use serde_json::Value;

use crate::OutputShape;

#[derive(Debug)]
pub struct NoneOutput {}

impl OutputShape for NoneOutput {
    fn print_output(&self, _content: Value) {}
}
