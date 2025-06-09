/// Default value indicating any language.
pub const DEFAULT: &str = "*";

/// Basic authentication scheme.
pub const BASIC: &str = "Basic";

/// Indicates no-cache.
pub const NO_CACHE: &str = "no-cache";

/// Indicates keep-alive connection.
pub const KEEP_ALIVE: &str = "keep-alive";

/// Indicates chunked transfer encoding.
pub const CHUNKED: &str = "chunked";

/// Deny page from being framed.
pub const DENY: &str = "DENY";

/// Prevent MIME sniffing.
pub const NOSNIFF: &str = "nosniff";

/// Indicates an XMLHttpRequest (AJAX).
pub const XMLHTTPREQUEST: &str = "XMLHttpRequest";

/// Gzip compression.
pub const GZIP: &str = "gzip";

/// Deflate compression.
pub const DEFLATE: &str = "deflate";

/// Brotli compression.
pub const BROTLI: &str = "br";

/// No encoding (identity).
pub const IDENTITY: &str = "identity";

/// Accept any content type.
pub const ACCEPT_ANY: &str = "*/*";

/// Wildcard any.
pub const WILDCARD_ANY: &str = "*";

/// Prefer English language.
pub const ENGLISH: &str = "en";

/// Bearer token authentication.
pub const BEARER: &str = "Bearer";

/// Private caching.
pub const PRIVATE: &str = "private";

/// Public caching.
pub const PUBLIC: &str = "public";

/// Close the connection.
pub const CLOSE: &str = "close";

/// Allow framing only by same origin.
pub const SAMEORIGIN: &str = "SAMEORIGIN";

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
