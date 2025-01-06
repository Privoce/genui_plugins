use std::collections::HashMap;

use crate::{HttpRequestBody, HttpRequestHeader};

#[derive(Debug, Clone, Default)]
pub struct PatchRequest {
    pub params: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<HttpRequestHeader, String>>,
    pub body: Option<HttpRequestBody>,
}

impl PatchRequest {
    pub fn new(
        params: Option<HashMap<String, String>>,
        headers: Option<HashMap<HttpRequestHeader, String>>,
        body: Option<HttpRequestBody>,
    ) -> Self {
        Self {
            params,
            headers,
            body,
        }
    }
    pub fn push_param(&mut self, key: String, value: String) {
        if let Some(params) = &mut self.params {
            params.insert(key, value);
        } else {
            let mut params = HashMap::new();
            params.insert(key, value);
            self.params = Some(params);
        }
    }

    pub fn push_header(&mut self, key: HttpRequestHeader, value: String) {
        if let Some(headers) = &mut self.headers {
            headers.insert(key, value);
        } else {
            let mut headers = HashMap::new();
            headers.insert(key, value);
            self.headers = Some(headers);
        }
    }
}

impl<T> From<T> for PatchRequest
where
    T: serde::Serialize,
{
    fn from(value: T) -> Self {
        Self {
            params: None,
            headers: None,
            body: Some(value.into()),
        }
    }
}
