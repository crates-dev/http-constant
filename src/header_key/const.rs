/// The HTTP header field name `server`.
pub const SERVER: &'static str = "server";

/// The HTTP header field name `accept`.
pub const ACCEPT: &'static str = "accept";

/// The HTTP header field name `accept-encoding`, used to specify acceptable content encodings.
pub const ACCEPT_ENCODING: &'static str = "accept-encoding";

/// The HTTP header field name `accept-language`, used to specify preferred languages for the response.
pub const ACCEPT_LANGUAGE: &'static str = "accept-language";

/// The HTTP header field name `authorization`, used to specify authentication credentials.
pub const AUTHORIZATION: &'static str = "authorization";

/// The HTTP header field name `cache-control`, used to specify caching directives.
pub const CACHE_CONTROL: &'static str = "cache-control";

/// The HTTP header field name `connection`, used to specify control options for the current connection.
pub const CONNECTION: &'static str = "connection";

/// The HTTP header field name `cookie`, used to send cookies from the server to the client.
pub const COOKIE: &'static str = "cookie";

/// The HTTP header field name `date`, used to specify the date and time at which the message was sent.
pub const DATE: &'static str = "date";

/// The HTTP header field name `etag`, used to specify a unique identifier for a resource version.
pub const ETAG: &'static str = "etag";

/// The HTTP header field name `host`, used to specify the host and port number of the server.
pub const HOST: &'static str = "host";

/// The HTTP header field name `last-modified`, used to specify the last modification date of the resource.
pub const LAST_MODIFIED: &'static str = "last-modified";

/// The HTTP header field name `location`, used to specify the url to redirect a client.
pub const LOCATION: &'static str = "location";

/// The HTTP header field name `referer`, used to specify the url of the referring resource.
pub const REFERER: &'static str = "referer";

/// The HTTP header field name `set-cookie`, used to send cookies from the server to the client.
pub const SET_COOKIE: &'static str = "set-cookie";

/// The HTTP header field name `transfer-encoding`, used to specify the form of encoding used to safely transfer the entity to the user.
pub const TRANSFER_ENCODING: &'static str = "transfer-encoding";

/// The HTTP header field name `upgrade`, used to indicate the protocol the client wants to upgrade to.
pub const UPGRADE: &'static str = "upgrade";

/// The HTTP header field name `sec-websocket-accept`.
pub const SEC_WEBSOCKET_ACCEPT: &'static str = "sec-websocket-accept";

/// The HTTP header field name `sec-websocket-key`.
pub const SEC_WEBSOCKET_KEY: &'static str = "sec-websocket-key";

/// The HTTP header field name `sec-websocket-protocol`.
pub const SEC_WEBSOCKET_VERSION: &'static str = "sec-websocket-version";

/// The HTTP header field name `sec-websocket-protocol`.
pub const SEC_WEBSOCKET_PROTOCOL: &'static str = "sec-websocket-protocol";

/// The HTTP header field name `sec-websocket-extensions`.
pub const SEC_WEBSOCKET_EXTENSIONS: &'static str = "sec-websocket-extensions";

/// The HTTP header field name `vary`, used to specify that the response may vary based on certain request headers.
pub const VARY: &'static str = "vary";

/// The HTTP header field name `x-frame-options`, used to specify whether a browser should be allowed to render a page in a `<frame>`.
pub const X_FRAME_OPTIONS: &'static str = "x-frame-options";

/// The HTTP header field name `x-content-type-options`, used to indicate that the browser should not sniff the mime type of a response.
pub const X_CONTENT_TYPE_OPTIONS: &'static str = "x-content-type-options";

/// The HTTP header field name `x-powered-by`, used to indicate the technology used by the server.
pub const X_POWERED_BY: &'static str = "x-powered-by";

/// The HTTP header field name `x-requested-with`, used to identify the type of request, typically for ajax requests.
pub const X_REQUESTED_WITH: &'static str = "x-requested-with";

/// The HTTP header field name `content-length`.
pub const CONTENT_LENGTH: &'static str = "content-length";

/// The HTTP header field name `content-encoding`, used to specify the encoding transformations applied to the response body.
pub const CONTENT_ENCODING: &'static str = "content-encoding";

/// The HTTP header field name content-type, used to specify the media type of the resource or the data being sent in an http request or response.
pub const CONTENT_TYPE: &'static str = "content-type";

/// The HTTP header field `user-agent`.
pub const USER_AGENT: &'static str = "user-agent";

/// The HTTP header field `access-control-allow-origin`, used to specify which origins are allowed to access the resource.
pub const ACCESS_CONTROL_ALLOW_ORIGIN: &'static str = "access-control-allow-origin";

