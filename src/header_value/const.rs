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

/// Bytes
pub const BYTES: &str = "bytes";

/// Cache control directive: max-age
pub const MAX_AGE: &str = "max-age";

/// Cache control directive: must-revalidate
pub const MUST_REVALIDATE: &str = "must-revalidate";

/// Cache control directive: no-store
pub const NO_STORE: &str = "no-store";

/// Cache control directive: no-transform
pub const NO_TRANSFORM: &str = "no-transform";

/// Cache control directive: proxy-revalidate
pub const PROXY_REVALIDATE: &str = "proxy-revalidate";

/// Cache control directive: s-maxage
pub const S_MAXAGE: &str = "s-maxage";

/// Cache control directive: immutable
pub const IMMUTABLE: &str = "immutable";

/// Cache control directive: stale-while-revalidate
pub const STALE_WHILE_REVALIDATE: &str = "stale-while-revalidate";

/// Cache control directive: stale-if-error
pub const STALE_IF_ERROR: &str = "stale-if-error";

/// Content disposition: inline
pub const INLINE: &str = "inline";

/// Content disposition: attachment
pub const ATTACHMENT: &str = "attachment";

/// Content disposition: form-data
pub const FORM_DATA: &str = "form-data";

/// X-Frame-Options: ALLOWALL
pub const ALLOWALL: &str = "ALLOWALL";

/// X-Content-Type-Options: nosniff (already exists as NOSNIFF)

/// Referrer Policy: no-referrer
pub const NO_REFERRER: &str = "no-referrer";

/// Referrer Policy: no-referrer-when-downgrade
pub const NO_REFERRER_WHEN_DOWNGRADE: &str = "no-referrer-when-downgrade";

/// Referrer Policy: origin
pub const ORIGIN_ONLY: &str = "origin";

/// Referrer Policy: origin-when-cross-origin
pub const ORIGIN_WHEN_CROSS_ORIGIN: &str = "origin-when-cross-origin";

/// Referrer Policy: same-origin
pub const SAME_ORIGIN: &str = "same-origin";

/// Referrer Policy: strict-origin
pub const STRICT_ORIGIN: &str = "strict-origin";

/// Referrer Policy: strict-origin-when-cross-origin
pub const STRICT_ORIGIN_WHEN_CROSS_ORIGIN: &str = "strict-origin-when-cross-origin";

/// Referrer Policy: unsafe-url
pub const UNSAFE_URL: &str = "unsafe-url";

/// Cross-Origin-Embedder-Policy: require-corp
pub const REQUIRE_CORP: &str = "require-corp";

/// Cross-Origin-Embedder-Policy: credentialless
pub const CREDENTIALLESS: &str = "credentialless";

/// Cross-Origin-Opener-Policy: same-origin
pub const SAME_ORIGIN_COOP: &str = "same-origin";

/// Cross-Origin-Opener-Policy: same-origin-allow-popups
pub const SAME_ORIGIN_ALLOW_POPUPS: &str = "same-origin-allow-popups";

/// Cross-Origin-Opener-Policy: unsafe-none
pub const UNSAFE_NONE: &str = "unsafe-none";

/// Cross-Origin-Resource-Policy: same-site
pub const SAME_SITE: &str = "same-site";

/// Cross-Origin-Resource-Policy: cross-origin
pub const CROSS_ORIGIN: &str = "cross-origin";

/// X-XSS-Protection: 1; mode=block
pub const XSS_PROTECTION_BLOCK: &str = "1; mode=block";

/// X-XSS-Protection: 0
pub const XSS_PROTECTION_DISABLED: &str = "0";

/// X-DNS-Prefetch-Control: on
pub const DNS_PREFETCH_ON: &str = "on";

/// X-DNS-Prefetch-Control: off
pub const DNS_PREFETCH_OFF: &str = "off";

/// X-Download-Options: noopen
pub const NOOPEN: &str = "noopen";

/// X-Permitted-Cross-Domain-Policies: none
pub const NONE: &str = "none";

/// X-Permitted-Cross-Domain-Policies: master-only
pub const MASTER_ONLY: &str = "master-only";

/// X-Permitted-Cross-Domain-Policies: by-content-type
pub const BY_CONTENT_TYPE: &str = "by-content-type";

/// X-Permitted-Cross-Domain-Policies: all
pub const ALL: &str = "all";

/// X-Robots-Tag: noindex
pub const NOINDEX: &str = "noindex";

/// X-Robots-Tag: nofollow
pub const NOFOLLOW: &str = "nofollow";

