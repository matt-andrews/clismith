use std::collections::HashMap;

use reqwest::{
    Error, Method,
    header::{HeaderMap, HeaderValue},
};
use serde_json::Value;

use crate::{
    request::{
        HttpMethod::{self},
        HttpRequestMessage,
    },
    response::HttpResponseMessage,
};

pub mod request;
pub mod response;

pub async fn http_dispatch(req: HttpRequestMessage) -> Result<HttpResponseMessage, Error> {
    let url = format!(
        "{}/{}",
        req.base_uri.trim_end_matches('/'),
        req.route.trim_start_matches('/')
    );

    log_if(&req, &url);

    let raw_result = reqwest::Client::new()
        .request(map_method(req.method), url)
        .send()
        .await?;

    log_if(&req, &format!("{raw_result:#?}"));

    if req.output_type_headers {
        let headers = raw_result.headers().clone();
        Ok(HttpResponseMessage {
            content: headermap_to_json(&headers).unwrap_or_default(),
        })
    } else {
        let json_object: Value = raw_result.json().await?;
        Ok(HttpResponseMessage {
            content: json_object,
        })
    }
}

fn log_if(req: &HttpRequestMessage, message: &str) {
    if req.verbose {
        eprintln!("{}", message);
    }
}

fn map_method(method: HttpMethod) -> Method {
    match method {
        HttpMethod::Get => Method::GET,
        HttpMethod::Put => Method::PUT,
        HttpMethod::Post => Method::POST,
        HttpMethod::Patch => Method::PATCH,
        HttpMethod::Delete => Method::DELETE,
        HttpMethod::Options => Method::OPTIONS,
    }
}

fn headermap_to_json(headers: &HeaderMap<HeaderValue>) -> Result<Value, serde_json::Error> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for (key, value) in headers {
        let key_str = key.as_str().to_string();
        let val_str = String::from_utf8_lossy(value.as_bytes()).to_string();

        map.entry(key_str).or_default().push(val_str);
    }

    serde_json::to_value(map)
}
