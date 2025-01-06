use std::collections::HashMap;

use crate::{ContentType, HttpRequestHeader, Protocol};

/// # Basic Config for Http Publisher
/// the real request will be: `BasicConfig` + `PatchRequest`
/// ## Default Config
/// - protocol: `Protocol::Http`
/// - headers: `Content-Type: application/json`
#[derive(Debug, Clone)]
pub struct BasicConfig {
    pub protocol: Protocol,
    pub url: String,
    pub headers: HashMap<HttpRequestHeader, String>,
}

impl BasicConfig {
    pub fn new(url: String) -> Self {
        BasicConfig {
            protocol: Protocol::Http,
            url,
            headers: vec![(HttpRequestHeader::ContentType, ContentType::Json.to_string())]
                .into_iter()
                .collect(),
        }
    }
    pub fn push_header(&mut self, header: HttpRequestHeader, value: String) {
        self.headers.insert(header, value);
    }
}
