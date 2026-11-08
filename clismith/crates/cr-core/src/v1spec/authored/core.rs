use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::v1spec::{
    authored::param::ParamSpec,
    compiled::{self, core::CoreCompiled},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoreSpec {
    pub name: String,
    pub about: Option<String>,
    pub grammar: Grammar,
    pub modules: Vec<PathBuf>,

    #[serde(default)]
    pub is_singleton: Option<bool>,

    #[serde(default)]
    pub global_flags: Vec<ParamSpec>,
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

impl CoreSpec {
    pub fn to_compiled(&self) -> CoreCompiled {
        CoreCompiled {
            name: self.name.to_string(),
            about: self.about.clone(),
            grammar: self.grammar.to_compiled(),
            global_flags: self
                .global_flags
                .clone()
                .into_iter()
                .map(|m| m.to_compiled())
                .collect(),
        }
    }
}

impl Grammar {
    pub fn to_compiled(&self) -> compiled::core::Grammar {
        compiled::core::Grammar {
            pattern: self.pattern.to_string(),
            naming: self.naming.to_compiled(),
            verbs: self
                .verbs
                .clone()
                .into_iter()
                .map(|m| m.to_compiled())
                .collect(),
        }
    }
}

impl Naming {
    pub fn to_compiled(&self) -> compiled::core::Naming {
        compiled::core::Naming {
            case: self.case.to_string(),
        }
    }
}

impl VerbDefinition {
    pub fn to_compiled(&self) -> compiled::core::VerbDefinition {
        compiled::core::VerbDefinition {
            method: self.method.to_string(),
            target: self.target.to_string(),
            verb: self.verb.to_string(),
        }
    }
}
