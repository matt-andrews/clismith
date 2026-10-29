use serde::Deserialize;
use std::collections::BTreeMap;

use crate::v1spec::common::ParamSpec;

#[derive(Debug, Clone, Deserialize)]
pub struct ModuleSpec {
    pub name: String,
    pub about: Option<String>,
    pub commands: Vec<CommandSpec>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommandSpec {
    #[serde(default)]
    pub about: Option<String>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub route: Option<String>,
    #[serde(default)]
    pub args: Vec<ParamSpec>,
    #[serde(default)]
    pub flags: Vec<ParamSpec>,
    #[serde(default)]
    pub result: Option<SchemaRef>,
    #[serde(flatten)]
    pub bindings: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SchemaRef {
    #[serde(rename = "$ref")]
    pub reference: String,
}

impl CommandSpec {
    pub fn describe(&self) -> String {
        self.bindings
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(" ")
    }
}
