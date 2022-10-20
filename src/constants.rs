pub const URI_KEY: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-$";
pub const BASE64_KEY: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

/// The stream code for a `u8`.
pub const U8_CODE: u8 = 0;

/// The stream code for a `u16`.
pub const U16_CODE: u8 = 1;

/// End of stream signal
pub const CLOSE_CODE: u8 = 2;

/// The starting size of a code.
///
/// Compression starts with the following codes:
/// 0: u8
/// 1: u16
/// 2: close stream
pub const START_CODE_BITS: u8 = 2;
