use std::collections::BTreeMap;
use std::path::PathBuf;

use cr_core::v1spec::compiled::module::CommandCompiled;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Path(PathBuf),
    List(Vec<Value>),
}

pub const SYS_FLAG_VERBOSE: &str = "SYS_FLAG_VERBOSE";
pub const SYS_FLAG_OUTPUT: &str = "SYS_FLAG_OUTPUT";
pub const SYS_FLAG_HEAD: &str = "SYS_FLAG_HEAD";
pub const SYS_FLAG_OUTPUT_PROP: &str = "SYS_FLAG_OUTPUT_PROP";
pub const SYS_FLAG_OUTPUT_SORT: &str = "SYS_FLAG_OUTPUT_SORT";

#[derive(Debug, Clone)]
pub struct Invocation {
    pub path: Vec<String>,
    pub bindings: BTreeMap<String, String>,
    pub params: BTreeMap<String, Value>,
    pub triggered_command: CommandCompiled,
    pub base_uri: String,
}

impl Invocation {
    pub fn str(&self, name: &str) -> Option<&str> {
        match self.params.get(name)? {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_verbose(&self) -> bool {
        self.flag(SYS_FLAG_VERBOSE)
    }

    pub fn get_output(&self) -> Option<&str> {
        self.str(SYS_FLAG_OUTPUT)
    }

    pub fn is_head(&self) -> bool {
        self.flag(SYS_FLAG_HEAD)
    }

    pub fn flag(&self, name: &str) -> bool {
        matches!(self.params.get(name), Some(Value::Bool(true)))
    }

    pub fn list(&self, name: &str) -> Option<Vec<String>> {
        match self.params.get(name) {
            Some(Value::List(vs)) => Some(
                vs.iter()
                    .filter_map(|m| match m {
                        Value::Str(s) => Some(s.to_owned()),
                        _ => None,
                    })
                    .collect(),
            ),
            _ => None,
        }
    }

    /* pub fn list(&self, name: &str) -> &[Value] {
        match self.params.get(name) {
            Some(Value::List(vs)) => vs,
            _ => &[],
        }
    }

    pub fn int(&self, name: &str) -> Option<i64> {
        match self.params.get(name)? {
            Value::Int(n) => Some(*n),
            _ => None,
        }
    }

    pub fn float(&self, name: &str) -> Option<f64> {
        match self.params.get(name)? {
            Value::Float(n) => Some(*n),
            _ => None,
        }
    }

    pub fn path(&self, name: &str) -> Option<&std::path::Path> {
        match self.params.get(name)? {
            Value::Path(p) => Some(p),
            _ => None,
        }
    } */
}
