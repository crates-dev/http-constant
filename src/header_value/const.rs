/// The default value for the `Accept-Language` header indicating any language is acceptable.
pub const ACCEPT_LANGUAGE_DEFAULT: &str = "*";

/// The value for the `Authorization` header when using Basic authentication.
pub const AUTHORIZATION_BASIC: &str = "Basic";

/// The value for the `Cache-Control` header indicating no-cache.
pub const CACHE_CONTROL_NO_CACHE: &str = "no-cache";

/// The value for the `Connection` header indicating keep-alive.
pub const CONNECTION_KEEP_ALIVE: &str = "keep-alive";

/// The value for the `Transfer-Encoding` header indicating chunked transfer encoding.
pub const TRANSFER_ENCODING_CHUNKED: &str = "chunked";

/// The value for the `X-Frame-Options` header to prevent the page from being framed.
pub const X_FRAME_OPTIONS_DENY: &str = "DENY";

/// The value for the `X-Content-Type-Options` header to prevent MIME sniffing.
pub const X_CONTENT_TYPE_OPTIONS_NOSNIFF: &str = "nosniff";

/// The value for the `X-Requested-With` header indicating an AJAX request.
pub const X_REQUESTED_WITH_XMLHTTPREQUEST: &str = "XMLHttpRequest";

/// The value for the `Accept` header indicating that any content type is acceptable.
pub const ACCEPT_ANY: &str = "*/*";

/// The value for the `Accept-Encoding` header indicating gzip compression.
pub const ACCEPT_ENCODING_GZIP: &str = "gzip";

/// The value for the `Accept-Encoding` header indicating deflate compression.
pub const ACCEPT_ENCODING_DEFLATE: &str = "deflate";

/// The value for the `Accept-Encoding` header indicating br (Brotli) compression.
pub const ACCEPT_ENCODING_BROTLI: &str = "br";

/// The value for the `Accept-Encoding` header indicating no encoding (identity).
pub const ACCEPT_ENCODING_IDENTITY: &str = "identity";

/// The value for the `Content-Encoding` header indicating gzip compression.
/// The response body is compressed using the Gzip algorithm.
pub const CONTENT_ENCODING_GZIP: &str = "gzip";

/// The value for the `Content-Encoding` header indicating deflate compression.
/// The response body is compressed using the Deflate algorithm.
pub const CONTENT_ENCODING_DEFLATE: &str = "deflate";

/// The value for the `Content-Encoding` header indicating Brotli compression.
/// The response body is compressed using the Brotli algorithm, a more modern compression algorithm.
pub const CONTENT_ENCODING_BROTLI: &str = "br";

/// The value for the `Content-Encoding` header indicating no encoding (identity).
/// The response body is not compressed or encoded.
pub const CONTENT_ENCODING_IDENTITY: &str = "identity";

/// The value for the `Accept-Language` header indicating any language is acceptable.
pub const ACCEPT_LANGUAGE_ANY: &str = "*";

/// Any
pub const ANY: &str = "*";

/// The value for the `Accept-Language` header indicating English as the preferred language.
pub const ACCEPT_LANGUAGE_ENGLISH: &str = "en";

/// The value for the `Authorization` header indicating Bearer token authentication.
pub const AUTHORIZATION_BEARER: &str = "Bearer";

/// The value for the `Cache-Control` header indicating that the response should not be cached.
pub const CACHE_CONTROL_PRIVATE: &str = "private";

/// The value for the `Cache-Control` header indicating that the response is cacheable by any cache.
pub const CACHE_CONTROL_PUBLIC: &str = "public";

/// The value for the `Connection` header indicating a close connection.
pub const CONNECTION_CLOSE: &str = "close";

/// The value for the `X-Frame-Options` header to allow the page to be framed only by the same origin.
pub const X_FRAME_OPTIONS_SAMEORIGIN: &str = "SAMEORIGIN";

/// Charset
pub const CHARSET: &str = "charset";

/// Charset Equal
pub const CHARSET_EQUAL: &str = "charset=";

/// UTF-8
pub const UTF8: &str = "utf-8";

/// ASCII
pub const ASCII: &str = "us-ascii";

/// ISO-8859-1 (Latin-1)
pub const ISO_8859_1: &str = "iso-8859-1";

/// ISO-8859-2 (Latin-2, Central European)
pub const ISO_8859_2: &str = "iso-8859-2";

