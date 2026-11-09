use std::collections::BTreeMap;

use cr_core::v1spec::{
    authored::{core::CoreSpec, module::ModuleSpec},
    index::Index,
};
use oas3::Spec;

pub struct LoadResponse {
    pub index: Index,
    pub project: CoreSpec,
    pub modules: Vec<ModuleSpec>,
    pub oapi_specs: BTreeMap<String, Spec>,
}
pub struct MergeResponse {
    pub index: Index,
    pub project: CoreSpec,
    pub modules: Vec<ModuleSpec>,
}
