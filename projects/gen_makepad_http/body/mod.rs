/// # HttpRequestBody
/// this is a wrapper around a Vec<u8> that can be converted from a serde::Serialize
/// ## Example
/// ```rust
/// #[derive(serde::Serialize)]
/// struct Person{
///    name: String,
/// }
/// 
/// let body = Person{name: "John".to_string()};
/// 
/// let _ = http_post!(PersonReq, "/post", body, cx);
/// ```
#[derive(Debug, Clone, Default)]
pub struct HttpRequestBody(pub Vec<u8>);

impl<T> From<T> for HttpRequestBody
where
    T: serde::Serialize,
{
    fn from(value: T) -> Self {
        let body = serde_json::to_vec(&value).unwrap();
        Self(body)
    }
}

impl From<HttpRequestBody> for Vec<u8> {
    fn from(value: HttpRequestBody) -> Self {
        value.0
    }
}
