use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::v1spec::{param_where::ParamWhere, value_kind::ValueKind};

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ParamCompiled {
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
