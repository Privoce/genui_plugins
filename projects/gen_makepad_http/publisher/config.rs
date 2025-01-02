use std::collections::HashMap;

use crate::{ContentType, Header, Protocol};

/// # 基础HTTP配置
/// 当后续HTTP请求没有任何特殊配置时，使用此配置
#[derive(Debug, Clone)]
pub struct BasicConfig {
    pub protocol: Protocol,
    pub url: String,
    pub headers: HashMap<Header, String>,
}

impl BasicConfig {
    pub fn new(url: String) -> Self {
        BasicConfig {
            protocol: Protocol::Http,
            url,
            headers: vec![(Header::ContentType, ContentType::Json.to_string())]
                .into_iter()
                .collect(),
        }
    }
}
