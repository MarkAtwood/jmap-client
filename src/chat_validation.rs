//! Input validation for JMAP Chat fields.
//!
//! All peer-supplied fields are attacker-controlled. These functions enforce
//! the security invariants from draft-atwood-jmap-chat §Security Considerations.

use std::fmt;

/// Errors returned by chat input validation functions.
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    /// Filename contains a path separator or null byte.
    InvalidFilename(String),
    /// Content-Type is syntactically invalid.
    InvalidContentType(String),
    /// Emoji string is empty or exceeds the maximum byte length.
    InvalidEmoji(String),
    /// A mention's offset+length exceeds the body byte length.
    MentionOutOfBounds {
        offset: u64,
        length: u64,
        body_len: u64,
    },
    /// SHA-256 hex string is not exactly 64 lowercase hex characters.
    InvalidSha256Hex(String),
    /// Body exceeds the server-advertised maximum byte length.
    BodyTooLarge { actual: usize, max: usize },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidFilename(msg) => write!(f, "invalid filename: {}", msg),
            ValidationError::InvalidContentType(msg) => write!(f, "invalid content-type: {}", msg),
            ValidationError::InvalidEmoji(msg) => write!(f, "invalid emoji: {}", msg),
            ValidationError::MentionOutOfBounds {
                offset,
                length,
                body_len,
            } => {
                if *length == 0 {
                    write!(f, "mention has zero length at offset {}", offset)
                } else {
                    write!(
                        f,
                        "mention out of bounds: offset {} + length {} > body {} bytes",
                        offset, length, body_len
                    )
                }
            }
            ValidationError::InvalidSha256Hex(msg) => write!(f, "invalid sha256 hex: {}", msg),
            ValidationError::BodyTooLarge { actual, max } => {
                write!(f, "body too large: {} bytes, max {} bytes", actual, max)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validate an attachment filename.
///
/// Rejects names containing `/`, `\`, or null bytes (0x00), per spec §Input Validation.
pub fn validate_filename(s: &str) -> Result<(), ValidationError> {
    if s.contains('/') || s.contains('\\') || s.contains('\0') {
        Err(ValidationError::InvalidFilename(format!(
            "filename {:?} contains a forbidden character (/, \\, or null)",
            s
        )))
    } else {
        Ok(())
    }
}

/// Validate a MIME content-type string.
///
/// Requires the form `type/subtype` with optional `;parameters`.
/// Does not import a MIME crate; uses simple structural checks.
pub fn validate_content_type(s: &str) -> Result<(), ValidationError> {
    if s.is_empty() {
        return Err(ValidationError::InvalidContentType(
            "content-type is empty".to_string(),
        ));
    }
    // Split on first '/' only
    let slash_pos = s.find('/').ok_or_else(|| {
        ValidationError::InvalidContentType(format!("no '/' in content-type {:?}", s))
    })?;
    let type_part = &s[..slash_pos];
    let rest = &s[slash_pos + 1..];
    // subtype is the part before an optional ';'
    let subtype_part = rest.split(';').next().unwrap_or("");

    if type_part.is_empty() {
        return Err(ValidationError::InvalidContentType(
            "media type is empty".to_string(),
        ));
    }
    if subtype_part.trim().is_empty() {
        return Err(ValidationError::InvalidContentType(
            "media subtype is empty".to_string(),
        ));
    }
    // subtype_part must not contain a nested '/' (e.g. "text//plain" gives subtype "/plain")
    if subtype_part.contains('/') {
        return Err(ValidationError::InvalidContentType(
            "extra '/' in content-type".to_string(),
        ));
    }
    Ok(())
}

/// Validate an emoji string.
///
/// Rejects empty strings and strings exceeding `max_bytes` in UTF-8 byte length,
/// per spec §Input Validation.
pub fn validate_emoji(s: &str, max_bytes: usize) -> Result<(), ValidationError> {
    if s.is_empty() {
        return Err(ValidationError::InvalidEmoji("emoji is empty".to_string()));
    }
    if s.len() > max_bytes {
        return Err(ValidationError::InvalidEmoji(format!(
            "emoji is {} bytes, exceeds maximum of {}",
            s.len(),
            max_bytes
        )));
    }
    Ok(())
}

/// Validate that a mention's byte range fits within the body.
///
/// Rejects a mention where `offset + length` overflows or exceeds `body_byte_len`,
/// per spec §Input Validation.
pub fn validate_mention_bounds(
    offset: u64,
    length: u64,
    body_byte_len: u64,
) -> Result<(), ValidationError> {
    if length == 0 {
        return Err(ValidationError::MentionOutOfBounds {
            offset,
            length,
            body_len: body_byte_len,
        });
    }
    let end = offset
        .checked_add(length)
        .ok_or(ValidationError::MentionOutOfBounds {
            offset,
            length,
            body_len: body_byte_len,
        })?;
    if end > body_byte_len {
        return Err(ValidationError::MentionOutOfBounds {
            offset,
            length,
            body_len: body_byte_len,
        });
    }
    Ok(())
}

/// Validate a lowercase hex SHA-256 string (exactly 64 hex chars).
pub fn validate_sha256_hex(s: &str) -> Result<(), ValidationError> {
    if s.len() != 64 {
        return Err(ValidationError::InvalidSha256Hex(format!(
            "expected 64 hex chars, got {}",
            s.len()
        )));
    }
    if !s.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')) {
        return Err(ValidationError::InvalidSha256Hex(
            "contains non-lowercase-hex character".to_string(),
        ));
    }
    Ok(())
}