/// ISO-8859-3 (Latin-3, South European)
pub const ISO_8859_3: &str = "iso-8859-3";

/// ISO-8859-4 (Latin-4, North European)
pub const ISO_8859_4: &str = "iso-8859-4";

/// ISO-8859-5 (Cyrillic)
pub const ISO_8859_5: &str = "iso-8859-5";

/// ISO-8859-6 (Arabic)
pub const ISO_8859_6: &str = "iso-8859-6";

/// ISO-8859-7 (Greek)
pub const ISO_8859_7: &str = "iso-8859-7";

/// ISO-8859-8 (Hebrew)
pub const ISO_8859_8: &str = "iso-8859-8";

/// ISO-8859-9 (Latin-5, Turkish)
pub const ISO_8859_9: &str = "iso-8859-9";

/// ISO-8859-10 (Latin-6, Nordic)
pub const ISO_8859_10: &str = "iso-8859-10";

/// ISO-8859-11 (Thai)
pub const ISO_8859_11: &str = "iso-8859-11";

/// ISO-8859-13 (Latin-7, Baltic Rim)
pub const ISO_8859_13: &str = "iso-8859-13";

/// ISO-8859-14 (Latin-8, Celtic)
pub const ISO_8859_14: &str = "iso-8859-14";

/// ISO-8859-15 (Latin-9, Western European with € symbol)
pub const ISO_8859_15: &str = "iso-8859-15";

/// ISO-8859-16 (Latin-10, South-Eastern European)
pub const ISO_8859_16: &str = "iso-8859-16";

/// Windows-1250 (Central European)
pub const WINDOWS_1250: &str = "windows-1250";

/// Windows-1251 (Cyrillic)
pub const WINDOWS_1251: &str = "windows-1251";

/// Windows-1252 (Western European)
pub const WINDOWS_1252: &str = "windows-1252";

/// Windows-1253 (Greek)
pub const WINDOWS_1253: &str = "windows-1253";

/// Windows-1254 (Turkish)
pub const WINDOWS_1254: &str = "windows-1254";

/// Windows-1255 (Hebrew)
pub const WINDOWS_1255: &str = "windows-1255";

/// Windows-1256 (Arabic)
pub const WINDOWS_1256: &str = "windows-1256";

/// Windows-1257 (Baltic)
pub const WINDOWS_1257: &str = "windows-1257";

/// Windows-1258 (Vietnamese)
pub const WINDOWS_1258: &str = "windows-1258";

/// KOI8-R (Russian)
pub const KOI8_R: &str = "koi8-r";

/// KOI8-U (Ukrainian)
pub const KOI8_U: &str = "koi8-u";

/// Shift JIS (Japanese)
pub const SHIFT_JIS: &str = "shift_jis";

/// EUC-JP (Japanese)
pub const EUC_JP: &str = "euc-jp";

/// EUC-KR (Korean)
pub const EUC_KR: &str = "euc-kr";

/// GB2312 (Simplified Chinese)
pub const GB2312: &str = "gb2312";

/// Big5 (Traditional Chinese)
pub const BIG5: &str = "big5";

/// UTF-16 (16-bit Unicode)
pub const UTF16: &str = "utf-16";

/// UTF-16LE (UTF-16 Little Endian)
pub const UTF16LE: &str = "utf-16le";

/// UTF-16BE (UTF-16 Big Endian)
pub const UTF16BE: &str = "utf-16be";

/// UTF-32 (32-bit Unicode)
pub const UTF32: &str = "utf-32";

/// UTF-32LE (UTF-32 Little Endian)
pub const UTF32LE: &str = "utf-32le";

/// UTF-32BE (UTF-32 Big Endian)
pub const UTF32BE: &str = "utf-32be";

/// Charset utf8
pub const CHARSET_UTF_8: &str = "charset=utf-8";

/// Charset iso-8859-1
pub const CHARSET_ISO_8859_1: &str = "charset=iso-8859-1";

/// Charset windows-1252
pub const CHARSET_WINDOWS_1252: &str = "charset=windows-1252";

/// Charset shift_jis
pub const CHARSET_SHIFT_JIS: &str = "charset=shift_jis";

/// Charset gb2312
pub const CHARSET_GB2312: &str = "charset=gb2312";

/// Charset big5
pub const CHARSET_BIG5: &str = "charset=big5";

