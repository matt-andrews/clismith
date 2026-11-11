use std::collections::BTreeMap;

use serde::Deserialize;

#[allow(dead_code)]
pub struct HttpRequestMessage {
    pub route: String,
    pub base_uri: String,
    pub method: HttpMethod,
    pub headers: BTreeMap<String, String>,
    pub body: Option<serde_json::Value>,
    pub verbose: bool,
    pub output_type_headers: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum HttpMethod {
    Get,
    Put,
    Post,
    Patch,
    Delete,
    Options,
}