/// Validate a custom emoji name.
///
/// Names must be non-empty and contain only ASCII lowercase letters, digits, `-`, or `_`,
/// per spec §Custom Emoji.
pub fn validate_emoji_name(name: &str) -> Result<(), ValidationError> {
    if name.is_empty()
        || !name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    {
        Err(ValidationError::InvalidEmoji(format!(
            "emoji name {:?} must be non-empty and contain only [a-z0-9_-]",
            name
        )))
    } else {
        Ok(())
    }
}

/// Validate that a body fits within the server-advertised maximum byte length.
pub fn validate_body_bytes(body: &str, max_bytes: usize) -> Result<(), ValidationError> {
    if body.len() > max_bytes {
        return Err(ValidationError::BodyTooLarge {
            actual: body.len(),
            max: max_bytes,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- validate_filename ---
    #[test]
    fn test_filename_ok() {
        assert!(validate_filename("photo.jpg").is_ok());
    }
    #[test]
    fn test_filename_slash() {
        assert!(validate_filename("dir/file.jpg").is_err());
    }
    #[test]
    fn test_filename_backslash() {
        assert!(validate_filename("dir\\file").is_err());
    }
    #[test]
    fn test_filename_null() {
        assert!(validate_filename("file\0name").is_err());
    }
    #[test]
    fn test_filename_empty() {
        assert!(validate_filename("").is_ok());
    } // empty ok, no bad chars

    // --- validate_content_type ---
    #[test]
    fn test_content_type_ok() {
        assert!(validate_content_type("image/png").is_ok());
    }
    #[test]
    fn test_content_type_ok_params() {
        assert!(validate_content_type("text/plain; charset=utf-8").is_ok());
    }
    #[test]
    fn test_content_type_empty() {
        assert!(validate_content_type("").is_err());
    }
    #[test]
    fn test_content_type_no_slash() {
        assert!(validate_content_type("imagepng").is_err());
    }
    #[test]
    fn test_content_type_empty_subtype() {
        assert!(validate_content_type("image/").is_err());
    }

    // --- validate_emoji ---
    #[test]
    fn test_emoji_ok() {
        assert!(validate_emoji("👍", 64).is_ok());
    }
    #[test]
    fn test_emoji_empty() {
        assert!(validate_emoji("", 64).is_err());
    }
    #[test]
    fn test_emoji_too_long() {
        let long = "a".repeat(65);
        assert!(validate_emoji(&long, 64).is_err());
    }

    // --- validate_mention_bounds ---
    #[test]
    fn test_mention_ok() {
        assert!(validate_mention_bounds(5, 4, 10).is_ok());
    } // 5+4=9 <= 10
    #[test]
    fn test_mention_exact() {
        assert!(validate_mention_bounds(6, 4, 10).is_ok());
    } // 6+4=10 == 10
    #[test]
    fn test_mention_overflow_arith() {
        assert!(validate_mention_bounds(u64::MAX, 1, 10).is_err());
    }
    #[test]
    fn test_mention_out_of_bounds() {
        assert!(validate_mention_bounds(8, 5, 10).is_err());
    } // 8+5=13 > 10
    #[test]
    fn test_mention_zero_length() {
        assert!(validate_mention_bounds(0, 0, 10).is_err());
    }
    #[test]
    fn test_mention_zero_length_valid_offset() {
        assert!(validate_mention_bounds(5, 0, 10).is_err());
    }

    // --- validate_sha256_hex ---
    #[test]
    fn test_sha256_ok() {
        assert!(validate_sha256_hex(
            "aabbccddeeff00112233445566778899aabbccddeeff00112233445566778899"
        )
        .is_ok());
    }
    #[test]
    fn test_sha256_uppercase() {
        assert!(validate_sha256_hex(
            "AABBCCDDEEFF00112233445566778899AABBCCDDEEFF00112233445566778899"
        )
        .is_err());
    }
    #[test]
    fn test_sha256_wrong_length() {
        assert!(validate_sha256_hex("aabbcc").is_err());
    }

    // --- validate_emoji_name ---
    #[test]
    fn test_emoji_name_ok() {
        assert!(validate_emoji_name("cat-jam_2").is_ok());
    }
    #[test]
    fn test_emoji_name_empty() {
        assert!(validate_emoji_name("").is_err());
    }
    #[test]
    fn test_emoji_name_uppercase() {
        assert!(validate_emoji_name("Cat").is_err());
    }
    #[test]
    fn test_emoji_name_space() {
        assert!(validate_emoji_name("cat jam").is_err());
    }

    // --- validate_body_bytes ---
    #[test]
    fn test_body_ok() {
        assert!(validate_body_bytes("hello", 100).is_ok());
    }
    #[test]
    fn test_body_too_large() {
        assert!(validate_body_bytes("hello world", 5).is_err());
    }
    #[test]
    fn test_body_exact() {
        assert!(validate_body_bytes("hello", 5).is_ok());
    }
}