/// The HTTP header field `access-control-allow-methods`, used to specify The HTTP methods that are allowed when accessing the resource.
pub const ACCESS_CONTROL_ALLOW_METHODS: &'static str = "access-control-allow-methods";

/// The HTTP header field `access-control-allow-headers`, used to specify which http headers can be used during the request.
pub const ACCESS_CONTROL_ALLOW_HEADERS: &'static str = "access-control-allow-headers";

/// The HTTP header field `expires`, used to specify the date/time after which the response is considered stale.
pub const EXPIRES: &'static str = "expires";

/// The HTTP header field `if-match`, used to make a request conditional based on etag values.
pub const IF_MATCH: &'static str = "if-match";

/// The HTTP header field `if-none-match`, used to make a request conditional based on etag values.
pub const IF_NONE_MATCH: &'static str = "if-none-match";

/// The HTTP header field `if-modified-since`, used to make a request conditional based on timestamps.
pub const IF_MODIFIED_SINCE: &'static str = "if-modified-since";

/// The HTTP header field `if-unmodified-since`, used to make a request conditional based on timestamps.
pub const IF_UNMODIFIED_SINCE: &'static str = "if-unmodified-since";

/// The HTTP header field `accept-charset`, used to specify which character sets are acceptable.
pub const ACCEPT_CHARSET: &'static str = "accept-charset";

/// The HTTP header field `access-control-max-age`, used to indicate how long the results of a preflight request can be cached.
pub const ACCESS_CONTROL_MAX_AGE: &'static str = "access-control-max-age";

/// The HTTP header field `access-control-expose-headers`, used to indicate which headers can be exposed as part of the response.
pub const ACCESS_CONTROL_EXPOSE_HEADERS: &'static str = "access-control-expose-headers";

/// The HTTP header field `access-control-request-headers`, used in preflight requests to indicate which headers will be used.
pub const ACCESS_CONTROL_REQUEST_HEADERS: &'static str = "access-control-request-headers";

/// The HTTP header field `access-control-request-method`, used in preflight requests to indicate which HTTP method will be used.
pub const ACCESS_CONTROL_REQUEST_METHOD: &'static str = "access-control-request-method";

/// The HTTP header field `allow`, used to specify supported HTTP methods.
pub const ALLOW: &'static str = "allow";

/// The HTTP header field `content-disposition`, used to indicate how the content should be displayed.
pub const CONTENT_DISPOSITION: &'static str = "content-disposition";

/// The HTTP header field `content-language`, used to specify the language of the content.
pub const CONTENT_LANGUAGE: &'static str = "content-language";

/// The HTTP header field `content-range`, used to indicate where in the full resource this partial message belongs.
pub const CONTENT_RANGE: &'static str = "content-range";

/// The HTTP header field `origin`, used to indicate where the cross-origin request originates from.
pub const ORIGIN: &'static str = "origin";

/// The HTTP header field `pragma`, used to include implementation-specific directives.
pub const PRAGMA: &'static str = "pragma";

/// The HTTP header field `proxy-authenticate`, used in responses from a proxy to indicate authentication is required.
pub const PROXY_AUTHENTICATE: &'static str = "proxy-authenticate";

/// The HTTP header field `proxy-authorization`, used to authenticate with a proxy server.
pub const PROXY_AUTHORIZATION: &'static str = "proxy-authorization";

/// The HTTP header field `retry-after`, used to indicate how long to wait before making a new request.
pub const RETRY_AFTER: &'static str = "retry-after";

/// The HTTP header field `strict-transport-security`, used to specify that the browser should only connect using HTTPS.
pub const STRICT_TRANSPORT_SECURITY: &'static str = "strict-transport-security";

/// The HTTP header field `www-authenticate`, used to indicate the authentication scheme.
pub const WWW_AUTHENTICATE: &'static str = "www-authenticate";

/// The HTTP/2 pseudo-header field `:authority`, used to specify the authority portion of the target URI.
pub const COLON_AUTHORITY: &'static str = ":authority";

/// The HTTP/2 pseudo-header field `:method`, used to specify the HTTP method.
pub const COLON_METHOD: &'static str = ":method";

/// The HTTP/2 pseudo-header field `:path`, used to specify the path and query parts of the target URI.
pub const COLON_PATH: &'static str = ":path";

/// The HTTP/2 pseudo-header field `:scheme`, used to specify the scheme portion of the target URI.
pub const COLON_SCHEME: &'static str = ":scheme";

/// The HTTP header field `priority`, used to indicate the priority of the request.
pub const PRIORITY: &'static str = "priority";

