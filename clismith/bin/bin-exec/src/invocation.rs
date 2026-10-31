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

pub const SYS_VERBOSE: &str = "SYS_VERBOSE";
pub const SYS_OUTPUT: &str = "SYS_OUTPUT";
pub const SYS_HEAD: &str = "SYS_HEAD";

#[derive(Debug, Clone)]
pub struct Invocation {
    pub path: Vec<String>,
    pub bindings: BTreeMap<String, String>,
    pub params: BTreeMap<String, Value>,
    pub triggered_command: Option<CommandCompiled>,
}

impl Invocation {
    pub fn str(&self, name: &str) -> Option<&str> {
        match self.params.get(name)? {
            Value::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn is_verbose(&self) -> bool {
        self.flag(SYS_VERBOSE)
    }

    pub fn get_output(&self) -> Option<&str> {
        self.str(SYS_OUTPUT)
    }

    pub fn is_head(&self) -> bool {
        self.flag(SYS_HEAD)
    }

    pub fn flag(&self, name: &str) -> bool {
        matches!(self.params.get(name), Some(Value::Bool(true)))
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
