use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{collections::BTreeMap, path::PathBuf};

use crate::v1spec::{
    diff::{Bump, Diff, cmp_field, diff_by_key},
    param::ParamSpec,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ModuleSpec {
    pub name: String,
    #[serde(default = "vspec")]
    pub vspec: String,
    #[serde(default)]
    pub openapi_spec: Option<PathBuf>,
    #[serde(default)]
    pub about: Option<String>,
    #[serde(default)]
    pub base_uri: Option<String>,
    #[serde(default)]
    pub commands: Vec<CommandSpec>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CommandSpec {
    #[serde(default)]
    pub about: Option<String>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub route: Option<String>,
    #[serde(default)]
    pub args: Vec<ParamSpec>,
    #[serde(default)]
    pub flags: Vec<ParamSpec>,
    #[serde(default)]
    pub request_body: Option<RequestBody>,
    #[serde(flatten)]
    pub bindings: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RequestBody {
    pub content_type: String,
    pub schema: serde_json::Value,
    pub required: bool,
}

impl CommandSpec {
    pub fn describe(&self) -> String {
        self.bindings
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl Diff for ModuleSpec {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(out, path, "name", &self.name, &new.name, Bump::Major);
        cmp_field(out, path, "about", &self.about, &new.about, Bump::Minor);
        cmp_field(
            out,
            path,
            "base_uri",
            &self.base_uri,
            &new.base_uri,
            Bump::Major,
        );
        diff_by_key(
            &self.commands,
            &new.commands,
            &format!("{path}.commands"),
            out,
            |key| &key.target,
        );
    }
}

impl Diff for CommandSpec {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(out, path, "about", &self.about, &new.about, Bump::Minor);
        cmp_field(out, path, "target", &self.target, &new.target, Bump::Major);
        cmp_field(out, path, "method", &self.method, &new.method, Bump::Major);
        cmp_field(out, path, "route", &self.route, &new.route, Bump::Major);
        diff_by_key(&self.args, &new.args, &format!("{path}.args"), out, |key| {
            &key.long
        });
        diff_by_key(
            &self.flags,
            &new.flags,
            &format!("{path}.flags"),
            out,
            |key| &key.long,
        );
        cmp_field(
            out,
            path,
            "bindings",
            &self.bindings,
            &new.bindings,
            Bump::Major,
        );
    }
}

fn vspec() -> String {
    "v1".to_string()
}