/// The HTTP header field `sec-ch-ua`, used to indicate the user agent's brand and version.
pub const SEC_CH_UA: &'static str = "sec-ch-ua";

/// The HTTP header field `sec-ch-ua-mobile`, used to indicate whether the user agent is running on a mobile device.
pub const SEC_CH_UA_MOBILE: &'static str = "sec-ch-ua-mobile";

/// The HTTP header field `sec-ch-ua-platform`, used to indicate the platform the user agent is running on.
pub const SEC_CH_UA_PLATFORM: &'static str = "sec-ch-ua-platform";

/// The HTTP header field `sec-fetch-dest`, used to indicate the destination of the request.
pub const SEC_FETCH_DEST: &'static str = "sec-fetch-dest";

/// The HTTP header field `sec-fetch-mode`, used to indicate the mode of the request.
pub const SEC_FETCH_MODE: &'static str = "sec-fetch-mode";

/// The HTTP header field `sec-fetch-site`, used to indicate the relationship between the request initiator's origin and the origin of the requested resource.
pub const SEC_FETCH_SITE: &'static str = "sec-fetch-site";

/// The HTTP header field `age`, used to indicate the age of the response in seconds.
pub const AGE: &'static str = "age";

/// The HTTP header field `alt-svc`, used to advertise alternative services through which the same resource can be reached.
pub const ALT_SVC: &'static str = "alt-svc";

/// The HTTP header field `expect`, used to indicate expectations that need to be fulfilled by the server.
pub const EXPECT: &'static str = "expect";

/// The HTTP header field `forwarded`, used to disclose original information of a client connecting to a web server through an HTTP proxy.
pub const FORWARDED: &'static str = "forwarded";

/// The HTTP header field `from`, used to specify an Internet email address for a human user.
pub const FROM: &'static str = "from";

/// The HTTP header field `link`, used to specify relationships between resources.
pub const LINK: &'static str = "link";

/// The HTTP header field `max-forwards`, used to limit the number of times a message can be forwarded through proxies or gateways.
pub const MAX_FORWARDS: &'static str = "max-forwards";

/// The HTTP header field `range`, used to specify the range of a resource to be retrieved.
pub const RANGE: &'static str = "range";

/// The HTTP header field `te`, used to specify which transfer encodings the user agent is willing to accept.
pub const TE: &'static str = "te";

/// The HTTP header field `via`, used to indicate the intermediate protocols and recipients between the user agent and the server.
pub const VIA: &'static str = "via";

/// The HTTP header field `dnt`, used to specify the user's tracking preference.
pub const DNT: &'static str = "dnt";

/// The HTTP header field `sec-fetch-user`, used to indicate whether the request was triggered by a user activation.
pub const SEC_FETCH_USER: &'static str = "sec-fetch-user";

/// The HTTP header field `accept-ranges`, used to specify the range units that can be accepted.
pub const ACCEPT_RANGES: &'static str = "accept-ranges";

/// The HTTP header field `content-md5`, used to specify the MD5 checksum of the entity-body.
pub const CONTENT_MD5: &'static str = "content-md5";

/// The HTTP header field `content-location`, used to specify the location of the resource.
pub const CONTENT_LOCATION: &'static str = "content-location";

/// The HTTP header field `content-security-policy`, used to specify the security policy of the resource.
pub const CONTENT_SECURITY_POLICY: &'static str = "content-security-policy";

/// The HTTP header field `content-security-policy-report-only`, used to specify the security policy of the resource.
pub const CONTENT_SECURITY_POLICY_REPORT_ONLY: &'static str = "content-security-policy-report-only";

/// The HTTP header field `content-security-policy-report`, used to specify the security policy of the resource.
pub const CONTENT_SECURITY_POLICY_REPORT: &'static str = "content-security-policy-report";

/// The HTTP header field `content-security-policy-report-to`, used to specify the security policy of the resource.
pub const CONTENT_SECURITY_POLICY_REPORT_TO: &'static str = "content-security-policy-report-to";

/// The HTTP header field `content-security-policy-report-uri`, used to specify the security policy of the resource.
pub const CONTENT_SECURITY_POLICY_REPORT_URI: &'static str = "content-security-policy-report-uri";

/// The HTTP header field `x-forwarded-for`, used to identify the originating IP address of a client connecting through a proxy.
pub const X_FORWARDED_FOR: &'static str = "x-forwarded-for";

/// The HTTP header field `x-forwarded-host`, used to identify the original host requested by the client.
pub const X_FORWARDED_HOST: &'static str = "x-forwarded-host";

