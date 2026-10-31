use serde::{Deserialize, Serialize};

use crate::v1spec::value_kind::ValueKind;

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
}
