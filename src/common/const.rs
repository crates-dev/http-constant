use crate::*;

/// A single space character.
///
/// This constant is used to represent a space character in string
/// or byte operations.
pub const SPACE: &str = " ";

/// The byte representation of a single space character.
///
/// This constant provides the byte equivalent of the space character
/// for use in low-level operations.
pub const SPACE_U8: u8 = SPACE.as_bytes()[0];

/// A tab character.
///
/// This constant is used to represent a tab character in string
/// or byte operations.
pub const TAB: &str = "\t";

/// The byte representation of a tab character.
///
/// This constant provides the byte equivalent of the tab character
/// for use in low-level operations.
pub const TAB_U8: u8 = TAB.as_bytes()[0];

/// A line break character (newline).
///
/// This constant is used to represent a line break character in
/// string or byte operations.
pub const BR: &str = "\n";

/// A const byte slice representation of the string `BR`.
pub const BR_BYTES: &[u8] = BR.as_bytes();

/// A colon followed by a space (`: `).
///
/// This constant is commonly used in formatted strings, such as
/// headers or key-value pairs, where a colon and a space are needed.
pub const COLON_SPACE: &str = ": ";

/// The byte representation of the first character in the `COLON_SPACE`.
///
/// This constant provides the byte equivalent of the colon character
/// from the `COLON_SPACE` string.
pub const COLON_SPACE_BYTES: &[u8] = COLON_SPACE.as_bytes();

/// A colon followed by a space symbol (`:`).
///
/// This constant is commonly used in formatted strings, such as
/// headers or key-value pairs, where a colon and a space are needed.
pub const COLON_SPACE_SYMBOL: &str = ":";

/// Query symbols
pub const QUERY_SYMBOL: &str = "?";

/// Hash symbols
pub const HASH_SYMBOL: &str = "#";

/// Empty str
pub const EMPTY_STR: &str = "";

/// Default host
pub const DEFAULT_HOST: &str = "0.0.0.0";

/// Default web port
pub const DEFAULT_WEB_PORT: usize = 80;

/// Http br
pub const HTTP_BR: &str = "\r\n";

/// Http br bytes
pub const HTTP_BR_BYTES: &[u8] = HTTP_BR.as_bytes();

/// Http double br
pub const HTTP_DOUBLE_BR: &str = "\r\n\r\n";

/// Http double br bytes
pub const HTTP_DOUBLE_BR_BYTES: &[u8] = HTTP_DOUBLE_BR.as_bytes();

/// Default http path
pub const DEFAULT_HTTP_PATH: &str = "/";

/// Default http path bytes
pub const DEFAULT_HTTP_PATH_BYTES: &[u8] = DEFAULT_HTTP_PATH.as_bytes();

/// And
pub const AND: &str = "&";

/// And bytes
pub const AND_BYTES: &[u8] = AND.as_bytes();

/// Equal
pub const EQUAL: &str = "=";

/// Equal bytes
pub const EQUAL_BYTES: &[u8] = EQUAL.as_bytes();

/// Zero str
pub const ZERO_STR: &str = "0";

/// Zero str bytes
pub const ZERO_STR_BYTES: &[u8] = ZERO_STR.as_bytes();

/// Default buffer size
pub const DEFAULT_BUFFER_SIZE: usize = 4096;

/// Default max redirect times
pub const DEFAULT_MAX_REDIRECT_TIMES: usize = 8;

/// Default timeout
pub const DEFAULT_TIMEOUT: u64 = u64::MAX;

/// Point
pub const POINT: &str = ".";

/// Root path
pub const ROOT_PATH: &str = "/";

/// Semicolon
pub const SEMICOLON: &str = ";";

/// Semicolon and space
pub const SEMICOLON_SPACE: &str = "; ";

/// GUID
pub const GUID: &[u8; 36] = b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

/// HASH_STATE
pub const HASH_STATE: [u32; 5] = [
    0x67452301u32,
    0xEFCDAB89,
    0x98BADCFE,
    0x10325476,
    0xC3D2E1F0,
];

/// BASE64_CHARSET_TABLE
pub const BASE64_CHARSET_TABLE: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// MAX_FRAME_SIZE
pub const MAX_FRAME_SIZE: usize = 65535;

/// MAX_UTF8_ATTEMPTS
pub const MAX_UTF8_ATTEMPTS: usize = 4;

/// DEFAULT_SOCKET_ADDR
pub const DEFAULT_SOCKET_ADDR: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0));

/// SOCKET_ADDR_127_0_0_1
pub const SOCKET_ADDR_127_0_0_1: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));

/// hyperlane
pub const HYPERLANE: &str = "hyperlane";

/// Hyperlane
pub const HYPERLANE_PASCAL_CASE: &str = "Hyperlane";

/// DEFAULT_INNER_PRINT
pub const DEFAULT_INNER_PRINT: bool = true;

/// DEFAULT_NODELAY
pub const DEFAULT_INNER_LOG: bool = true;

/// DEFAULT_NODELAY
pub const DEFAULT_NODELAY: bool = false;

/// DEFAULT_LINGER
pub const DEFAULT_LINGER: Option<Duration> = None;

/// DEFAULT_TTI
pub const DEFAULT_TTI: Option<u32> = None;

/// warning
pub const WARNING: &str = "warning";

/// success
pub const SUCCESS: &str = "success";

/// fail
pub const FAIL: &str = "fail";

/// error
pub const ERROR: &str = "error";

/// info
pub const INFO: &str = "info";

/// debug
pub const DEBUG: &str = "debug";

/// plain
pub const PLAIN: &str = "plain";

/// binary
pub const BINARY: &str = "binary";
