use super::error::Error;
use crate::parser::RawIter;

/// HTTP defines a set of request methods to indicate the desired action to be performed for a given resource. Although they can also be nouns, these request methods are sometimes referred to as HTTP verbs. Each of them implements a different semantic, but some common features are shared by a group of them: e.g. a request method can be safe, idempotent, or cacheable.
/// All docs for this are from the source: https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Method {
    /// The GET method requests a representation of the specified resource. Requests using GET should only retrieve data.
    /// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/GET
    Get,
    /// The HEAD method asks for a response identical to that of a GET request, but without the response body
    /// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/HEAD
    Head,
    /// The POST method is used to submit an entity to the specified resource, often causing a change in state or side effects on the server.
    /// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST
    Post,
    /// The PUT method replaces all current representations of the target resource with the request payload.
    /// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/PUT
    Put,
    /// The DELETE method deletes the specified resource.
    /// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/DELETE
    Delete,
    /// The CONNECT method establishes a tunnel to the server identified by the target resource.
    /// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/CONNECT
    Connect,
    /// The OPTIONS method is used to describe the communication options for the target resource.
    /// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/OPTIONS
    Options,
    /// The TRACE method performs a message loop-back test along the path to the target resource.
    /// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/TRACE
    Trace,
    /// The PATCH method is used to apply partial modifications to a resource.
    /// See https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/PATCH
    Patch,
}

macro_rules! parse_request_method {
    ($request:expr, [($str1:expr, $enum1:expr), $(($str:expr, $enum:expr)),*]) => {
        if $request.data().starts_with($str1) {
            $request.advance($str1.len());
            Ok($enum1)
        }
        $(
            else if $request.data().starts_with($str) {
                $request.advance($str.len());
                Ok($enum)
            }
        )*

        else {
            Err(Error::HttpMethodParseFailed)
        }
    };
}

pub fn parse_method(request: &mut RawIter) -> Result<Method, Error> {
    parse_request_method!(
        request,
        [
            (b"GET", Method::Get),
            (b"HEAD", Method::Head),
            (b"POST", Method::Post),
            (b"PUT", Method::Put),
            (b"DELETE", Method::Delete),
            (b"CONNECT", Method::Connect),
            (b"OPTIONS", Method::Options),
            (b"TRACE", Method::Trace),
            (b"PATCH", Method::Patch)
        ]
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::RawIter;

    #[test]
    fn parse_methodget_returns_ok() {
        let request = b"GET some stuff";
        let mut request = RawIter::new(request);
        let method = parse_method(&mut request);
        assert_eq!(true, method.is_ok());

        let expected = Method::Get;
        let actual = method.unwrap();
        assert_eq!(expected, actual);

        // Make sure the iterator was advanced properly
        assert_eq!(b"GET".len(), request.index());
    }

    #[test]
    fn parse_methodhead_returns_ok() {
        let request = b"HEAD some stuff";
        let mut request = RawIter::new(request);
        let method = parse_method(&mut request);
        assert_eq!(true, method.is_ok());

        let expected = Method::Head;
        let actual = method.unwrap();
        assert_eq!(expected, actual);

        // Make sure the iterator was advanced properly
        assert_eq!(b"HEAD".len(), request.index());
    }

    #[test]
    fn parse_methodpost_returns_ok() {
        let request = b"POST some stuff";
        let mut request = RawIter::new(request);
        let method = parse_method(&mut request);
        assert_eq!(true, method.is_ok());

        let expected = Method::Post;
        let actual = method.unwrap();
        assert_eq!(expected, actual);

        // Make sure the iterator was advanced properly
        assert_eq!(b"POST".len(), request.index());
    }

    #[test]
    fn parse_methodput_returns_ok() {
        let request = b"PUT some stuff";
        let mut request = RawIter::new(request);
        let method = parse_method(&mut request);
        assert_eq!(true, method.is_ok());

        let expected = Method::Put;
        let actual = method.unwrap();
        assert_eq!(expected, actual);

        // Make sure the iterator was advanced properly
        assert_eq!(b"PUT".len(), request.index());
    }

    #[test]
    fn parse_methoddelete_returns_ok() {
        let request = b"DELETE some stuff";
        let mut request = RawIter::new(request);
        let method = parse_method(&mut request);
        assert_eq!(true, method.is_ok());

        let expected = Method::Delete;
        let actual = method.unwrap();
        assert_eq!(expected, actual);

        // Make sure the iterator was advanced properly
        assert_eq!(b"DELETE".len(), request.index());
    }

    #[test]
    fn parse_methodconnect_returns_ok() {
        let request = b"CONNECT some stuff";
        let mut request = RawIter::new(request);
        let method = parse_method(&mut request);
        assert_eq!(true, method.is_ok());

        let expected = Method::Connect;
        let actual = method.unwrap();
        assert_eq!(expected, actual);

        // Make sure the iterator was advanced properly
        assert_eq!(b"CONNECT".len(), request.index());
    }

    #[test]
    fn parse_methodoptions_returns_ok() {
        let request = b"OPTIONS some stuff";
        let mut request = RawIter::new(request);
        let method = parse_method(&mut request);
        assert_eq!(true, method.is_ok());

        let expected = Method::Options;
        let actual = method.unwrap();
        assert_eq!(expected, actual);

        // Make sure the iterator was advanced properly
        assert_eq!(b"OPTIONS".len(), request.index());
    }

    #[test]
    fn parse_methodtrace_returns_ok() {
        let request = b"TRACE some stuff";
        let mut request = RawIter::new(request);
        let method = parse_method(&mut request);
        assert_eq!(true, method.is_ok());

        let expected = Method::Trace;
        let actual = method.unwrap();
        assert_eq!(expected, actual);

        // Make sure the iterator was advanced properly
        assert_eq!(b"TRACE".len(), request.index());
    }

    #[test]
    fn parse_methodpatch_returns_ok() {
        let request = b"PATCH some stuff";
        let mut request = RawIter::new(request);
        let method = parse_method(&mut request);
        assert_eq!(true, method.is_ok());

        let expected = Method::Patch;
        let actual = method.unwrap();
        assert_eq!(expected, actual);

        // Make sure the iterator was advanced properly
        assert_eq!(b"PATCH".len(), request.index());
    }

    #[test]
    fn parse_methodunable_to_parse_returns_err() {
        let request = b"some stuff";
        let mut request = RawIter::new(request);
        let method = parse_method(&mut request);
        assert_eq!(true, method.is_err());

        let expected = Error::HttpMethodParseFailed;
        let actual = method.unwrap_err();
        assert_eq!(expected, actual);

        // Make sure the iterator was advanced properly
        assert_eq!(0, request.index());
    }
}
