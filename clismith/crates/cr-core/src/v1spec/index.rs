use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Index {
    pub name: String,
    pub core: String,
    pub modules: Vec<IndexModule>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IndexModule {
    pub name: String,
    pub version: String,
    pub target: String,
    pub sha256: String,
}

impl Index {
    pub fn new(name: &str, core_module_file: &str) -> Self {
        Self {
            name: name.to_string(),
            modules: Vec::new(),
            core: core_module_file.to_string(),
        }
    }
    pub fn latest_module<'a>(&'a self, name: &str) -> Option<&'a IndexModule> {
        //
        self.modules
            .iter()
            .filter(|m| m.name == name)
            .filter_map(|m| Version::parse(&m.version).ok().map(|v| (v, m)))
            .max_by(|(a, _), (b, _)| a.cmp(b))
            .map(|(_, m)| m)
    }
}
