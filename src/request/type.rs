use crate::*;

/// HTTP request method.
pub type RequestMethod = Method;
/// The host part of an HTTP request.
pub type RequestHost = String;
/// The HTTP version (e.g., HTTP/1.1).
pub type RequestVersion = HttpVersion;
/// The path portion of the request URL.
pub type RequestPath = String;
/// Key type used in the request query parameters.
pub type RequestQuerysKey = String;
/// Value type used in the request query parameters.
pub type RequestQuerysValue = String;
/// All query parameters parsed from the request URL.
pub type RequestQuerys = HashMapXxHash3_64<RequestQuerysKey, RequestQuerysValue>;
/// The raw binary body of the request.
pub type RequestBody = Vec<u8>;
/// The request body as a UTF-8 string.
pub type RequestBodyString = String;
/// Key type used in the request headers.
pub type RequestHeadersKey = String;
/// Value type used in the request headers.
pub type RequestHeadersValue = String;
/// All headers sent with the HTTP request.
pub type RequestHeaders = HashMapXxHash3_64<RequestHeadersKey, RequestHeadersValue>;
/// The result type returned from a request reader handler.
pub type RequestReaderHandleResult = Result<Request, RequestError>;
/// Read guard for a `Request` wrapped in a `RwLock`.
pub type RwLockReadGuardRequest<'a> = RwLockReadGuard<'a, Request>;
/// Write guard for a `Request` wrapped in a `RwLock`.
pub type RwLockWriteGuardRequest<'a> = RwLockWriteGuard<'a, Request>;
/// Optional value for a query parameter.
pub type OptionRequestQuerysValue = Option<RequestQuerysValue>;
/// Optional value for a header.
pub type OptionRequestHeadersValue = Option<RequestHeadersValue>;
