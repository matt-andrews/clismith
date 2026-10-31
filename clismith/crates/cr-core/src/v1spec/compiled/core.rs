use serde::{Deserialize, Serialize};

use crate::v1spec::compiled::param::ParamCompiled;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoreCompiled {
    pub name: String,
    pub about: Option<String>,
    pub grammar: Grammar,

    #[serde(default)]
    pub global_flags: Vec<ParamCompiled>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Grammar {
    pub pattern: String,
    pub naming: Naming,
    pub verbs: Vec<VerbDefinition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Naming {
    pub case: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerbDefinition {
    pub method: String,
    pub target: String,
    pub verb: String,
}
