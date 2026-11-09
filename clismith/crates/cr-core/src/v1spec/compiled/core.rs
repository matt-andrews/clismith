use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::v1spec::{
    compiled::param::ParamCompiled,
    diff::{Bump, Diff, cmp_field, diff_by_key},
};

#[skip_serializing_none]
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

impl Diff for CoreCompiled {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(out, path, "name", &self.name, &new.name, Bump::Major);
        cmp_field(out, path, "about", &self.about, &new.about, Bump::Minor);

        self.grammar
            .diff(&new.grammar, &format!("{path}.grammar"), out);

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
