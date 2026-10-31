use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::v1spec::compiled::param::ParamCompiled;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleCompiled {
    pub name: String,
    pub about: Option<String>,
    pub commands: Vec<CommandCompiled>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandCompiled {
    #[serde(default)]
    pub about: Option<String>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub route: Option<String>,
    #[serde(default)]
    pub args: Vec<ParamCompiled>,
    #[serde(default)]
    pub flags: Vec<ParamCompiled>,
    #[serde(default)]
    pub result: Option<SchemaRef>,
    #[serde(flatten)]
    pub bindings: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SchemaRef {
    #[serde(rename = "$ref")]
    pub reference: String,
}

impl CommandCompiled {
    pub fn describe(&self) -> String {
        self.bindings
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(" ")
    }
}