/// Charset utf-16
pub const CHARSET_UTF_16: &str = "charset=utf-16";

/// Charset utf-32
pub const CHARSET_UTF_32: &str = "charset=utf-32";

/// Charset macintosh
pub const CHARSET_MACINTOSH: &str = "charset=macintosh";

/// Charset euc-kr
pub const CHARSET_EUC_KR: &str = "charset=euc-kr";

/// Charset us-ascii
pub const CHARSET_ASCII: &str = "charset=us-ascii";

/// Charset iso-8859-2
pub const CHARSET_ISO_8859_2: &str = "charset=iso-8859-2";

/// Charset iso-8859-3
pub const CHARSET_ISO_8859_3: &str = "charset=iso-8859-3";

/// Charset iso-8859-4
pub const CHARSET_ISO_8859_4: &str = "charset=iso-8859-4";

/// Charset iso-8859-5
pub const CHARSET_ISO_8859_5: &str = "charset=iso-8859-5";

/// Charset iso-8859-6
pub const CHARSET_ISO_8859_6: &str = "charset=iso-8859-6";

/// Charset iso-8859-7
pub const CHARSET_ISO_8859_7: &str = "charset=iso-8859-7";

/// Charset iso-8859-8
pub const CHARSET_ISO_8859_8: &str = "charset=iso-8859-8";

/// Charset iso-8859-9
pub const CHARSET_ISO_8859_9: &str = "charset=iso-8859-9";

/// Charset iso-8859-10
pub const CHARSET_ISO_8859_10: &str = "charset=iso-8859-10";

/// Charset iso-8859-11
pub const CHARSET_ISO_8859_11: &str = "charset=iso-8859-11";

/// Charset iso-8859-13
pub const CHARSET_ISO_8859_13: &str = "charset=iso-8859-13";

/// Charset iso-8859-14
pub const CHARSET_ISO_8859_14: &str = "charset=iso-8859-14";

/// Charset iso-8859-15
pub const CHARSET_ISO_8859_15: &str = "charset=iso-8859-15";

/// Charset iso-8859-16
pub const CHARSET_ISO_8859_16: &str = "charset=iso-8859-16";

/// Charset windows-1250
pub const CHARSET_WINDOWS_1250: &str = "charset=windows-1250";

/// Charset windows-1251
pub const CHARSET_WINDOWS_1251: &str = "charset=windows-1251";

/// Charset windows-1253
pub const CHARSET_WINDOWS_1253: &str = "charset=windows-1253";

/// Charset windows-1254
pub const CHARSET_WINDOWS_1254: &str = "charset=windows-1254";

/// Charset windows-1255
pub const CHARSET_WINDOWS_1255: &str = "charset=windows-1255";

/// Charset windows-1256
pub const CHARSET_WINDOWS_1256: &str = "charset=windows-1256";

/// Charset windows-1257
pub const CHARSET_WINDOWS_1257: &str = "charset=windows-1257";

/// Charset windows-1258
pub const CHARSET_WINDOWS_1258: &str = "charset=windows-1258";

/// Charset koi8-r
pub const CHARSET_KOI8_R: &str = "charset=koi8-r";

/// Charset koi8-u
pub const CHARSET_KOI8_U: &str = "charset=koi8-u";

/// Charset euc-jp
pub const CHARSET_EUC_JP: &str = "charset=euc-jp";

/// Charset utf-16le
pub const CHARSET_UTF_16LE: &str = "charset=utf-16le";

/// Charset utf-16be
pub const CHARSET_UTF_16BE: &str = "charset=utf-16be";

/// Charset utf-32le
pub const CHARSET_UTF_32LE: &str = "charset=utf-32le";

/// Charset utf-32be
pub const CHARSET_UTF_32BE: &str = "charset=utf-32be";

/// H2C
pub const H2C_LOWERCASE: &str = "h2c";

/// H2C
pub const H2C_UPPERCASE: &str = "H2C";

// TLS
pub const TLS_LOWERCASE: &str = "tls";

/// TLS
pub const TLS_UPPERCASE: &str = "TLS";

/// TLS/1.0
pub const TLS_1_0: &str = "TLS/1.0";

/// TLS/1.1
pub const TLS_1_1: &str = "TLS/1.1";

/// TLS/1.2
pub const TLS_1_2: &str = "TLS/1.2";

/// TLS/1.3
pub const TLS_1_3: &str = "TLS/1.3";
