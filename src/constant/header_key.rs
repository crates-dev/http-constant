/// The HTTP header field name `Accept`.
pub static ACCEPT: &str = "Accept";

/// The HTTP header field name `Accept-Encoding`, used to specify acceptable content encodings.
pub static ACCEPT_ENCODING: &str = "Accept-Encoding";

/// The HTTP header field name `Accept-Language`, used to specify preferred languages for the response.
pub static ACCEPT_LANGUAGE: &str = "Accept-Language";

/// The HTTP header field name `Authorization`, used to specify authentication credentials.
pub static AUTHORIZATION: &str = "Authorization";

/// The HTTP header field name `Cache-Control`, used to specify caching directives.
pub static CACHE_CONTROL: &str = "Cache-Control";

/// The HTTP header field name `Connection`, used to specify control options for the current connection.
pub static CONNECTION: &str = "Connection";

/// The HTTP header field name `Cookie`, used to send cookies from the server to the client.
pub static COOKIE: &str = "Cookie";

/// The HTTP header field name `Date`, used to specify the date and time at which the message was sent.
pub static DATE: &str = "Date";

/// The HTTP header field name `ETag`, used to specify a unique identifier for a resource version.
pub static ETAG: &str = "ETag";

/// The HTTP header field name `Host`, used to specify the host and port number of the server.
pub static HOST: &str = "Host";

/// The HTTP header field name `Last-Modified`, used to specify the last modification date of the resource.
pub static LAST_MODIFIED: &str = "Last-Modified";

/// The HTTP header field name `Location`, used to specify the URL to redirect a client.
pub static LOCATION: &str = "Location";

/// The HTTP header field name `Referer`, used to specify the URL of the referring resource.
pub static REFERER: &str = "Referer";

/// The HTTP header field name `Set-Cookie`, used to send cookies from the server to the client.
pub static SET_COOKIE: &str = "Set-Cookie";

/// The HTTP header field name `Transfer-Encoding`, used to specify the form of encoding used to safely transfer the entity to the user.
pub static TRANSFER_ENCODING: &str = "Transfer-Encoding";

/// The HTTP header field name `Upgrade`, used to indicate the protocol the client wants to upgrade to.
pub static UPGRADE: &str = "Upgrade";

/// The HTTP header field name `Vary`, used to specify that the response may vary based on certain request headers.
pub static VARY: &str = "Vary";

/// The HTTP header field name `X-Frame-Options`, used to specify whether a browser should be allowed to render a page in a `<frame>`.
pub static X_FRAME_OPTIONS: &str = "X-Frame-Options";

/// The HTTP header field name `X-Content-Type-Options`, used to indicate that the browser should not sniff the MIME type of a response.
pub static X_CONTENT_TYPE_OPTIONS: &str = "X-Content-Type-Options";

/// The HTTP header field name `X-Powered-By`, used to indicate the technology used by the server.
pub static X_POWERED_BY: &str = "X-Powered-By";

/// The HTTP header field name `X-Requested-With`, used to identify the type of request, typically for AJAX requests.
pub static X_REQUESTED_WITH: &str = "X-Requested-With";

/// The value for the `Content-Length` header, used to specify the size of the request body in octets (8-bit bytes).
pub static CONTENT_LENGTH: &str = "Content-Length";
