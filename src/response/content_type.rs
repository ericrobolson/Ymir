#[derive(PartialEq, Copy, Clone, Debug)]
pub enum ContentType {
    Json,
    Html,
}

impl ContentType {
    pub fn to_header_value(&self) -> &'static str {
        match self {
            ContentType::Json => "application/json",
            ContentType::Html => "text/html",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json() {
        let content = ContentType::Json;
        assert_eq!("application/json", content.to_header_value());
    }

    #[test]
    fn html() {
        let content = ContentType::Html;
        assert_eq!("text/html", content.to_header_value());
    }
}
