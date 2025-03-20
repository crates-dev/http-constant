/// The default value for the `Accept-Language` header indicating any language is acceptable.
pub static ACCEPT_LANGUAGE_DEFAULT: &str = "*";

/// The value for the `Authorization` header when using Basic authentication.
pub static AUTHORIZATION_BASIC: &str = "Basic";

/// websocket
pub static WEBSOCKET: &str = "websocket";

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

/// Any
pub static ANY: &str = "*";

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

/// Charset
pub static CHARSET: &str = "charset";

/// Charset Equal
pub static CHARSET_EQUAL: &str = "charset=";

/// UTF-8
pub static UTF8: &str = "utf-8";

/// ASCII
pub static ASCII: &str = "us-ascii";

/// ISO-8859-1 (Latin-1)
pub static ISO_8859_1: &str = "iso-8859-1";

/// ISO-8859-2 (Latin-2, Central European)
pub static ISO_8859_2: &str = "iso-8859-2";

/// ISO-8859-3 (Latin-3, South European)
pub static ISO_8859_3: &str = "iso-8859-3";

/// ISO-8859-4 (Latin-4, North European)
pub static ISO_8859_4: &str = "iso-8859-4";

/// ISO-8859-5 (Cyrillic)
pub static ISO_8859_5: &str = "iso-8859-5";

/// ISO-8859-6 (Arabic)
pub static ISO_8859_6: &str = "iso-8859-6";

/// ISO-8859-7 (Greek)
pub static ISO_8859_7: &str = "iso-8859-7";

/// ISO-8859-8 (Hebrew)
pub static ISO_8859_8: &str = "iso-8859-8";

/// ISO-8859-9 (Latin-5, Turkish)
pub static ISO_8859_9: &str = "iso-8859-9";

/// ISO-8859-10 (Latin-6, Nordic)
pub static ISO_8859_10: &str = "iso-8859-10";

/// ISO-8859-11 (Thai)
pub static ISO_8859_11: &str = "iso-8859-11";

/// ISO-8859-13 (Latin-7, Baltic Rim)
pub static ISO_8859_13: &str = "iso-8859-13";

/// ISO-8859-14 (Latin-8, Celtic)
pub static ISO_8859_14: &str = "iso-8859-14";

/// ISO-8859-15 (Latin-9, Western European with € symbol)
pub static ISO_8859_15: &str = "iso-8859-15";

/// ISO-8859-16 (Latin-10, South-Eastern European)
pub static ISO_8859_16: &str = "iso-8859-16";

/// Windows-1250 (Central European)
pub static WINDOWS_1250: &str = "windows-1250";

/// Windows-1251 (Cyrillic)
pub static WINDOWS_1251: &str = "windows-1251";

/// Windows-1252 (Western European)
pub static WINDOWS_1252: &str = "windows-1252";

/// Windows-1253 (Greek)
pub static WINDOWS_1253: &str = "windows-1253";

/// Windows-1254 (Turkish)
pub static WINDOWS_1254: &str = "windows-1254";

/// Windows-1255 (Hebrew)
pub static WINDOWS_1255: &str = "windows-1255";

/// Windows-1256 (Arabic)
pub static WINDOWS_1256: &str = "windows-1256";

/// Windows-1257 (Baltic)
pub static WINDOWS_1257: &str = "windows-1257";

/// Windows-1258 (Vietnamese)
pub static WINDOWS_1258: &str = "windows-1258";

/// KOI8-R (Russian)
pub static KOI8_R: &str = "koi8-r";

/// KOI8-U (Ukrainian)
pub static KOI8_U: &str = "koi8-u";

/// Shift JIS (Japanese)
pub static SHIFT_JIS: &str = "shift_jis";

/// EUC-JP (Japanese)
pub static EUC_JP: &str = "euc-jp";

/// EUC-KR (Korean)
pub static EUC_KR: &str = "euc-kr";

/// GB2312 (Simplified Chinese)
pub static GB2312: &str = "gb2312";

/// Big5 (Traditional Chinese)
pub static BIG5: &str = "big5";

