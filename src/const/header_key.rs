/// The HTTP header field name `server`.
pub const SERVER: &str = "server";

/// The HTTP header field name `accept`.
pub const ACCEPT: &str = "accept";

/// The HTTP header field name `accept-encoding`, used to specify acceptable content encodings.
pub const ACCEPT_ENCODING: &str = "accept-encoding";

/// The HTTP header field name `accept-language`, used to specify preferred languages for the response.
pub const ACCEPT_LANGUAGE: &str = "accept-language";

/// The HTTP header field name `authorization`, used to specify authentication credentials.
pub const AUTHORIZATION: &str = "authorization";

/// The HTTP header field name `cache-control`, used to specify caching directives.
pub const CACHE_CONTROL: &str = "cache-control";

/// The HTTP header field name `connection`, used to specify control options for the current connection.
pub const CONNECTION: &str = "connection";

/// The HTTP header field name `cookie`, used to send cookies from the server to the client.
pub const COOKIE: &str = "cookie";

/// The HTTP header field name `date`, used to specify the date and time at which the message was sent.
pub const DATE: &str = "date";

/// The HTTP header field name `etag`, used to specify a unique identifier for a resource version.
pub const ETAG: &str = "etag";

/// The HTTP header field name `host`, used to specify the host and port number of the server.
pub const HOST: &str = "host";

/// The HTTP header field name `last-modified`, used to specify the last modification date of the resource.
pub const LAST_MODIFIED: &str = "last-modified";

/// The HTTP header field name `location`, used to specify the url to redirect a client.
pub const LOCATION: &str = "location";

/// The HTTP header field name `referer`, used to specify the url of the referring resource.
pub const REFERER: &str = "referer";

/// The HTTP header field name `set-cookie`, used to send cookies from the server to the client.
pub const SET_COOKIE: &str = "set-cookie";

/// The HTTP header field name `transfer-encoding`, used to specify the form of encoding used to safely transfer the entity to the user.
pub const TRANSFER_ENCODING: &str = "transfer-encoding";

/// The HTTP header field name `upgrade`, used to indicate the protocol the client wants to upgrade to.
pub const UPGRADE: &str = "upgrade";

/// The HTTP header field name `sec-websocket-accept`.
pub const SEC_WEB_SOCKET_ACCEPT: &str = "sec-websocket-accept";

/// The HTTP header field name `sec-websocket-key`.
pub const SEC_WEBSOCKET_KEY: &str = "sec-websocket-key";

/// The HTTP header field name `vary`, used to specify that the response may vary based on certain request headers.
pub const VARY: &str = "vary";

/// The HTTP header field name `x-frame-options`, used to specify whether a browser should be allowed to render a page in a `<frame>`.
pub const X_FRAME_OPTIONS: &str = "x-frame-options";

/// The HTTP header field name `x-content-type-options`, used to indicate that the browser should not sniff the mime type of a response.
pub const X_CONTENT_TYPE_OPTIONS: &str = "x-content-type-options";

/// The HTTP header field name `x-powered-by`, used to indicate the technology used by the server.
pub const X_POWERED_BY: &str = "x-powered-by";

/// The HTTP header field name `x-requested-with`, used to identify the type of request, typically for ajax requests.
pub const X_REQUESTED_WITH: &str = "x-requested-with";

/// The HTTP header field name `content-length`.
pub const CONTENT_LENGTH: &str = "content-length";

/// The HTTP header field name `content-encoding`, used to specify the encoding transformations applied to the response body.
pub const CONTENT_ENCODING: &str = "content-encoding";

/// The HTTP header field name content-type, used to specify the media type of the resource or the data being sent in an http request or response.
pub const CONTENT_TYPE: &str = "content-type";

/// The HTTP header field `user-agent`.
pub const USER_AGENT: &str = "user-agent";

/// The HTTP header field `access-control-allow-origin`, used to specify which origins are allowed to access the resource.
pub const ACCESS_CONTROL_ALLOW_ORIGIN: &str = "access-control-allow-origin";

/// The HTTP header field `access-control-allow-methods`, used to specify The HTTP methods that are allowed when accessing the resource.
pub const ACCESS_CONTROL_ALLOW_METHODS: &str = "access-control-allow-methods";

/// The HTTP header field `access-control-allow-headers`, used to specify which http headers can be used during the request.
pub const ACCESS_CONTROL_ALLOW_HEADERS: &str = "access-control-allow-headers";

/// Expires
pub const EXPIRES: &str = "expires";

/// The HTTP header field `if-match`, used to make a request conditional based on etag values.
pub const IF_MATCH: &str = "if-match";

/// The HTTP header field `if-none-match`, used to make a request conditional based on etag values.
pub const IF_NONE_MATCH: &str = "if-none-match";

/// The HTTP header field `if-modified-since`, used to make a request conditional based on timestamps.
pub const IF_MODIFIED_SINCE: &str = "if-modified-since";

/// The HTTP header field `if-unmodified-since`, used to make a request conditional based on timestamps.
pub const IF_UNMODIFIED_SINCE: &str = "if-unmodified-since";

/// The HTTP header field `accept-charset`, used to specify which character sets are acceptable.
pub const ACCEPT_CHARSET: &str = "accept-charset";

/// The HTTP header field `access-control-max-age`, used to indicate how long the results of a preflight request can be cached.
pub const ACCESS_CONTROL_MAX_AGE: &str = "access-control-max-age";

