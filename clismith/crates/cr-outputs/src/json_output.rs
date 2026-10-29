use serde_json::Value;

use crate::OutputShape;

#[derive(Debug)]
pub struct JsonOutput {}

impl OutputShape for JsonOutput {
    fn print_output(&self, content: Value) {
        println!("{}", serde_json::to_string_pretty(&content).unwrap());
    }
}
