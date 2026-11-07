use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::v1spec::{authored::param::ParamSpec, compiled};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleSpec {
    pub name: String,
    pub about: Option<String>,
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

impl ModuleSpec {
    pub fn to_compiled(&self) -> compiled::module::ModuleCompiled {
        compiled::module::ModuleCompiled {
            name: self.name.to_string(),
            about: self.about.clone(),
            commands: self
                .commands
                .clone()
                .into_iter()
                .map(|m| m.to_compiled())
                .collect(),
        }
    }
}

impl CommandSpec {
    pub fn to_compiled(&self) -> compiled::module::CommandCompiled {
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
            result: self.result.clone().map(|m| m.to_compiled()),
            bindings: self.bindings.clone(),
        }
    }
    pub fn describe(&self) -> String {
        self.bindings
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl SchemaRef {
    pub fn to_compiled(&self) -> compiled::module::SchemaRef {
        compiled::module::SchemaRef {
            reference: self.reference.to_string(),
        }
    }
}
