mod error;
mod method;
pub use error::Error;
pub use method::{parse_method, Method};

use crate::parser::RawIter;

pub const MAX_REQUEST_BYTES: usize = 8192;
pub const MAX_PATH_BYTES: usize = 256;

pub fn parse(raw_request: &[u8]) -> Result<Request, Error> {
    if raw_request.len() > MAX_REQUEST_BYTES {
        return Err(Error::RequestSizeExceeded {
            max_bytes: MAX_REQUEST_BYTES,
            bytes: raw_request.len(),
        });
    }

    let mut raw_iter = RawIter::new(raw_request);
    let method = parse_method(&mut raw_iter)?;
    let path = parse_path(&mut raw_iter)?;
    Ok(Request { method, path })
}

#[derive(PartialEq, Clone, Debug)]
pub struct Request {
    method: Method,
    path: Path,
}

impl<'a> Request {
    pub fn method(&self) -> Method {
        self.method
    }

    pub fn path(&self) -> Path {
        self.path
    }
}

const HTTP_VERSION: &'static [u8] = b"HTTP/1.1";

#[derive(PartialEq, Copy, Clone)]
pub struct Path {
    path: [u8; MAX_PATH_BYTES],
    len: usize,
}

impl Path {
    pub fn bytes(&self) -> &[u8] {
        &self.path[..self.len]
    }

    pub fn to_str(&self) -> &str {
        core::str::from_utf8(&self.path[..self.len]).unwrap()
    }
}

impl core::fmt::Debug for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let data = core::str::from_utf8(&self.path[..self.len]).unwrap();
        f.debug_struct("Path").field("path", &data).finish()
    }
}

fn parse_path(request: &mut RawIter) -> Result<Path, Error> {
    let mut found_route = false;
    let mut end_index = 0;
    while end_index < request.data().len() {
        if request.data()[end_index..].starts_with(HTTP_VERSION) {
            found_route = true;
            break;
        }

        end_index += 1;
    }

    if !found_route {
        end_index = 0;
    }

    let mut len = 0;
    let path = {
        let mut path = [0; MAX_PATH_BYTES];
        if end_index >= MAX_PATH_BYTES {
            return Err(Error::PathSizeExceeded {
                max_bytes: MAX_PATH_BYTES,
                bytes: end_index + 1,
            });
        }

        {
            let s = match core::str::from_utf8(&request.data()[..end_index]) {
                Ok(s) => s,
                Err(e) => {
                    return Err(Error::PathParseError);
                }
            }
            .trim();

            for byte in s.as_bytes() {
                path[len] = *byte;
                len += 1;
            }
        }

        path
    };

    request.advance(end_index);
    Ok(Path { path, len })
}
