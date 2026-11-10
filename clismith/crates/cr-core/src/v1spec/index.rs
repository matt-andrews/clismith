use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Index {
    pub name: String,
    pub core: String,
    pub modules: Vec<IndexModule>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct IndexModule {
    pub name: String,
    pub version: String,
    pub target: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct IndexDisplayModule {
    #[serde(rename = "module")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "hash")]
    pub sha256: String,
    #[serde(rename = "target")]
    pub target: String,
}

impl From<IndexModule> for IndexDisplayModule {
    fn from(value: IndexModule) -> Self {
        Self {
            name: value.name.clone(),
            version: value.version.clone(),
            sha256: value.sha256.clone(),
            target: value.target.clone(),
        }
    }
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
