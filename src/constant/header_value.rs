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

/// Charset utf8
pub static CHARSET_UTF_8: &str = "charset=utf-8";

/// Charset iso-8859-1
pub static CHARSET_ISO_8859_1: &str = "charset=iso-8859-1";

/// Charset windows-1252
pub static CHARSET_WINDOWS_1252: &str = "charset=windows-1252";

/// Charset shift_jis
pub static CHARSET_SHIFT_JIS: &str = "charset=shift_jis";

/// Charset gb2312
pub static CHARSET_GB2312: &str = "charset=gb2312";

/// Charset big5
pub static CHARSET_BIG5: &str = "charset=big5";

/// Charset utf-16
pub static CHARSET_UTF_16: &str = "charset=utf-16";

/// Charset utf-32
pub static CHARSET_UTF_32: &str = "charset=utf-32";

/// Charset macintosh
pub static CHARSET_MACINTOSH: &str = "charset=macintosh";

/// Charset euc-kr
pub static CHARSET_EUC_KR: &str = "charset=euc-kr";
