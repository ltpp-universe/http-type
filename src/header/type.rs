use crate::*;

/// A type alias for a `HashMap<String, String>`, representing the headers of an HTTP request or response.
/// This structure stores key-value pairs, where each key is the name of an HTTP header (e.g., `Content-Type`, `Host`),
/// and the value is the corresponding header value (e.g., `application/json`, `example.com`).
pub type HttpHeaderMap = HashMap<String, String>;

/// A type alias for a `HashMap<&str, &str>`, representing the headers of an HTTP request or response.
/// This structure stores key-value pairs, where each key is the name of an HTTP header (e.g., `Content-Type`, `Host`),
/// and the value is the corresponding header value (e.g., `application/json`, `example.com`).
pub type HttpHeaderSliceMap = HashMap<String, String>;
