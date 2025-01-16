mod config;
use std::error::Error;
pub use config::*;
use makepad_widgets::*;

use crate::http_request;

use super::{Method, PatchRequest};

/// # HTTP Request Publisher
/// Used to publish HTTP requests. There can be multiple publishers during initialization, and each publisher can have different configurations.
/// ## Supported Methods
/// - GET
/// - POST
/// - PUT
/// - DELETE
/// - PATCH
/// ## Meaning
/// - BasicConfig:
///     - Protocol::Http
///     - url: `127.0.0.1:6666`
///     - headers: `Content-Type: application/json`
/// - PatchRequest:
///     - params: `None`
///     - headers: `flexible_header: "flexible_value"`
///     - body: `None`
/// - Url: `/test`
/// as above, the real request will be:
/// - url: `http://127.0.0.1:6666/test`
/// - headers: `[Content-Type: application/json, flexible_header: "flexible_value"]`
/// - body: `None`
/// ## Example
/// ```rust
/// fn init() -> HttpPublisher {
///     let mut publisher = HttpPublisher::new("127.0.0.1:5800");
///     publisher.before_request = Some(|_request| {
///         println!("before_request");
///     });
///     publisher
/// }
///
/// plugin! {
///     http1: HttpPublisher => init()
/// }
/// ```
pub struct HttpPublisher {
    pub basic: BasicConfig,
    pub before_request: Option<fn(&mut HttpRequest)>,
}

impl HttpPublisher {
    pub fn new(url: &str) -> Self {
        Self {
            basic: BasicConfig::new(url.to_string()),
            before_request: None,
        }
    }
    http_request! {
        get: Method::GET
        post: Method::POST
        put: Method::PUT
        delete: Method::DELETE
        patch: Method::PATCH
    }
    pub fn send_request<P, M>(
        &self,
        url: &str,
        patch: P,
        method: M,
        live_id: LiveId,
    ) -> Result<HttpRequest, Box<dyn Error>>
    where
        M: Into<HttpMethod>,
        P: Into<PatchRequest>,
    {
        let patch_request: PatchRequest = patch.into();
        let url = format!("{}{}{}", self.basic.protocol, self.basic.url, url);
        let mut headers = self.basic.headers.clone();
        if let Some(patch_headers) = patch_request.headers {
            headers.extend(patch_headers);
        }
        let mut request = HttpRequest {
            metadata_id: live_id,
            url,
            method: method.into(),
            headers: headers
                .into_iter()
                .map(|(k, v)| (k.to_string(), vec![v]))
                .collect(),
            ignore_ssl_cert: false,
            is_streaming: false,
            body: patch_request.body.map(|body| body.into()),
        };

        // do handler before_request
        if let Some(before_request) = self.before_request.as_ref() {
            before_request(&mut request);
        }

        Ok(request)
    }
}
