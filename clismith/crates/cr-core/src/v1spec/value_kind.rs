use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ValueKind {
    #[default]
    String,
    Int,
    Float,
    Bool,
    Path,
}
