pub mod get;
pub mod helpers;
pub mod set;

use crate::core::changes::ChangesObject;
use crate::core::Object;
use crate::{Get, Set};
use serde::{Deserialize, Serialize};

/// SpaceInvite grants a new member access to a Space via a shared invite code.
/// create: accepts spaceId (required), defaultChannelId/expiresAt/maxUses (optional).
/// destroy: revokes an invite.
/// update: NOT supported (spec says any update MUST return forbidden).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceInvite<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,
    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    // Server-set immutable
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    // Server-set: the shareable invite code string
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    // Immutable: set at create
    #[serde(rename = "spaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,

    #[serde(rename = "defaultChannelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_channel_id: Option<String>,

    // Server-set immutable
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,

    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,

    #[serde(rename = "maxUses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<u64>,

    // Server-set: number of times redeemed
    #[serde(rename = "uses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uses: Option<u64>,

    // Server-set immutable
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "code")]
    Code,
    #[serde(rename = "spaceId")]
    SpaceId,
    #[serde(rename = "defaultChannelId")]
    DefaultChannelId,
    #[serde(rename = "createdBy")]
    CreatedBy,
    #[serde(rename = "expiresAt")]
    ExpiresAt,
    #[serde(rename = "maxUses")]
    MaxUses,
    #[serde(rename = "uses")]
    Uses,
    #[serde(rename = "createdAt")]
    CreatedAt,
}

impl std::fmt::Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::Code => write!(f, "code"),
            Property::SpaceId => write!(f, "spaceId"),
            Property::DefaultChannelId => write!(f, "defaultChannelId"),
            Property::CreatedBy => write!(f, "createdBy"),
            Property::ExpiresAt => write!(f, "expiresAt"),
            Property::MaxUses => write!(f, "maxUses"),
            Property::Uses => write!(f, "uses"),
            Property::CreatedAt => write!(f, "createdAt"),
        }
    }
}

impl Object for SpaceInvite<Set> {
    type Property = Property;
    fn requires_account_id() -> bool {
        true
    }
}
impl Object for SpaceInvite<Get> {
    type Property = Property;
    fn requires_account_id() -> bool {
        true
    }
}
impl ChangesObject for SpaceInvite<Set> {
    type ChangesResponse = ();
}
impl ChangesObject for SpaceInvite<Get> {
    type ChangesResponse = ();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::set::SetObject;

    #[test]
    fn test_space_invite_get_deserialize() {
        let json = r#"{"id":"inv1","code":"abc123","spaceId":"sp1","createdBy":"u1","uses":3,"maxUses":10,"createdAt":"2026-01-01T00:00:00Z"}"#;
        let inv: SpaceInvite<Get> = serde_json::from_str(json).unwrap();
        assert_eq!(inv.id.as_deref(), Some("inv1"));
        assert_eq!(inv.code.as_deref(), Some("abc123"));
        assert_eq!(inv.uses, Some(3));
        assert_eq!(inv.max_uses, Some(10));
    }

    #[test]
    fn test_space_invite_set_skip_server_fields() {
        let inv = SpaceInvite::<Set>::new(None);
        let json = serde_json::to_string(&inv).unwrap();
        assert!(!json.contains("\"id\""), "{}", json);
        assert!(!json.contains("\"code\""), "{}", json);
        assert!(!json.contains("\"createdBy\""), "{}", json);
        assert!(!json.contains("\"uses\""), "{}", json);
        assert!(!json.contains("\"createdAt\""), "{}", json);
    }

    #[test]
    fn test_space_invite_create_serialize() {
        let mut inv = SpaceInvite::<Set>::new(Some(0));
        inv.space_id("sp1").max_uses(5);
        let json = serde_json::to_string(&inv).unwrap();
        assert!(json.contains("\"spaceId\":\"sp1\""), "{}", json);
        assert!(json.contains("\"maxUses\":5"), "{}", json);
        assert!(!json.contains("\"id\""), "{}", json);
        assert!(!json.contains("\"code\""), "{}", json);
    }
}
