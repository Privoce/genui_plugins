use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Protocol {
    Http,
    Https,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::Http => "http://",
            Self::Https => "https://",
        };
        f.write_str(v)
    }
}
