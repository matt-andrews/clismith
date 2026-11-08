use serde::{Deserialize, Serialize};

use crate::v1spec::{compiled, param_where::ParamWhere, value_kind::ValueKind};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParamSpec {
    #[serde(alias = "name")]
    pub long: String,
    #[serde(default)]
    pub short: Option<char>,
    #[serde(default)]
    pub about: Option<String>,
    #[serde(default, rename = "type")]
    pub kind: ValueKind,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub multiple: bool,
    #[serde(default)]
    pub default: Option<String>,
    #[serde(default)]
    pub choices: Option<Vec<String>>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default, rename = "where")]
    pub param_where: ParamWhere,
}

impl ParamSpec {
    pub fn to_compiled(&self) -> compiled::param::ParamCompiled {
        compiled::param::ParamCompiled {
            long: self.long.to_string(),
            short: self.short,
            about: self.about.clone(),
            kind: self.kind,
            required: self.required,
            multiple: self.multiple,
            default: self.default.clone(),
            choices: self.choices.clone(),
            target: self.target.clone(),
            param_where: self.param_where,
        }
    }
}
