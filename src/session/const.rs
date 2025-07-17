/// Default session cookie name
pub const SESSION_COOKIE_NAME: &str = "session_id";

/// Default session ID length
pub const SESSION_ID_LENGTH: usize = 32;

/// Default session timeout in seconds (30 minutes)
pub const SESSION_TIMEOUT_SECONDS: u64 = 1800;

/// Default session timeout in milliseconds (30 minutes)
pub const SESSION_TIMEOUT_MILLISECONDS: u64 = 1800000;

/// Session
pub const SESSION: &str = "session";

/// Session data key for user ID
pub const SESSION_USER_ID_KEY: &str = "user_id";

/// Session data key for username
pub const SESSION_USERNAME_KEY: &str = "username";

/// Session data key for user role
pub const SESSION_USER_ROLE_KEY: &str = "user_role";

/// Session data key for login time
pub const SESSION_LOGIN_TIME_KEY: &str = "login_time";

/// Session data key for last access time
pub const SESSION_LAST_ACCESS_TIME_KEY: &str = "last_access_time";

/// Session data key for IP address
pub const SESSION_IP_ADDRESS_KEY: &str = "ip_address";

/// Session data key for user agent
pub const SESSION_USER_AGENT_KEY: &str = "user_agent";

/// Session data key for CSRF token
pub const SESSION_CSRF_TOKEN_KEY: &str = "csrf_token";

/// Session data key for language preference
pub const SESSION_LANGUAGE_KEY: &str = "language";

/// Session data key for timezone
pub const SESSION_TIMEZONE_KEY: &str = "timezone";

/// Session state: active
pub const SESSION_STATE_ACTIVE: &str = "active";

/// Session state: expired
pub const SESSION_STATE_EXPIRED: &str = "expired";

/// Session state: invalid
pub const SESSION_STATE_INVALID: &str = "invalid";

/// Session state: destroyed
pub const SESSION_STATE_DESTROYED: &str = "destroyed";

/// Default session cleanup interval in seconds (5 minutes)
pub const SESSION_CLEANUP_INTERVAL_SECONDS: u64 = 300;

/// Maximum number of sessions per user
pub const MAX_SESSIONS_PER_USER: usize = 5;

/// Session ID character set for generation
pub const SESSION_ID_CHARSET: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// Session ID character set bytes
pub const SESSION_ID_CHARSET_BYTES: &[u8] = SESSION_ID_CHARSET.as_bytes();

/// Session regeneration threshold in seconds (15 minutes)
pub const SESSION_REGENERATION_THRESHOLD_SECONDS: u64 = 900;

/// Session flash message key prefix
pub const SESSION_FLASH_MESSAGE_PREFIX: &str = "flash:";

/// Session flash message types
pub const SESSION_FLASH_SUCCESS: &str = "success";
pub const SESSION_FLASH_ERROR: &str = "error";
pub const SESSION_FLASH_WARNING: &str = "warning";
pub const SESSION_FLASH_INFO: &str = "info";

/// Session remember me cookie name
pub const SESSION_REMEMBER_ME_COOKIE_NAME: &str = "remember_me";

/// Session remember me token length
pub const SESSION_REMEMBER_ME_TOKEN_LENGTH: usize = 64;

/// Session remember me timeout in seconds (30 days)
pub const SESSION_REMEMBER_ME_TIMEOUT_SECONDS: u64 = 2592000;

/// Session fingerprint components separator
pub const SESSION_FINGERPRINT_SEPARATOR: &str = "|";

/// Session lock timeout in milliseconds
pub const SESSION_LOCK_TIMEOUT_MILLISECONDS: u64 = 5000;

/// Session concurrent access limit
pub const SESSION_CONCURRENT_ACCESS_LIMIT: usize = 10;
