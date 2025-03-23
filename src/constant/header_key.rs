/// The HTTP header field name `server`.
pub static SERVER: &str = "server";

/// The HTTP header field name `accept`.
pub static ACCEPT: &str = "accept";

/// The HTTP header field name `accept-encoding`, used to specify acceptable content encodings.
pub static ACCEPT_ENCODING: &str = "accept-encoding";

/// The HTTP header field name `accept-language`, used to specify preferred languages for the response.
pub static ACCEPT_LANGUAGE: &str = "accept-language";

/// The HTTP header field name `authorization`, used to specify authentication credentials.
pub static AUTHORIZATION: &str = "authorization";

/// The HTTP header field name `cache-control`, used to specify caching directives.
pub static CACHE_CONTROL: &str = "cache-control";

/// The HTTP header field name `connection`, used to specify control options for the current connection.
pub static CONNECTION: &str = "connection";

/// The HTTP header field name `cookie`, used to send cookies from the server to the client.
pub static COOKIE: &str = "cookie";

/// The HTTP header field name `date`, used to specify the date and time at which the message was sent.
pub static DATE: &str = "date";

/// The HTTP header field name `etag`, used to specify a unique identifier for a resource version.
pub static ETAG: &str = "etag";

/// The HTTP header field name `host`, used to specify the host and port number of the server.
pub static HOST: &str = "host";

/// The HTTP header field name `last-modified`, used to specify the last modification date of the resource.
pub static LAST_MODIFIED: &str = "last-modified";

/// The HTTP header field name `location`, used to specify the url to redirect a client.
pub static LOCATION: &str = "location";

/// The HTTP header field name `referer`, used to specify the url of the referring resource.
pub static REFERER: &str = "referer";

/// The HTTP header field name `set-cookie`, used to send cookies from the server to the client.
pub static SET_COOKIE: &str = "set-cookie";

/// The HTTP header field name `transfer-encoding`, used to specify the form of encoding used to safely transfer the entity to the user.
pub static TRANSFER_ENCODING: &str = "transfer-encoding";

/// The HTTP header field name `upgrade`, used to indicate the protocol the client wants to upgrade to.
pub static UPGRADE: &str = "upgrade";

/// The HTTP header field name `sec-websocket-accept`.
pub static SEC_WEB_SOCKET_ACCEPT: &str = "sec-websocket-accept";

/// The HTTP header field name `sec-websocket-key`.
pub static SEC_WEBSOCKET_KEY: &str = "sec-websocket-key";

/// The HTTP header field name `vary`, used to specify that the response may vary based on certain request headers.
pub static VARY: &str = "vary";

/// The HTTP header field name `x-frame-options`, used to specify whether a browser should be allowed to render a page in a `<frame>`.
pub static X_FRAME_OPTIONS: &str = "x-frame-options";

/// The HTTP header field name `x-content-type-options`, used to indicate that the browser should not sniff the mime type of a response.
pub static X_CONTENT_TYPE_OPTIONS: &str = "x-content-type-options";

/// The HTTP header field name `x-powered-by`, used to indicate the technology used by the server.
pub static X_POWERED_BY: &str = "x-powered-by";

/// The HTTP header field name `x-requested-with`, used to identify the type of request, typically for ajax requests.
pub static X_REQUESTED_WITH: &str = "x-requested-with";

/// The HTTP header field name `content-length`.
pub static CONTENT_LENGTH: &str = "content-length";

/// The HTTP header field name `content-encoding`, used to specify the encoding transformations applied to the response body.
pub static CONTENT_ENCODING: &str = "content-encoding";

/// The HTTP header field name content-type, used to specify the media type of the resource or the data being sent in an http request or response.
pub static CONTENT_TYPE: &str = "content-type";

/// The HTTP header field `user-agent`.
pub static USER_AGENT: &str = "user-agent";

/// The HTTP header field `access-control-allow-origin`, used to specify which origins are allowed to access the resource.
pub static ACCESS_CONTROL_ALLOW_ORIGIN: &str = "access-control-allow-origin";

/// The HTTP header field `access-control-allow-methods`, used to specify The HTTP methods that are allowed when accessing the resource.
pub static ACCESS_CONTROL_ALLOW_METHODS: &str = "access-control-allow-methods";

/// The HTTP header field `access-control-allow-headers`, used to specify which http headers can be used during the request.
pub static ACCESS_CONTROL_ALLOW_HEADERS: &str = "access-control-allow-headers";