/// X-Robots-Tag: noarchive
pub const NOARCHIVE: &str = "noarchive";

/// X-Robots-Tag: nosnippet
pub const NOSNIPPET: &str = "nosnippet";

/// X-Robots-Tag: noimageindex
pub const NOIMAGEINDEX: &str = "noimageindex";

/// X-UA-Compatible: IE=edge
pub const IE_EDGE: &str = "IE=edge";

/// Expect: 100-continue
pub const CONTINUE_EXPECT: &str = "100-continue";

/// Transfer-Encoding: compress
pub const COMPRESS: &str = "compress";

/// Accept-Ranges: none
pub const NONE_RANGES: &str = "none";

/// Vary: accept-encoding
pub const VARY_ACCEPT_ENCODING: &str = "accept-encoding";

/// Vary: user-agent
pub const VARY_USER_AGENT: &str = "user-agent";

/// Vary: origin
pub const VARY_ORIGIN: &str = "origin";

/// domain
pub const COOKIE_DOMAIN: &str = "Domain";

/// domain lowercase
pub const COOKIE_DOMAIN_LOWERCASE: &str = "domain";

/// path
pub const COOKIE_PATH: &str = "Path";

/// path lowercase
pub const COOKIE_PATH_LOWERCASE: &str = "path";

/// secure
pub const COOKIE_SECURE: &str = "Secure";

/// secure lowercase
pub const COOKIE_SECURE_LOWERCASE: &str = "secure";

/// HttpOnly
pub const COOKIE_HTTP_ONLY: &str = "HttpOnly";

/// httponly lowercase
pub const COOKIE_HTTP_ONLY_LOWERCASE: &str = "httponly";

/// SameSite
pub const COOKIE_SAME_SITE: &str = "SameSite";

/// samesite lowercase
pub const COOKIE_SAME_SITE_LOWERCASE: &str = "samesite";

/// max-age
pub const COOKIE_MAX_AGE: &str = "Max-Age";

/// max-age lowercase
pub const COOKIE_MAX_AGE_LOWERCASE: &str = "max-age";

/// expires
pub const COOKIE_EXPIRES: &str = "Expires";

/// expires lowercase
pub const COOKIE_EXPIRES_LOWERCASE: &str = "expires";

/// priority
pub const COOKIE_PRIORITY: &str = "Priority";

/// priority lowercase
pub const COOKIE_PRIORITY_LOWERCASE: &str = "priority";

/// preload
pub const COOKIE_PRELOAD: &str = "Preload";

/// preload lowercase
pub const COOKIE_PRELOAD_LOWERCASE: &str = "preload";

/// SameSite value: Strict
pub const COOKIE_SAME_SITE_STRICT: &str = "Strict";

/// SameSite value: Lax
pub const COOKIE_SAME_SITE_LAX: &str = "Lax";

/// SameSite value: None
pub const COOKIE_SAME_SITE_NONE: &str = "None";

/// SameSite value: strict lowercase
pub const COOKIE_SAME_SITE_STRICT_LOWERCASE: &str = "strict";

/// SameSite value: lax lowercase
pub const COOKIE_SAME_SITE_LAX_LOWERCASE: &str = "lax";

/// SameSite value: none lowercase
pub const COOKIE_SAME_SITE_NONE_LOWERCASE: &str = "none";

/// Priority value: Low
pub const COOKIE_PRIORITY_LOW: &str = "Low";

/// Priority value: Medium
pub const COOKIE_PRIORITY_MEDIUM: &str = "Medium";

/// Priority value: High
pub const COOKIE_PRIORITY_HIGH: &str = "High";

/// Priority value: low lowercase
pub const COOKIE_PRIORITY_LOW_LOWERCASE: &str = "low";

/// Priority value: medium lowercase
pub const COOKIE_PRIORITY_MEDIUM_LOWERCASE: &str = "medium";

/// Priority value: high lowercase
pub const COOKIE_PRIORITY_HIGH_LOWERCASE: &str = "high";

/// Cookie attribute with value: SameSite=Strict
pub const COOKIE_SAME_SITE_STRICT_ATTR: &str = "SameSite=Strict";

/// Cookie attribute with value: SameSite=Lax
pub const COOKIE_SAME_SITE_LAX_ATTR: &str = "SameSite=Lax";

/// Cookie attribute with value: SameSite=None
pub const COOKIE_SAME_SITE_NONE_ATTR: &str = "SameSite=None";

