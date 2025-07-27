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

/// A query symbol (`?`).
///
/// This constant represents the question mark character, which is
/// commonly used to denote the beginning of a query string in a URL.
pub const QUERY_SYMBOL: &str = "?";

/// A hash symbol (`#`).
///
/// This constant represents the hash character, which is used to
/// identify a fragment or anchor in a URL.
pub const HASH_SYMBOL: &str = "#";

/// An empty string.
///
/// This constant represents an empty string, which can be used as a
/// default or placeholder value.
pub const EMPTY_STR: &str = "";

/// The default host address.
///
/// This constant represents the default host address, which is typically
/// used to bind a server to all available network interfaces.
pub const DEFAULT_HOST: &str = "0.0.0.0";

/// The default web port.
///
/// This constant represents the default port for HTTP traffic.
pub const DEFAULT_WEB_PORT: usize = 80;

/// An HTTP line break (`\r\n`).
///
/// This constant represents the standard line break sequence used in
/// the HTTP protocol.
pub const HTTP_BR: &str = "\r\n";

/// The byte representation of an HTTP line break.
///
/// This constant provides the byte equivalent of the HTTP line break
/// for use in low-level network operations.
pub const HTTP_BR_BYTES: &[u8] = HTTP_BR.as_bytes();

/// A double HTTP line break (`\r\n\r\n`).
///
/// This constant represents the sequence used to separate headers
/// from the body in an HTTP message.
pub const HTTP_DOUBLE_BR: &str = "\r\n\r\n";

/// The byte representation of a double HTTP line break.
///
/// This constant provides the byte equivalent of the double HTTP line
/// break for use in parsing and constructing HTTP messages.
pub const HTTP_DOUBLE_BR_BYTES: &[u8] = HTTP_DOUBLE_BR.as_bytes();

/// The default HTTP path.
///
/// This constant represents the root path of a URL, which is used
/// when no specific path is provided.
pub const DEFAULT_HTTP_PATH: &str = "/";

/// The byte representation of the default HTTP path.
///
/// This constant provides the byte equivalent of the default HTTP path
/// for use in low-level operations.
pub const DEFAULT_HTTP_PATH_BYTES: &[u8] = DEFAULT_HTTP_PATH.as_bytes();

/// An ampersand character (`&`).
///
/// This constant represents the ampersand character, which is used
/// to separate query parameters in a URL.
pub const AND: &str = "&";

/// The byte representation of an ampersand character.
///
/// This constant provides the byte equivalent of the ampersand character
/// for use in URL parsing and construction.
pub const AND_BYTES: &[u8] = AND.as_bytes();

/// An equal sign (`=`).
///
/// This constant represents the equal sign, which is used to separate
/// keys and values in query parameters.
pub const EQUAL: &str = "=";

/// The byte representation of an equal sign.
///
/// This constant provides the byte equivalent of the equal sign for
/// use in URL parsing and construction.
pub const EQUAL_BYTES: &[u8] = EQUAL.as_bytes();

/// The string representation of the number zero.
///
/// This constant represents the character '0' as a string.
pub const ZERO_STR: &str = "0";

/// The byte representation of the number zero.
///
/// This constant provides the byte equivalent of the character '0'.
pub const ZERO_STR_BYTES: &[u8] = ZERO_STR.as_bytes();

/// The default buffer size.
///
/// This constant defines the default size for buffers used in I/O
/// operations, such as reading from a network stream.
pub const DEFAULT_BUFFER_SIZE: usize = 4096;

/// The default maximum number of redirect times.
///
/// This constant specifies the default limit for the number of times
/// an HTTP client should follow a redirect.
pub const DEFAULT_MAX_REDIRECT_TIMES: usize = 8;

/// The default timeout value.
///
/// This constant represents the default timeout for operations, which
/// is set to the maximum value of `u64` to indicate no timeout.
pub const DEFAULT_TIMEOUT: u64 = u64::MAX;

/// A point character (`.`).
///
/// This constant represents the period or dot character, which is
/// often used as a separator in file names or domain names.
pub const POINT: &str = ".";

