/// A single space character.
///
/// This constant is used to represent a space character in string
/// or byte operations.
pub static SPACE: &str = " ";

/// The byte representation of a single space character.
///
/// This constant provides the byte equivalent of the space character
/// for use in low-level operations.
pub static SPACE_U8: u8 = SPACE.as_bytes()[0];

/// A tab character.
///
/// This constant is used to represent a tab character in string
/// or byte operations.
pub static TAB: &str = "\t";

/// The byte representation of a tab character.
///
/// This constant provides the byte equivalent of the tab character
/// for use in low-level operations.
pub static TAB_U8: u8 = TAB.as_bytes()[0];

/// A line break character (newline).
///
/// This constant is used to represent a line break character in
/// string or byte operations.
pub static BR: &str = "\n";

/// A static byte slice representation of the string `BR`.
pub static BR_BYTES: &[u8] = BR.as_bytes();

/// A colon followed by a space (`: `).
///
/// This constant is commonly used in formatted strings, such as
/// headers or key-value pairs, where a colon and a space are needed.
pub static COLON_SPACE: &str = ": ";

/// The byte representation of the first character in the `COLON_SPACE`.
///
/// This constant provides the byte equivalent of the colon character
/// from the `COLON_SPACE` string.
pub static COLON_SPACE_BYTES: &[u8] = COLON_SPACE.as_bytes();

/// A colon followed by a space symbol (`:`).
///
/// This constant is commonly used in formatted strings, such as
/// headers or key-value pairs, where a colon and a space are needed.
pub static COLON_SPACE_SYMBOL: &str = ":";

/// Query symbols
pub static QUERY_SYMBOL: &str = "?";

/// Hash symbols
pub static HASH_SYMBOL: &str = "#";

/// Empty str
pub static EMPTY_STR: &str = "";

/// Default host
pub static DEFAULT_HOST: &str = "0.0.0.0";

/// Default web port
pub static DEFAULT_WEB_PORT: usize = 80;

/// Http br
pub static HTTP_BR: &str = "\r\n";

/// Http br bytes
pub static HTTP_BR_BYTES: &[u8] = HTTP_BR.as_bytes();

/// Http doubble br
pub static HTTP_DOUBLE_BR: &str = "\r\n\r\n";

/// Http doubble br bytes
pub static HTTP_DOUBLE_BR_BYTES: &[u8] = HTTP_DOUBLE_BR.as_bytes();

/// Default http path
pub static DEFAULT_HTTP_PATH: &str = "/";

/// Default http path bytes
pub static DEFAULT_HTTP_PATH_BYTES: &[u8] = DEFAULT_HTTP_PATH.as_bytes();

/// And
pub static AND: &str = "&";

/// And bytes
pub static AND_BYTES: &[u8] = AND.as_bytes();

/// Equal
pub static EQUAL: &str = "=";

/// Equa bytes
pub static EQUAL_BYTES: &[u8] = EQUAL.as_bytes();

/// Zero str
pub static ZERO_STR: &str = "0";

/// Zero str bytes
pub static ZERO_STR_BYTES: &[u8] = ZERO_STR.as_bytes();
