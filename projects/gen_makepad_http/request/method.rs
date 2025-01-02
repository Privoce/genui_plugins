use std::fmt::Display;

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