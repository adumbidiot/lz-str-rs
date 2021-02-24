pub const URI_KEY: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-$";
pub const BASE64_KEY: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

/// Signal that a char is next
///
pub const CHAR_CODE: u16 = 0;

/// Signal that a wide char is next.
///
pub const WIDE_CHAR_CODE: u16 = 1;

/// End of stream signal
///
pub const CLOSE_CODE: u16 = 2;
