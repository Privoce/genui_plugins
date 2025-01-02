use std::collections::HashMap;

use crate::Header;

#[derive(Debug, Clone, Default)]
pub struct PatchRequest {
    pub params: Option<HashMap<String, String>>,
    pub headers: Option<HashMap<Header, String>>,
    pub body: Option<Vec<u8>>,
}

impl PatchRequest {
    pub fn new(
        params: Option<HashMap<String, String>>,
        headers: Option<HashMap<Header, String>>,
        body: Option<Vec<u8>>,
    ) -> Self {
        Self {
            params,
            headers,
            body,
        }
    }
}

impl From<HashMap<String, String>> for PatchRequest {
    fn from(value: HashMap<String, String>) -> Self {
        Self {
            params: Some(value),
            headers: None,
            body: None,
        }
    }
}

impl From<Vec<u8>> for PatchRequest {
    fn from(value: Vec<u8>) -> Self {
        Self {
            params: None,
            headers: None,
            body: Some(value),
        }
    }
}

impl From<HashMap<Header, String>> for PatchRequest {
    fn from(value: HashMap<Header, String>) -> Self {
        Self {
            params: None,
            headers: Some(value),
            body: None,
        }
    }
}

impl From<(HashMap<String, String>, Vec<u8>)> for PatchRequest {
    fn from(value: (HashMap<String, String>, Vec<u8>)) -> Self {
        Self {
            params: Some(value.0),
            headers: None,
            body: Some(value.1),
        }
    }
}

impl From<(HashMap<String, String>, HashMap<Header, String>)> for PatchRequest {
    fn from(value: (HashMap<String, String>, HashMap<Header, String>)) -> Self {
        Self {
            params: Some(value.0),
            headers: Some(value.1),
            body: None,
        }
    }
}

impl From<(HashMap<String, String>, HashMap<Header, String>, Vec<u8>)> for PatchRequest {
    fn from(value: (HashMap<String, String>, HashMap<Header, String>, Vec<u8>)) -> Self {
        Self {
            params: Some(value.0),
            headers: Some(value.1),
            body: Some(value.2),
        }
    }
}
