#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Status {
    Ok,
    NotFound,
}

impl Status {
    pub fn to_status(&self) -> &'static str {
        match self {
            Status::Ok => "200 OK",
            Status::NotFound => "404 NOT FOUND",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_status_Ok() {
        let status = Status::Ok;
        assert_eq!("200 OK", status.to_status());
    }

    #[test]
    fn to_status_NotFound() {
        let status = Status::NotFound;
        assert_eq!("404 NOT FOUND", status.to_status());
    }
}
