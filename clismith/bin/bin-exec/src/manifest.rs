use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub name: String,
    pub core: String,
    pub modules: Vec<ManifestModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestModule {
    pub name: String,
    pub version: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestDisplayModule {
    #[serde(rename = "module")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
}

impl Manifest {
    pub fn latest_module<'a>(&'a self, name: &str) -> Option<&'a ManifestModule> {
        //
        self.modules
            .iter()
            .filter(|m| m.name == name)
            .filter_map(|m| Version::parse(&m.version).ok().map(|v| (v, m)))
            .max_by(|(a, _), (b, _)| a.cmp(b))
            .map(|(_, m)| m)
    }
}

impl ManifestModule {
    pub fn to_display_module(&self) -> ManifestDisplayModule {
        ManifestDisplayModule {
            name: self.name.clone(),
            version: self.version.clone(),
        }
    }
}
