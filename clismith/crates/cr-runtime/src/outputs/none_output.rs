use serde_json::Value;

use crate::outputs::OutputShape;

#[derive(Debug)]
pub struct NoneOutput {}

impl OutputShape for NoneOutput {
    fn print_output(&self, _content: Value) {}
}
