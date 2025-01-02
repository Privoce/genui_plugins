use std::fmt::Display;

pub enum ContentType {
    Text,
    Json,
    Xml,
    Html,
    Other(String),
}

impl Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::Text => write!(f, "text/plain"),
            ContentType::Json => write!(f, "application/json"),
            ContentType::Xml => write!(f, "application/xml"),
            ContentType::Html => write!(f, "text/html"),
            ContentType::Other(s) => write!(f, "{}", s),
        }
    }
}