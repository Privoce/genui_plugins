use std::fmt::Display;

use makepad_widgets::HttpMethod;

#[derive(Debug, Clone, PartialEq, Hash, Copy)]
pub enum Method{
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl Display for Method{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::GET => "GET",
            Self::HEAD => "HEAD",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::DELETE => "DELETE",
            Self::CONNECT => "CONNECT",
            Self::OPTIONS => "OPTIONS",
            Self::TRACE => "TRACE",
            Self::PATCH => "PATCH",
        };
        f.write_str(v)
    }
}

impl From<Method> for HttpMethod {
    fn from(value: Method) -> Self {
        match value {
            Method::GET => HttpMethod::GET,
            Method::HEAD => HttpMethod::HEAD,
            Method::POST => HttpMethod::POST,
            Method::PUT => HttpMethod::PUT,
            Method::DELETE => HttpMethod::DELETE,
            Method::CONNECT => HttpMethod::CONNECT,
            Method::OPTIONS => HttpMethod::OPTIONS,
            Method::TRACE => HttpMethod::TRACE,
            Method::PATCH => HttpMethod::PATCH,
        }
    }
}