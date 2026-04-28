pub mod get;
pub mod helpers;
pub mod set;

use crate::core::changes::ChangesObject;
use crate::core::Object;
use crate::{Get, Set};
use serde::{Deserialize, Serialize};

/// PresenceStatus represents the owner's self-reported availability.
/// There is exactly one PresenceStatus record per account (singleton).
/// Create and destroy are forbidden; only update is supported.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceStatus<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,
    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    // One of "online"/"away"/"busy"/"invisible"/"offline"; open string for forward compat
    #[serde(rename = "presence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<String>,

    #[serde(rename = "statusText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,

    #[serde(rename = "statusEmoji")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_emoji: Option<String>,

    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    #[serde(rename = "receiptSharing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_sharing: Option<bool>,

    // Server-set: never sent by client; skip if None keeps it out of set payloads
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "presence")]
    Presence,
    #[serde(rename = "statusText")]
    StatusText,
    #[serde(rename = "statusEmoji")]
    StatusEmoji,
    #[serde(rename = "expiresAt")]
    ExpiresAt,
    #[serde(rename = "receiptSharing")]
    ReceiptSharing,
    #[serde(rename = "updatedAt")]
    UpdatedAt,
}

impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::Presence => write!(f, "presence"),
            Property::StatusText => write!(f, "statusText"),
            Property::StatusEmoji => write!(f, "statusEmoji"),
            Property::ExpiresAt => write!(f, "expiresAt"),
            Property::ReceiptSharing => write!(f, "receiptSharing"),
            Property::UpdatedAt => write!(f, "updatedAt"),
        }
    }
}

impl Object for PresenceStatus<Set> {
    type Property = Property;
    fn requires_account_id() -> bool {
        true
    }
}
impl Object for PresenceStatus<Get> {
    type Property = Property;
    fn requires_account_id() -> bool {
        true
    }
}
impl ChangesObject for PresenceStatus<Set> {
    type ChangesResponse = ();
}
impl ChangesObject for PresenceStatus<Get> {
    type ChangesResponse = ();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::set::SetObject;

    #[test]
    fn test_presence_status_get_deserialize() {
        let json = r#"{"id":"ps1","presence":"online","statusText":"Working","statusEmoji":"💻","receiptSharing":true,"updatedAt":"2026-01-01T00:00:00Z"}"#;
        let ps: PresenceStatus<Get> = serde_json::from_str(json).unwrap();
        assert_eq!(ps.id.as_deref(), Some("ps1"));
        assert_eq!(ps.presence.as_deref(), Some("online"));
        assert_eq!(ps.status_text.as_deref(), Some("Working"));
        assert_eq!(ps.receipt_sharing, Some(true));
    }

    #[test]
    fn test_presence_status_get_minimal() {
        let json = r#"{"id":"ps1","presence":"away","receiptSharing":false,"updatedAt":"2026-01-01T00:00:00Z"}"#;
        let ps: PresenceStatus<Get> = serde_json::from_str(json).unwrap();
        assert_eq!(ps.status_text, None);
        assert_eq!(ps.status_emoji, None);
        assert_eq!(ps.expires_at, None);
    }

    #[test]
    fn test_presence_status_set_skip_server_fields() {
        // In Set state, id and updatedAt start None and stay None (no setter exists)
        // so they won't appear in serialized output
        let ps = PresenceStatus::<Set>::new(None);
        let json = serde_json::to_string(&ps).unwrap();
        assert!(
            !json.contains("\"id\""),
            "id should not be serialized: {}",
            json
        );
        assert!(
            !json.contains("\"updatedAt\""),
            "updatedAt should not be serialized: {}",
            json
        );
    }

    #[test]
    fn test_presence_status_set_update_serialize() {
        let mut ps = PresenceStatus::<Set>::new(None);
        ps.presence("busy").status_text("In a meeting");
        let json = serde_json::to_string(&ps).unwrap();
        assert!(json.contains("\"presence\":\"busy\""), "{}", json);
        assert!(json.contains("\"statusText\":\"In a meeting\""), "{}", json);
        assert!(!json.contains("\"updatedAt\""), "{}", json);
    }
}
