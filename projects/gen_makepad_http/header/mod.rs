mod content_type;

use std::fmt::Display;

pub use content_type::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum Header {
    ContentType,
    Authorization,
    Other(String),
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::ContentType => "Content-Type",
            Self::Authorization => "Authorization",
            Self::Other(s) => s,
        };
        f.write_str(v)
    }
}
