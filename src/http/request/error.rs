#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Error {
    HttpMethodParseFailed,
    PathParseError,
    RequestSizeExceeded { max_bytes: usize, bytes: usize },
    PathSizeExceeded { max_bytes: usize, bytes: usize },
}