/// Cookie attribute with value: samesite=strict lowercase
pub const COOKIE_SAME_SITE_STRICT_ATTR_LOWERCASE: &str = "samesite=strict";

/// Cookie attribute with value: samesite=lax lowercase
pub const COOKIE_SAME_SITE_LAX_ATTR_LOWERCASE: &str = "samesite=lax";

/// Cookie attribute with value: samesite=none lowercase
pub const COOKIE_SAME_SITE_NONE_ATTR_LOWERCASE: &str = "samesite=none";

/// Cookie attribute with value: Priority=Low
pub const COOKIE_PRIORITY_LOW_ATTR: &str = "Priority=Low";

/// Cookie attribute with value: Priority=Medium
pub const COOKIE_PRIORITY_MEDIUM_ATTR: &str = "Priority=Medium";

/// Cookie attribute with value: Priority=High
pub const COOKIE_PRIORITY_HIGH_ATTR: &str = "Priority=High";

/// Cookie attribute with value: priority=low lowercase
pub const COOKIE_PRIORITY_LOW_ATTR_LOWERCASE: &str = "priority=low";

/// Cookie attribute with value: priority=medium lowercase
pub const COOKIE_PRIORITY_MEDIUM_ATTR_LOWERCASE: &str = "priority=medium";

/// Cookie attribute with value: priority=high lowercase
pub const COOKIE_PRIORITY_HIGH_ATTR_LOWERCASE: &str = "priority=high";

/// Cookie separator
pub const COOKIE_SEPARATOR: &str = "; ";

/// Cookie name-value separator
pub const COOKIE_NAME_VALUE_SEPARATOR: &str = "=";

/// Cookie attribute prefix: Domain=
pub const COOKIE_DOMAIN_PREFIX: &str = "Domain=";

/// Cookie attribute prefix: domain= lowercase
pub const COOKIE_DOMAIN_PREFIX_LOWERCASE: &str = "domain=";

/// Cookie attribute prefix: Path=
pub const COOKIE_PATH_PREFIX: &str = "Path=";

/// Cookie attribute prefix: path= lowercase
pub const COOKIE_PATH_PREFIX_LOWERCASE: &str = "path=";

/// Cookie attribute prefix: Max-Age=
pub const COOKIE_MAX_AGE_PREFIX: &str = "Max-Age=";

/// Cookie attribute prefix: max-age= lowercase
pub const COOKIE_MAX_AGE_PREFIX_LOWERCASE: &str = "max-age=";

/// Cookie attribute prefix: Expires=
pub const COOKIE_EXPIRES_PREFIX: &str = "Expires=";

/// Cookie attribute prefix: expires= lowercase
pub const COOKIE_EXPIRES_PREFIX_LOWERCASE: &str = "expires=";

/// Cookie attribute prefix: SameSite=
pub const COOKIE_SAME_SITE_PREFIX: &str = "SameSite=";

/// Cookie attribute prefix: samesite= lowercase
pub const COOKIE_SAME_SITE_PREFIX_LOWERCASE: &str = "samesite=";

/// Cookie attribute prefix: Priority=
pub const COOKIE_PRIORITY_PREFIX: &str = "Priority=";

/// Cookie attribute prefix: priority= lowercase
pub const COOKIE_PRIORITY_PREFIX_LOWERCASE: &str = "priority=";

/// Cookie common max-age values: 0 (delete immediately)
pub const COOKIE_MAX_AGE_DELETE: &str = "0";

/// Cookie common max-age values: session (no max-age)
pub const COOKIE_MAX_AGE_SESSION: &str = "";

/// Cookie common max-age values: 1 hour
pub const COOKIE_MAX_AGE_1_HOUR: &str = "3600";

/// Cookie common max-age values: 1 day
pub const COOKIE_MAX_AGE_1_DAY: &str = "86400";

/// Cookie common max-age values: 1 week
pub const COOKIE_MAX_AGE_1_WEEK: &str = "604800";

/// Cookie common max-age values: 1 month (30 days)
pub const COOKIE_MAX_AGE_1_MONTH: &str = "2592000";

/// Cookie common max-age values: 1 year
pub const COOKIE_MAX_AGE_1_YEAR: &str = "31536000";

/// Cookie common path values: root path
pub const COOKIE_PATH_ROOT: &str = "/";

/// Cookie common domain values: localhost
pub const COOKIE_DOMAIN_LOCALHOST: &str = "localhost";

/// Cookie __Secure- prefix
pub const COOKIE_SECURE_PREFIX: &str = "__Secure-";

