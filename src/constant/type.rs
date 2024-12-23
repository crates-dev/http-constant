/// A carriage return and newline character sequence (`\r\n`), used to separate lines in HTTP headers.
pub static HTTP_BR: &str = "\r\n";

/// A static reference to a byte slice representing the HTTP line break (`\r\n`).
pub static HTTP_BR_BYTES: &[u8] = HTTP_BR.as_bytes();

/// A double carriage return and newline character sequence (`\r\n\r\n`), used to separate HTTP headers from the body.
pub static HTTP_DOUBLE_BR: &str = "\r\n\r\n";

/// A static reference to a byte slice representing the HTTP double line break (`\r\n\r\n`).
pub static HTTP_DOUBLE_BR_BYTES: &[u8] = HTTP_DOUBLE_BR.as_bytes();

/// The HTTP header field name `Location`, used to specify the URL to redirect a client.
pub static LOCATION: &str = "Location";

/// The HTTP header field name `Content-Length`, used to specify the length of the response body in bytes.
pub static CONTENT_LENGTH: &str = "Content-Length";

/// The HTTP header field name `Content-Type`, used to specify the media type of the resource or the data being sent in an HTTP request or response.
pub static CONTENT_TYPE: &str = "Content-Type";

/// The HTTP header name used to indicate the content encoding of the response.
pub static CONTENT_ENCODING: &str = "Content-Encoding";

/// The HTTP header field "Accept".
pub static ACCEPT: &str = "Accept";

/// The default value for the `Accept` header.
pub static ACCEPT_VALUE: &str = "*/*";

/// The HTTP header field "User-Agent".
pub static USER_AGENT: &str = "User-Agent";

/// The HTTP header field name `Host`, used to specify the host and port number of the server.
pub static HOST: &str = "Host";

/// Unknown HTTP version
pub static UNKNOWN_HTTP_VERSION: &str = "";

/// The default HTTP version `HTTP/1.1` used in requests and responses.
pub static HTTP_VERSION_1_1: &str = "HTTP/1.1";

/// The default HTTP version `HTTP/2` used in requests and responses.
pub static HTTP_VERSION_2: &str = "HTTP/2";

/// The default HTTP path (`/`), typically used in requests when no specific path is provided.
pub static DEFAULT_HTTP_PATH: &str = "/";

/// The MIME type for JSON content, typically used for requests and responses
/// containing JSON data.
pub static APPLICATION_JSON: &str = "application/json";

/// The MIME type for XML content, typically used for requests and responses
/// containing XML data.
pub static APPLICATION_XML: &str = "application/xml";

/// The MIME type for plain text content, typically used for requests and responses
/// containing simple text data.
pub static TEXT_PLAIN: &str = "text/plain";

/// The MIME type for HTML content, typically used for requests and responses
/// containing HTML data.
pub static TEXT_HTML: &str = "text/html";

/// The MIME type for form-encoded data, commonly used for sending data in the
/// body of HTTP requests, especially for form submissions.
pub static FORM_URLENCODED: &str = "application/x-www-form-urlencoded";

/// Query symbols
pub static QUERY_SYMBOL: &str = "?";

/// Hash symbols
pub static HASH_SYMBOL: &str = "#";

/// A constant representing the "HTTP" protocol.
pub static HTTP: &str = "HTTP";

/// A constant representing the "HTTPS" protocol.
pub static HTTPS: &str = "HTTPS";

/// The name of the application.
///
/// This constant represents the name of the application used for
/// identifying the current application context.
pub static APP_NAME: &str = "http-request";

/// A single space character.
///
/// This constant is used to represent a space character in string
/// or byte operations.
pub static SPACE: &str = " ";

/// The byte representation of a single space character.
///
/// This constant provides the byte equivalent of the space character
/// for use in low-level operations.
pub static SPACE_U8: u8 = SPACE.as_bytes()[0];

/// A tab character.
///
/// This constant is used to represent a tab character in string
/// or byte operations.
pub static TAB: &str = "\t";

/// The byte representation of a tab character.
///
/// This constant provides the byte equivalent of the tab character
/// for use in low-level operations.
pub static TAB_U8: u8 = TAB.as_bytes()[0];

/// A line break character (newline).
///
/// This constant is used to represent a line break character in
/// string or byte operations.
pub static BR: &str = "\n";

/// A static byte slice representation of the string `BR`.
pub static BR_BYTES: &[u8] = BR.as_bytes();

/// A colon followed by a space (`: `).
///
/// This constant is commonly used in formatted strings, such as
/// headers or key-value pairs, where a colon and a space are needed.
pub static COLON_SPACE: &str = ": ";

/// The byte representation of the first character in the `COLON_SPACE`.
///
/// This constant provides the byte equivalent of the colon character
/// from the `COLON_SPACE` string.
pub static COLON_SPACE_BYTES: &[u8] = COLON_SPACE.as_bytes();

/// GET
pub static GET: &str = "GET";

/// POST
pub static POST: &str = "POST";

/// Empty str
pub static EMPTY_STR: &str = "";
