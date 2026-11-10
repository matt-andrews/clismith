use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

use crate::v1spec::{
    authored::{Compiled, param::ParamSpec},
    compiled,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleSpec {
    pub name: String,
    #[serde(default)]
    pub openapi_spec: Option<PathBuf>,
    #[serde(default)]
    pub about: Option<String>,
    #[serde(default)]
    pub base_uri: Option<String>,
    #[serde(default)]
    pub commands: Vec<CommandSpec>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SchemaRef {
    #[serde(rename = "$ref")]
    pub reference: String,
}

impl Compiled<compiled::module::ModuleCompiled> for ModuleSpec {
    fn to_compiled(&self) -> compiled::module::ModuleCompiled {
        compiled::module::ModuleCompiled {
            name: self.name.to_string(),
            about: self.about.clone(),
            commands: self
                .commands
                .clone()
                .into_iter()
                .map(|m| m.to_compiled())
                .collect(),
            base_uri: self.base_uri.clone(),
        }
    }
}

impl Compiled<compiled::module::CommandCompiled> for CommandSpec {
    fn to_compiled(&self) -> compiled::module::CommandCompiled {
        compiled::module::CommandCompiled {
            about: self.about.clone(),
            target: self.target.clone(),
            method: self.method.clone(),
            route: self.route.clone(),
            args: self
                .args
                .clone()
                .into_iter()
                .map(|m| m.to_compiled())
                .collect(),
            flags: self
                .flags
                .clone()
                .into_iter()
                .map(|m| m.to_compiled())
                .collect(),
            bindings: self.bindings.clone(),
        }
    }
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
