pub mod get;
pub mod helpers;
pub mod set;

use crate::core::changes::ChangesObject;
use crate::core::Object;
use crate::{Get, Set};
use serde::{Deserialize, Serialize};

/// ReadPosition tracks the owner's read state within a Chat (per-chat high-water mark).
/// The server creates/destroys these automatically. Only update (lastReadMessageId) is supported.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadPosition<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,
    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    // Immutable: set at creation, never sent in update payloads
    #[serde(rename = "chatId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,

    #[serde(rename = "lastReadMessageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_read_message_id: Option<String>,

    // Server-set: updated automatically when lastReadMessageId changes
    #[serde(rename = "lastReadAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_read_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "chatId")]
    ChatId,
    #[serde(rename = "lastReadMessageId")]
    LastReadMessageId,
    #[serde(rename = "lastReadAt")]
    LastReadAt,
}

impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::ChatId => write!(f, "chatId"),
            Property::LastReadMessageId => write!(f, "lastReadMessageId"),
            Property::LastReadAt => write!(f, "lastReadAt"),
        }
    }
}

impl Object for ReadPosition<Set> {
    type Property = Property;
    fn requires_account_id() -> bool {
        true
    }
}
impl Object for ReadPosition<Get> {
    type Property = Property;
    fn requires_account_id() -> bool {
        true
    }
}
impl ChangesObject for ReadPosition<Set> {
    type ChangesResponse = ();
}
impl ChangesObject for ReadPosition<Get> {
    type ChangesResponse = ();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::set::SetObject;

    #[test]
    fn test_read_position_get_deserialize() {
        let json = r#"{"id":"rp1","chatId":"c1","lastReadMessageId":"msg42","lastReadAt":"2026-01-01T00:00:00Z"}"#;
        let rp: ReadPosition<Get> = serde_json::from_str(json).unwrap();
        assert_eq!(rp.id.as_deref(), Some("rp1"));
        assert_eq!(rp.chat_id.as_deref(), Some("c1"));
        assert_eq!(rp.last_read_message_id.as_deref(), Some("msg42"));
    }

    #[test]
    fn test_read_position_get_no_last_read() {
        let json = r#"{"id":"rp2","chatId":"c2","lastReadAt":"2026-01-01T00:00:00Z"}"#;
        let rp: ReadPosition<Get> = serde_json::from_str(json).unwrap();
        assert_eq!(rp.last_read_message_id, None);
    }

    #[test]
    fn test_read_position_set_skip_server_fields() {
        let rp = ReadPosition::<Set>::new(None);
        let json = serde_json::to_string(&rp).unwrap();
        // All None at start → empty JSON object
        assert!(!json.contains("\"id\""), "{}", json);
        assert!(!json.contains("\"chatId\""), "{}", json);
        assert!(!json.contains("\"lastReadAt\""), "{}", json);
    }

    #[test]
    fn test_read_position_set_update_only_last_read() {
        let mut rp = ReadPosition::<Set>::new(None);
        rp.last_read_message_id("msg42");
        let json = serde_json::to_string(&rp).unwrap();
        assert!(json.contains("\"lastReadMessageId\":\"msg42\""), "{}", json);
        assert!(!json.contains("\"id\""), "{}", json);
        assert!(!json.contains("\"lastReadAt\""), "{}", json);
    }
}