/// UTF-16 (16-bit Unicode)
pub static UTF16: &str = "utf-16";

/// UTF-16LE (UTF-16 Little Endian)
pub static UTF16LE: &str = "utf-16le";

/// UTF-16BE (UTF-16 Big Endian)
pub static UTF16BE: &str = "utf-16be";

/// UTF-32 (32-bit Unicode)
pub static UTF32: &str = "utf-32";

/// UTF-32LE (UTF-32 Little Endian)
pub static UTF32LE: &str = "utf-32le";

/// UTF-32BE (UTF-32 Big Endian)
pub static UTF32BE: &str = "utf-32be";

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

/// Charset us-ascii
pub static CHARSET_ASCII: &str = "charset=us-ascii";

/// Charset iso-8859-2
pub static CHARSET_ISO_8859_2: &str = "charset=iso-8859-2";

/// Charset iso-8859-3
pub static CHARSET_ISO_8859_3: &str = "charset=iso-8859-3";

/// Charset iso-8859-4
pub static CHARSET_ISO_8859_4: &str = "charset=iso-8859-4";

/// Charset iso-8859-5
pub static CHARSET_ISO_8859_5: &str = "charset=iso-8859-5";

/// Charset iso-8859-6
pub static CHARSET_ISO_8859_6: &str = "charset=iso-8859-6";

/// Charset iso-8859-7
pub static CHARSET_ISO_8859_7: &str = "charset=iso-8859-7";

/// Charset iso-8859-8
pub static CHARSET_ISO_8859_8: &str = "charset=iso-8859-8";

/// Charset iso-8859-9
pub static CHARSET_ISO_8859_9: &str = "charset=iso-8859-9";

/// Charset iso-8859-10
pub static CHARSET_ISO_8859_10: &str = "charset=iso-8859-10";

/// Charset iso-8859-11
pub static CHARSET_ISO_8859_11: &str = "charset=iso-8859-11";

/// Charset iso-8859-13
pub static CHARSET_ISO_8859_13: &str = "charset=iso-8859-13";

/// Charset iso-8859-14
pub static CHARSET_ISO_8859_14: &str = "charset=iso-8859-14";

/// Charset iso-8859-15
pub static CHARSET_ISO_8859_15: &str = "charset=iso-8859-15";

/// Charset iso-8859-16
pub static CHARSET_ISO_8859_16: &str = "charset=iso-8859-16";

/// Charset windows-1250
pub static CHARSET_WINDOWS_1250: &str = "charset=windows-1250";

/// Charset windows-1251
pub static CHARSET_WINDOWS_1251: &str = "charset=windows-1251";

/// Charset windows-1253
pub static CHARSET_WINDOWS_1253: &str = "charset=windows-1253";

/// Charset windows-1254
pub static CHARSET_WINDOWS_1254: &str = "charset=windows-1254";

/// Charset windows-1255
pub static CHARSET_WINDOWS_1255: &str = "charset=windows-1255";

/// Charset windows-1256
pub static CHARSET_WINDOWS_1256: &str = "charset=windows-1256";

/// Charset windows-1257
pub static CHARSET_WINDOWS_1257: &str = "charset=windows-1257";

/// Charset windows-1258
pub static CHARSET_WINDOWS_1258: &str = "charset=windows-1258";

/// Charset koi8-r
pub static CHARSET_KOI8_R: &str = "charset=koi8-r";

/// Charset koi8-u
pub static CHARSET_KOI8_U: &str = "charset=koi8-u";

/// Charset euc-jp
pub static CHARSET_EUC_JP: &str = "charset=euc-jp";

/// Charset utf-16le
pub static CHARSET_UTF_16LE: &str = "charset=utf-16le";

/// Charset utf-16be
pub static CHARSET_UTF_16BE: &str = "charset=utf-16be";

/// Charset utf-32le
pub static CHARSET_UTF_32LE: &str = "charset=utf-32le";

/// Charset utf-32be
pub static CHARSET_UTF_32BE: &str = "charset=utf-32be";
