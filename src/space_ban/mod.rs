pub mod get;
pub mod helpers;
pub mod set;

use crate::core::changes::ChangesObject;
use crate::core::Object;
use crate::{Get, Set};
use serde::{Deserialize, Serialize};

/// SpaceBan prevents a user from participating in a Space.
/// create accepts spaceId, userId, reason (opt), expiresAt (opt).
/// update supports reason and expiresAt. destroy lifts the ban.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceBan<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,
    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    // Server-set: skip when None to exclude from set payloads
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    // Immutable: set at create, never updated
    #[serde(rename = "spaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,

    // Immutable: set at create, never updated
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    // Server-set: bannedBy is set from authenticated identity, never sent by client
    #[serde(rename = "bannedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned_by: Option<String>,

    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    // Server-set immutable
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "spaceId")]
    SpaceId,
    #[serde(rename = "userId")]
    UserId,
    #[serde(rename = "bannedBy")]
    BannedBy,
    #[serde(rename = "reason")]
    Reason,
    #[serde(rename = "createdAt")]
    CreatedAt,
    #[serde(rename = "expiresAt")]
    ExpiresAt,
}

impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::SpaceId => write!(f, "spaceId"),
            Property::UserId => write!(f, "userId"),
            Property::BannedBy => write!(f, "bannedBy"),
            Property::Reason => write!(f, "reason"),
            Property::CreatedAt => write!(f, "createdAt"),
            Property::ExpiresAt => write!(f, "expiresAt"),
        }
    }
}

impl Object for SpaceBan<Set> {
    type Property = Property;
    fn requires_account_id() -> bool {
        true
    }
}
impl Object for SpaceBan<Get> {
    type Property = Property;
    fn requires_account_id() -> bool {
        true
    }
}
impl ChangesObject for SpaceBan<Set> {
    type ChangesResponse = ();
}
impl ChangesObject for SpaceBan<Get> {
    type ChangesResponse = ();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::set::SetObject;

    #[test]
    fn test_space_ban_get_deserialize() {
        let json = r#"{"id":"ban1","spaceId":"sp1","userId":"u2","bannedBy":"u1","reason":"spam","createdAt":"2026-01-01T00:00:00Z"}"#;
        let ban: SpaceBan<Get> = serde_json::from_str(json).unwrap();
        assert_eq!(ban.id.as_deref(), Some("ban1"));
        assert_eq!(ban.space_id.as_deref(), Some("sp1"));
        assert_eq!(ban.user_id.as_deref(), Some("u2"));
        assert_eq!(ban.banned_by.as_deref(), Some("u1"));
        assert_eq!(ban.reason.as_deref(), Some("spam"));
        assert_eq!(ban.expires_at, None);
    }

    #[test]
    fn test_space_ban_set_skip_server_fields() {
        let ban = SpaceBan::<Set>::new(None);
        let json = serde_json::to_string(&ban).unwrap();
        assert!(!json.contains("\"id\""), "{}", json);
        assert!(!json.contains("\"bannedBy\""), "{}", json);
        assert!(!json.contains("\"createdAt\""), "{}", json);
    }

    #[test]
    fn test_space_ban_set_create_fields() {
        let mut ban = SpaceBan::<Set>::new(Some(0));
        ban.space_id("sp1").user_id("u2").reason("spam");
        let json = serde_json::to_string(&ban).unwrap();
        assert!(json.contains("\"spaceId\":\"sp1\""), "{}", json);
        assert!(json.contains("\"userId\":\"u2\""), "{}", json);
        assert!(json.contains("\"reason\":\"spam\""), "{}", json);
    }
}
