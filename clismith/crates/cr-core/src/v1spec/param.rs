use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::v1spec::{
    diff::{Bump, Diff, cmp_field, cmp_field_with, diff_string_list},
    param_where::ParamWhere,
    value_kind::ValueKind,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ParamSpec {
    #[serde(alias = "name")]
    pub long: String,
    #[serde(default)]
    pub short: Option<char>,
    #[serde(default)]
    pub about: Option<String>,
    #[serde(default, rename = "type")]
    pub kind: ValueKind,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub multiple: bool,
    #[serde(default)]
    pub default: Option<String>,
    #[serde(default)]
    pub choices: Option<Vec<String>>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default, rename = "where")]
    pub param_where: ParamWhere,
    #[serde(default)]
    pub allow_hyphen_values: Option<bool>,
}

impl Diff for ParamSpec {
    fn diff(&self, new: &Self, path: &str, out: &mut Vec<crate::v1spec::diff::Change>) {
        cmp_field(out, path, "long", &self.long, &new.long, Bump::Major);
        cmp_field(out, path, "short", &self.short, &new.short, Bump::Major);
        cmp_field(out, path, "about", &self.about, &new.about, Bump::Minor);
        cmp_field(out, path, "kind", &self.kind, &new.kind, Bump::Major);
        cmp_field_with(
            out,
            path,
            "allow_hyphen_values",
            &self.allow_hyphen_values,
            &new.allow_hyphen_values,
            |_, &now| {
                if now.unwrap_or_default() {
                    Bump::Minor
                } else {
                    Bump::Major
                }
            },
        );
        cmp_field_with(
            out,
            path,
            "required",
            &self.required,
            &new.required,
            |_, &now| {
                if now { Bump::Major } else { Bump::Minor }
            },
        );
        cmp_field_with(
            out,
            path,
            "multiple",
            &self.multiple,
            &new.multiple,
            |_, &now| {
                if now { Bump::Minor } else { Bump::Major }
            },
        );
        cmp_field(
            out,
            path,
            "default",
            &self.default,
            &new.default,
            Bump::Major,
        );

        diff_string_list(
            &self.choices,
            &new.choices,
            "choices",
            out,
            Bump::Minor,
            Bump::Major,
        );

        cmp_field(out, path, "target", &self.target, &new.target, Bump::Major);
        cmp_field(
            out,
            path,
            "param_where",
            &self.param_where,
            &new.param_where,
            Bump::Major,
        );
    }
}