/// Cookie __Host- prefix
pub const COOKIE_HOST_PREFIX: &str = "__Host-";

/// Cookie complete attribute strings with common values
/// Complete secure cookie attributes
pub const COOKIE_SECURE_HTTP_ONLY: &str = "Secure; HttpOnly";

/// Complete secure cookie attributes lowercase
pub const COOKIE_SECURE_HTTP_ONLY_LOWERCASE: &str = "secure; httponly";

/// Complete secure cookie with SameSite=Strict
pub const COOKIE_SECURE_HTTP_ONLY_SAME_SITE_STRICT: &str = "Secure; HttpOnly; SameSite=Strict";

/// Complete secure cookie with SameSite=Strict lowercase
pub const COOKIE_SECURE_HTTP_ONLY_SAME_SITE_STRICT_LOWERCASE: &str =
    "secure; httponly; samesite=strict";

/// Complete secure cookie with SameSite=Lax
pub const COOKIE_SECURE_HTTP_ONLY_SAME_SITE_LAX: &str = "Secure; HttpOnly; SameSite=Lax";

/// Complete secure cookie with SameSite=Lax lowercase
pub const COOKIE_SECURE_HTTP_ONLY_SAME_SITE_LAX_LOWERCASE: &str = "secure; httponly; samesite=lax";

/// Complete secure cookie with SameSite=None
pub const COOKIE_SECURE_HTTP_ONLY_SAME_SITE_NONE: &str = "Secure; HttpOnly; SameSite=None";

/// Complete secure cookie with SameSite=None lowercase
pub const COOKIE_SECURE_HTTP_ONLY_SAME_SITE_NONE_LOWERCASE: &str =
    "secure; httponly; samesite=none";

/// Cookie with root path
pub const COOKIE_PATH_ROOT_ATTR: &str = "Path=/";

/// Cookie with root path lowercase
pub const COOKIE_PATH_ROOT_ATTR_LOWERCASE: &str = "path=/";

/// Cookie deletion attributes (Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT)
pub const COOKIE_DELETE_ATTRS: &str = "Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT";

/// Cookie deletion attributes lowercase
pub const COOKIE_DELETE_ATTRS_LOWERCASE: &str = "max-age=0; expires=Thu, 01 Jan 1970 00:00:00 GMT";

/// Common expires date for deletion
pub const COOKIE_EXPIRES_EPOCH: &str = "Thu, 01 Jan 1970 00:00:00 GMT";

/// Cookie attribute: Expires
pub const COOKIE_EXPIRES_ATTRIBUTE: &str = "; Expires=";

/// Cookie attribute: expires lowercase
pub const COOKIE_EXPIRES_ATTRIBUTE_LOWERCASE: &str = "; expires=";

/// Cookie attribute: Max-Age
pub const COOKIE_MAX_AGE_ATTRIBUTE: &str = "; Max-Age=";

/// Cookie attribute: max-age lowercase
pub const COOKIE_MAX_AGE_ATTRIBUTE_LOWERCASE: &str = "; max-age=";

/// Cookie attribute: Domain
pub const COOKIE_DOMAIN_ATTRIBUTE: &str = "; Domain=";

/// Cookie attribute: domain lowercase
pub const COOKIE_DOMAIN_ATTRIBUTE_LOWERCASE: &str = "; domain=";

/// Cookie attribute: Path
pub const COOKIE_PATH_ATTRIBUTE: &str = "; Path=";

/// Cookie attribute: path lowercase
pub const COOKIE_PATH_ATTRIBUTE_LOWERCASE: &str = "; path=";

/// Cookie attribute: Secure
pub const COOKIE_SECURE_ATTRIBUTE: &str = "; Secure";

/// Cookie attribute: secure lowercase
pub const COOKIE_SECURE_ATTRIBUTE_LOWERCASE: &str = "; secure";

/// Cookie attribute: HttpOnly
pub const COOKIE_HTTP_ONLY_ATTRIBUTE: &str = "; HttpOnly";

/// Cookie attribute: httponly lowercase
pub const COOKIE_HTTP_ONLY_ATTRIBUTE_LOWERCASE: &str = "; httponly";

/// Cookie attribute: SameSite
pub const COOKIE_SAME_SITE_ATTRIBUTE: &str = "; SameSite=";

/// Cookie attribute: samesite lowercase
pub const COOKIE_SAME_SITE_ATTRIBUTE_LOWERCASE: &str = "; samesite=";
