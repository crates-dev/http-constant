/// The default value for the `Accept-Language` header indicating any language is acceptable.
pub static ACCEPT_LANGUAGE_DEFAULT: &str = "*";

/// The value for the `Authorization` header when using Basic authentication.
pub static AUTHORIZATION_BASIC: &str = "Basic";

/// The value for the `Cache-Control` header indicating no-cache.
pub static CACHE_CONTROL_NO_CACHE: &str = "no-cache";

/// The value for the `Connection` header indicating keep-alive.
pub static CONNECTION_KEEP_ALIVE: &str = "keep-alive";

/// The value for the `Transfer-Encoding` header indicating chunked transfer encoding.
pub static TRANSFER_ENCODING_CHUNKED: &str = "chunked";

/// The value for the `X-Frame-Options` header to prevent the page from being framed.
pub static X_FRAME_OPTIONS_DENY: &str = "DENY";

/// The value for the `X-Content-Type-Options` header to prevent MIME sniffing.
pub static X_CONTENT_TYPE_OPTIONS_NOSNIFF: &str = "nosniff";

/// The value for the `X-Requested-With` header indicating an AJAX request.
pub static X_REQUESTED_WITH_XMLHTTPREQUEST: &str = "XMLHttpRequest";

/// The value for the `Accept` header indicating that any content type is acceptable.
pub static ACCEPT_ANY: &str = "*/*";

/// The value for the `Accept-Encoding` header indicating gzip compression.
pub static ACCEPT_ENCODING_GZIP: &str = "gzip";

/// The value for the `Accept-Encoding` header indicating deflate compression.
pub static ACCEPT_ENCODING_DEFLATE: &str = "deflate";

/// The value for the `Accept-Encoding` header indicating br (Brotli) compression.
pub static ACCEPT_ENCODING_BROTLI: &str = "br";

/// The value for the `Accept-Encoding` header indicating no encoding (identity).
pub static ACCEPT_ENCODING_IDENTITY: &str = "identity";

/// The value for the `Content-Encoding` header indicating gzip compression.
/// The response body is compressed using the Gzip algorithm.
pub static CONTENT_ENCODING_GZIP: &str = "gzip";

/// The value for the `Content-Encoding` header indicating deflate compression.
/// The response body is compressed using the Deflate algorithm.
pub static CONTENT_ENCODING_DEFLATE: &str = "deflate";

/// The value for the `Content-Encoding` header indicating Brotli compression.
/// The response body is compressed using the Brotli algorithm, a more modern compression algorithm.
pub static CONTENT_ENCODING_BROTLI: &str = "br";

/// The value for the `Content-Encoding` header indicating no encoding (identity).
/// The response body is not compressed or encoded.
pub static CONTENT_ENCODING_IDENTITY: &str = "identity";

/// The value for the `Accept-Language` header indicating any language is acceptable.
pub static ACCEPT_LANGUAGE_ANY: &str = "*";

/// The value for the `Accept-Language` header indicating English as the preferred language.
pub static ACCEPT_LANGUAGE_ENGLISH: &str = "en";

/// The value for the `Authorization` header indicating Bearer token authentication.
pub static AUTHORIZATION_BEARER: &str = "Bearer";

/// The value for the `Cache-Control` header indicating that the response should not be cached.
pub static CACHE_CONTROL_PRIVATE: &str = "private";

/// The value for the `Cache-Control` header indicating that the response is cacheable by any cache.
pub static CACHE_CONTROL_PUBLIC: &str = "public";

/// The value for the `Connection` header indicating a close connection.
pub static CONNECTION_CLOSE: &str = "close";

/// The value for the `X-Frame-Options` header to allow the page to be framed only by the same origin.
pub static X_FRAME_OPTIONS_SAMEORIGIN: &str = "SAMEORIGIN";

/// The value for the `Content-Type` header indicating JSON format.
pub static CONTENT_TYPE_JSON: &str = "application/json";

/// The value for the `Content-Type` header indicating Form URL Encoded data.
pub static CONTENT_TYPE_FORM_URLENCODED: &str = "application/x-www-form-urlencoded";

/// The value for the `Content-Type` header indicating Multipart Form Data.
pub static CONTENT_TYPE_MULTIPART_FORM_DATA: &str = "multipart/form-data";

/// The value for the `Content-Type` header indicating plain text.
pub static CONTENT_TYPE_TEXT_PLAIN: &str = "text/plain";

/// The value for the `Content-Type` header indicating HTML format.
pub static CONTENT_TYPE_TEXT_HTML: &str = "text/html";

/// The value for the `Content-Type` header indicating XML format.
pub static CONTENT_TYPE_XML: &str = "application/xml";

/// The value for the `Content-Type` header indicating JavaScript format.
pub static CONTENT_TYPE_JS: &str = "application/javascript";

/// The value for the `Content-Type` header indicating CSS format.
pub static CONTENT_TYPE_CSS: &str = "text/css";

/// The value for the `Content-Type` header indicating PDF format.
pub static CONTENT_TYPE_PDF: &str = "application/pdf";

/// The value for the `Content-Type` header indicating image PNG format.
pub static CONTENT_TYPE_IMAGE_PNG: &str = "image/png";

/// The value for the `Content-Type` header indicating image JPEG format.
pub static CONTENT_TYPE_IMAGE_JPEG: &str = "image/jpeg";

/// The value for the `Content-Type` header indicating image GIF format.
pub static CONTENT_TYPE_IMAGE_GIF: &str = "image/gif";

/// The value for the `Content-Type` header indicating octet stream (binary data).
pub static CONTENT_TYPE_OCTET_STREAM: &str = "application/octet-stream";

/// The value for the `Content-Type` header indicating WebP image format.
pub static CONTENT_TYPE_IMAGE_WEBP: &str = "image/webp";

/// The value for the `Content-Type` header indicating a font in TTF format.
pub static CONTENT_TYPE_FONT_TTF: &str = "font/ttf";

/// The value for the `Content-Type` header indicating a font in OTF format.
pub static CONTENT_TYPE_FONT_OTF: &str = "font/otf";

/// The value for the `Content-Type` header indicating a font in WOFF format.
pub static CONTENT_TYPE_FONT_WOFF: &str = "font/woff";

/// The value for the `Content-Type` header indicating a font in WOFF2 format.
pub static CONTENT_TYPE_FONT_WOFF2: &str = "font/woff2";