/// The root path.
///
/// This constant represents the root path in a file system or URL.
pub const ROOT_PATH: &str = "/";

/// A semicolon character (`;`).
///
/// This constant represents the semicolon character, which is used
/// as a separator in various contexts.
pub const SEMICOLON: &str = ";";

/// A semicolon followed by a space (`; `).
///
/// This constant represents a semicolon followed by a space, which is
/// commonly used as a separator in formatted text.
pub const SEMICOLON_SPACE: &str = "; ";

/// A globally unique identifier (GUID) for WebSocket connections.
///
/// This constant is used in the WebSocket handshake process to create
/// the `Sec-WebSocket-Accept` header.
pub const GUID: &[u8; 36] = b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

/// The initial hash state for SHA-1.
///
/// This constant represents the initial values used in the SHA-1
/// hashing algorithm.
pub const HASH_STATE: [u32; 5] = [
    0x67452301u32,
    0xEFCDAB89,
    0x98BADCFE,
    0x10325476,
    0xC3D2E1F0,
];

/// The Base64 character set table.
///
/// This constant contains the characters used for Base64 encoding.
pub const BASE64_CHARSET_TABLE: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// The maximum frame size for WebSockets.
///
/// This constant defines the maximum size of a WebSocket frame that
/// can be processed.
pub const MAX_FRAME_SIZE: usize = 65535;

/// The maximum number of attempts to decode a UTF-8 character.
///
/// This constant specifies the maximum number of bytes that can be
/// part of a single UTF-8 character.
pub const MAX_UTF8_ATTEMPTS: usize = 4;

/// The default socket address.
///
/// This constant represents a default, unspecified socket address.
pub const DEFAULT_SOCKET_ADDR: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0));

/// The loopback socket address (127.0.0.1).
///
/// This constant represents the loopback address, which is used for
/// local network communication.
pub const SOCKET_ADDR_127_0_0_1: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0));

/// The string "hyperlane".
///
/// This constant is used for identification or naming purposes.
pub const HYPERLANE: &str = "hyperlane";

/// The string "Hyperlane" in PascalCase.
///
/// This constant is a PascalCase version of the "hyperlane" string.
pub const HYPERLANE_PASCAL_CASE: &str = "Hyperlane";

/// The string "Hyperlane" in UpperCase.
///
/// This constant is a UpperCase version of the "hyperlane" string.
pub const HYPERLANE_UPPERCASE: &str = "HYPERLANE";

/// The default setting for inner printing.
///
/// This constant determines whether internal printing is enabled by
/// default.
pub const DEFAULT_INNER_PRINT: bool = true;

/// The default setting for inner logging.
///
/// This constant determines whether internal logging is enabled by
/// default.
pub const DEFAULT_INNER_LOG: bool = true;

/// The default setting for TCP_NODELAY.
///
/// This constant specifies the default value for the `TCP_NODELAY`
/// socket option.
pub const DEFAULT_NODELAY: Option<bool> = None;

/// The default setting for socket linger.
///
/// This constant specifies the default value for the `SO_LINGER`
/// socket option.
pub const DEFAULT_LINGER: Option<Duration> = None;

/// The default time-to-live (TTL) setting.
///
/// This constant specifies the default value for the IP_TTL socket
/// option.
pub const DEFAULT_TTI: Option<u32> = None;

/// The string "warning".
///
/// This constant is used to represent a warning message type.
pub const WARNING: &str = "warning";

/// The string "success".
///
/// This constant is used to represent a success message type.
pub const SUCCESS: &str = "success";

/// The string "fail".
///
/// This constant is used to represent a failure message type.
pub const FAIL: &str = "fail";

/// The string "error".
///
/// This constant is used to represent an error message type.
pub const ERROR: &str = "error";

/// The string "info".
///
/// This constant is used to represent an informational message type.
pub const INFO: &str = "info";

/// The string "debug".
///
/// This constant is used to represent a debug message type.
pub const DEBUG: &str = "debug";

/// The string "plain".
///
/// This constant is used to represent plain text content.
pub const PLAIN: &str = "plain";

/// The string "binary".
///
/// This constant is used to represent binary content.
pub const BINARY: &str = "binary";