/// The HTTP header field `x-forwarded-proto`, used to identify the protocol used by the client to connect to the proxy.
pub const X_FORWARDED_PROTO: &'static str = "x-forwarded-proto";

/// The HTTP header field `x-forwarded-port`, used to identify the port used by the client to connect to the proxy.
pub const X_FORWARDED_PORT: &'static str = "x-forwarded-port";

/// The HTTP header field `x-real-ip`, used to identify the real IP address of the client.
pub const X_REAL_IP: &'static str = "x-real-ip";

/// The HTTP header field `accept-patch`, used to specify which patch document formats are accepted.
pub const ACCEPT_PATCH: &'static str = "accept-patch";

/// The HTTP header field `if-range`, used to make a range request conditional.
pub const IF_RANGE: &'static str = "if-range";

/// The HTTP header field `warning`, used to carry additional information about the status of a response.
pub const WARNING_HEADER: &'static str = "warning";

/// The HTTP header field `upgrade-insecure-requests`, used to signal the server that the client prefers encrypted responses.
pub const UPGRADE_INSECURE_REQUESTS: &'static str = "upgrade-insecure-requests";

/// The HTTP header field `x-xss-protection`, used to enable XSS filtering in browsers.
pub const X_XSS_PROTECTION: &'static str = "x-xss-protection";

/// The HTTP header field `referrer-policy`, used to control how much referrer information is included with requests.
pub const REFERRER_POLICY: &'static str = "referrer-policy";

/// The HTTP header field `feature-policy`, used to control which browser features can be used.
pub const FEATURE_POLICY: &'static str = "feature-policy";

/// The HTTP header field `permissions-policy`, used to control which browser features can be used.
pub const PERMISSIONS_POLICY: &'static str = "permissions-policy";

/// The HTTP header field `cross-origin-embedder-policy`, used to prevent a document from loading cross-origin resources.
pub const CROSS_ORIGIN_EMBEDDER_POLICY: &'static str = "cross-origin-embedder-policy";

/// The HTTP header field `cross-origin-opener-policy`, used to ensure a top-level document does not share a browsing context group.
pub const CROSS_ORIGIN_OPENER_POLICY: &'static str = "cross-origin-opener-policy";

/// The HTTP header field `cross-origin-resource-policy`, used to convey a desire that the browser blocks no-cors cross-origin requests.
pub const CROSS_ORIGIN_RESOURCE_POLICY: &'static str = "cross-origin-resource-policy";

/// The HTTP header field `timing-allow-origin`, used to specify origins that are allowed to see timing information.
pub const TIMING_ALLOW_ORIGIN: &'static str = "timing-allow-origin";

/// The HTTP header field `server-timing`, used to communicate performance metrics about the request-response cycle.
pub const SERVER_TIMING: &'static str = "server-timing";

/// The HTTP header field `clear-site-data`, used to clear browsing data associated with the requesting website.
pub const CLEAR_SITE_DATA: &'static str = "clear-site-data";

/// The HTTP header field `early-data`, used to indicate that the request was sent in TLS early data.
pub const EARLY_DATA: &'static str = "early-data";

/// The HTTP header field `accept-post`, used to specify which media types are accepted for POST requests.
pub const ACCEPT_POST: &'static str = "accept-post";

/// The HTTP header field `access-control-allow-credentials`, used to indicate whether credentials can be exposed.
pub const ACCESS_CONTROL_ALLOW_CREDENTIALS: &'static str = "access-control-allow-credentials";

/// The HTTP header field `nel`, used to define a network error logging policy.
pub const NEL: &'static str = "nel";

/// The HTTP header field `report-to`, used to specify endpoints for reporting.
pub const REPORT_TO: &'static str = "report-to";

/// The HTTP header field `x-dns-prefetch-control`, used to control DNS prefetching.
pub const X_DNS_PREFETCH_CONTROL: &'static str = "x-dns-prefetch-control";

/// The HTTP header field `x-download-options`, used to prevent file downloads from being executed.
pub const X_DOWNLOAD_OPTIONS: &'static str = "x-download-options";

/// The HTTP header field `x-permitted-cross-domain-policies`, used to control cross-domain policy files.
pub const X_PERMITTED_CROSS_DOMAIN_POLICIES: &'static str = "x-permitted-cross-domain-policies";

/// The HTTP header field `x-robots-tag`, used to control how search engines index content.
pub const X_ROBOTS_TAG: &'static str = "x-robots-tag";

/// The HTTP header field `x-ua-compatible`, used to specify which version of Internet Explorer the page should be rendered as.
pub const X_UA_COMPATIBLE: &'static str = "x-ua-compatible";
