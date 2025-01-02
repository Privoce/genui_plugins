mod config;
mod handlers;

use std::error::Error;

pub use config::*;
pub use handlers::*;
use makepad_widgets::*;

use super::PatchRequest;

/// # HTTP请求发布者
/// 用于发布HTTP请求，在初始化时可以有多个发布者，每个发布者可以有不同的配置
pub struct HttpPublisher {
    pub basic: BasicConfig,
    pub handlers: Option<Handlers>,
}

impl HttpPublisher {
    pub fn new(url: &str) -> Self {
        Self {
            basic: BasicConfig::new(url.to_string()),
            handlers: None,
        }
    }
    pub fn get<P>(&self, url: &str, patch: P) -> Result<HttpRequest, Box<dyn Error>>
    where
        P: Into<PatchRequest>,
    {
        let patch_request: PatchRequest = patch.into();
        let url = format!("{}{}{}", self.basic.protocol, self.basic.url, url);
        let mut headers = self.basic.headers.clone();

        if let Some(patch_headers) = patch_request.headers {
            headers.extend(patch_headers);
        }

        Ok(HttpRequest {
            metadata_id: live_id!(A),
            url,
            method: HttpMethod::GET,
            headers: headers
                .into_iter()
                .map(|(k, v)| (k.to_string(), vec![v]))
                .collect(),
            ignore_ssl_cert: false,
            is_streaming: false,
            body: patch_request.body,
        })
    }
}
