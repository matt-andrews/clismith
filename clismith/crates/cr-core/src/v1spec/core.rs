use serde::{Deserialize, Serialize};

use crate::v1spec::{
    diff::{Bump, Diff, cmp_field, diff_by_key, diff_opt_by_key},
    param::ParamSpec,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoreSpec {
    pub name: String,
    #[serde(default = "vspec")]
    pub vspec: String,
    pub about: Option<String>,
    pub grammar: Grammar,

    #[serde(default)]
    pub required_modules: Option<Vec<RequiredModule>>,

    #[serde(default)]
    pub is_singleton: Option<bool>,

    #[serde(default)]
    pub registries: Option<Vec<Registry>>,

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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequiredModule {
    pub module: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Registry {
    pub index: String,
    pub secure: bool,
}

impl Diff for CoreSpec {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(out, path, "name", &self.name, &new.name, Bump::Major);
        cmp_field(out, path, "about", &self.about, &new.about, Bump::Minor);

        self.grammar
            .diff(&new.grammar, &format!("{path}.grammar"), out);

        diff_opt_by_key(
            &self.required_modules,
            &new.required_modules,
            &format!("{path}.required_modules"),
            out,
            |key| &key.module,
        );

        diff_opt_by_key(
            &self.registries,
            &new.registries,
            &format!("{path}.registries"),
            out,
            |key| &key.index,
        );

        diff_by_key(
            &self.global_flags,
            &new.global_flags,
            &format!("{path}.global_flags"),
            out,
            |key| &key.long,
        );
    }
}

impl Diff for Grammar {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(
            out,
            path,
            "pattern",
            &self.pattern,
            &new.pattern,
            Bump::Major,
        );

        self.naming
            .diff(&new.naming, &format!("{path}.naming"), out);

        diff_by_key(
            &self.verbs,
            &new.verbs,
            &format!("{path}.verbs"),
            out,
            |key| (&key.method, &key.target, &key.verb),
        );
    }
}

impl Diff for Naming {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(out, path, "case", &self.case, &new.case, Bump::Major);
    }
}

impl Diff for VerbDefinition {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(out, path, "method", &self.method, &new.method, Bump::Major);
        cmp_field(out, path, "target", &self.target, &new.target, Bump::Major);
        cmp_field(out, path, "verb", &self.verb, &new.verb, Bump::Major);
    }
}

impl Diff for RequiredModule {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(out, path, "module", &self.module, &new.module, Bump::Major);
        cmp_field(
            out,
            path,
            "version",
            &self.version,
            &new.version,
            Bump::Major,
        );
    }
}

impl Diff for Registry {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(out, path, "index", &self.index, &new.index, Bump::Major);
        cmp_field(out, path, "secure", &self.secure, &new.secure, Bump::Major);
    }
}

fn vspec() -> String {
    "v1".to_string()
}