/// The HTTP header field `access-control-expose-headers`, used to indicate which headers can be exposed as part of the response.
pub const ACCESS_CONTROL_EXPOSE_HEADERS: &str = "access-control-expose-headers";

/// The HTTP header field `access-control-request-headers`, used in preflight requests to indicate which headers will be used.
pub const ACCESS_CONTROL_REQUEST_HEADERS: &str = "access-control-request-headers";

/// The HTTP header field `access-control-request-method`, used in preflight requests to indicate which HTTP method will be used.
pub const ACCESS_CONTROL_REQUEST_METHOD: &str = "access-control-request-method";

/// The HTTP header field `allow`, used to specify supported HTTP methods.
pub const ALLOW: &str = "allow";

/// The HTTP header field `content-disposition`, used to indicate how the content should be displayed.
pub const CONTENT_DISPOSITION: &str = "content-disposition";

/// The HTTP header field `content-language`, used to specify the language of the content.
pub const CONTENT_LANGUAGE: &str = "content-language";

/// The HTTP header field `content-range`, used to indicate where in the full resource this partial message belongs.
pub const CONTENT_RANGE: &str = "content-range";

/// The HTTP header field `origin`, used to indicate where the cross-origin request originates from.
pub const ORIGIN: &str = "origin";

/// The HTTP header field `pragma`, used to include implementation-specific directives.
pub const PRAGMA: &str = "pragma";

/// The HTTP header field `proxy-authenticate`, used in responses from a proxy to indicate authentication is required.
pub const PROXY_AUTHENTICATE: &str = "proxy-authenticate";

/// The HTTP header field `proxy-authorization`, used to authenticate with a proxy server.
pub const PROXY_AUTHORIZATION: &str = "proxy-authorization";

/// The HTTP header field `retry-after`, used to indicate how long to wait before making a new request.
pub const RETRY_AFTER: &str = "retry-after";

/// The HTTP header field `strict-transport-security`, used to specify that the browser should only connect using HTTPS.
pub const STRICT_TRANSPORT_SECURITY: &str = "strict-transport-security";

/// The HTTP header field `www-authenticate`, used to indicate the authentication scheme.
pub const WWW_AUTHENTICATE: &str = "www-authenticate";

/// The HTTP/2 pseudo-header field `:authority`, used to specify the authority portion of the target URI.
pub const AUTHORITY: &str = ":authority";

/// The HTTP/2 pseudo-header field `:method`, used to specify the HTTP method.
pub const METHOD: &str = ":method";

/// The HTTP/2 pseudo-header field `:path`, used to specify the path and query parts of the target URI.
pub const PATH: &str = ":path";

/// The HTTP/2 pseudo-header field `:scheme`, used to specify the scheme portion of the target URI.
pub const SCHEME: &str = ":scheme";

/// The HTTP header field `priority`, used to indicate the priority of the request.
pub const PRIORITY: &str = "priority";

/// The HTTP header field `sec-ch-ua`, used to indicate the user agent's brand and version.
pub const SEC_CH_UA: &str = "sec-ch-ua";

/// The HTTP header field `sec-ch-ua-mobile`, used to indicate whether the user agent is running on a mobile device.
pub const SEC_CH_UA_MOBILE: &str = "sec-ch-ua-mobile";

/// The HTTP header field `sec-ch-ua-platform`, used to indicate the platform the user agent is running on.
pub const SEC_CH_UA_PLATFORM: &str = "sec-ch-ua-platform";

/// The HTTP header field `sec-fetch-dest`, used to indicate the destination of the request.
pub const SEC_FETCH_DEST: &str = "sec-fetch-dest";

/// The HTTP header field `sec-fetch-mode`, used to indicate the mode of the request.
pub const SEC_FETCH_MODE: &str = "sec-fetch-mode";

/// The HTTP header field `sec-fetch-site`, used to indicate the relationship between the request initiator's origin and the origin of the requested resource.
pub const SEC_FETCH_SITE: &str = "sec-fetch-site";

/// The HTTP header field `age`, used to indicate the age of the response in seconds.
pub const AGE: &str = "age";

/// The HTTP header field `alt-svc`, used to advertise alternative services through which the same resource can be reached.
pub const ALT_SVC: &str = "alt-svc";

/// The HTTP header field `expect`, used to indicate expectations that need to be fulfilled by the server.
pub const EXPECT: &str = "expect";

/// The HTTP header field `forwarded`, used to disclose original information of a client connecting to a web server through an HTTP proxy.
pub const FORWARDED: &str = "forwarded";

/// The HTTP header field `from`, used to specify an Internet email address for a human user.
pub const FROM: &str = "from";

/// The HTTP header field `link`, used to specify relationships between resources.
pub const LINK: &str = "link";

/// The HTTP header field `max-forwards`, used to limit the number of times a message can be forwarded through proxies or gateways.
pub const MAX_FORWARDS: &str = "max-forwards";

/// The HTTP header field `range`, used to specify the range of a resource to be retrieved.
pub const RANGE: &str = "range";

/// The HTTP header field `te`, used to specify which transfer encodings the user agent is willing to accept.
pub const TE: &str = "te";

/// The HTTP header field `via`, used to indicate the intermediate protocols and recipients between the user agent and the server.
pub const VIA: &str = "via";

/// The HTTP header field `dnt`, used to specify the user's tracking preference.
pub const DNT: &str = "dnt";

/// The HTTP header field `sec-fetch-user`, used to indicate whether the request was triggered by a user activation.
pub const SEC_FETCH_USER: &str = "sec-fetch-user";
