use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;

use crate::v1spec::compiled::param::ParamCompiled;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModuleCompiled {
    pub name: String,
    pub about: Option<String>,
    #[serde(default)]
    pub base_uri: Option<String>,
    pub commands: Vec<CommandCompiled>,
}

#[skip_serializing_none]
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
    #[serde(flatten)]
    pub bindings: BTreeMap<String, String>,
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
