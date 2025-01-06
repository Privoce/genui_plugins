mod content_type;

use std::fmt::Display;

pub use content_type::*;

/// # Http Request Header
/// this enum use in `BasicConfig` and `PatchRequest`
/// - ContentType: `Content-Type`
/// - Authorization: `Authorization`
/// - Other: `other header` (set by user)
/// ## Example
/// ```rust
/// let mut publisher = HttpPublisher::new("127.0.0.1:5800");
/// publisher.push_header(HttpRequestHeader::Authorization, "Bearer token".to_string());
/// publisher.push_header(HttpRequestHeader::Other("X-Request-Id".to_string, "123456".to_string());
/// // ----------------
/// let mut patch_request = PatchRequest::default();
/// patch_request.push_header(HttpRequestHeader::Authorization, "Bearer token".to_string());
/// ```
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum HttpRequestHeader {
    ContentType,
    Authorization,
    Other(String),
}

impl Display for HttpRequestHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::ContentType => "Content-Type",
            Self::Authorization => "Authorization",
            Self::Other(s) => s,
        };
        f.write_str(v)
    }
}

impl From<&str> for HttpRequestHeader {
    fn from(value: &str) -> Self {
        match value {
            "Content-Type" | "content-type" => Self::ContentType,
            "Authorization" | "authorization" => Self::Authorization,
            _ => Self::Other(value.to_string()),
        }
    }
}
