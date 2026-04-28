/*
 * Copyright Stalwart Labs LLC See the COPYING
 * file at the top-level directory of this distribution.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

pub mod get;
pub mod helpers;
pub mod query;
pub mod set;

use std::fmt::Display;

use crate::chat_types::{ChannelPermission, ChatMember};
use crate::core::changes::ChangesObject;
use crate::core::Object;
use crate::{Get, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    // Server-set immutable — no setter
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    // Immutable after create — setter only used at create time
    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    // Immutable, direct only
    #[serde(rename = "contactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,

    // Group/channel
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    // Group only
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    // Group only
    #[serde(rename = "avatarBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_blob_id: Option<String>,

    // Group only
    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<ChatMember>>,

    // Channel only, immutable
    #[serde(rename = "spaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,

    // Channel only
    #[serde(rename = "categoryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,

    // Channel only
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,

    // Channel only
    #[serde(rename = "topic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    // Channel only
    #[serde(rename = "slowModeSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_seconds: Option<u64>,

    // Channel only
    #[serde(rename = "permissionOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overrides: Option<Vec<ChannelPermission>>,

    // Server-set immutable — no setter
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    // Server-set — no setter
    #[serde(rename = "lastMessageAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_at: Option<String>,

    // Server-set — no setter
    #[serde(rename = "unreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread_count: Option<u64>,

    #[serde(rename = "pinnedMessageIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message_ids: Option<Vec<String>>,

    #[serde(rename = "muted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,

    #[serde(rename = "muteUntil")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_until: Option<String>,

    #[serde(rename = "receiveTypingIndicators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_typing_indicators: Option<bool>,

    #[serde(rename = "receiptSharing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_sharing: Option<bool>,

    #[serde(rename = "messageExpirySeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_expiry_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "kind")]
    Kind,
    #[serde(rename = "contactId")]
    ContactId,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "avatarBlobId")]
    AvatarBlobId,
    #[serde(rename = "members")]
    Members,
    #[serde(rename = "spaceId")]
    SpaceId,
    #[serde(rename = "categoryId")]
    CategoryId,
    #[serde(rename = "position")]
    Position,
    #[serde(rename = "topic")]
    Topic,
    #[serde(rename = "slowModeSeconds")]
    SlowModeSeconds,
    #[serde(rename = "permissionOverrides")]
    PermissionOverrides,
    #[serde(rename = "createdAt")]
    CreatedAt,
    #[serde(rename = "lastMessageAt")]
    LastMessageAt,
    #[serde(rename = "unreadCount")]
    UnreadCount,
    #[serde(rename = "pinnedMessageIds")]
    PinnedMessageIds,
    #[serde(rename = "muted")]
    Muted,
    #[serde(rename = "muteUntil")]
    MuteUntil,
    #[serde(rename = "receiveTypingIndicators")]
    ReceiveTypingIndicators,
    #[serde(rename = "receiptSharing")]
    ReceiptSharing,
    #[serde(rename = "messageExpirySeconds")]
    MessageExpirySeconds,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::Kind => write!(f, "kind"),
            Property::ContactId => write!(f, "contactId"),
            Property::Name => write!(f, "name"),
            Property::Description => write!(f, "description"),
            Property::AvatarBlobId => write!(f, "avatarBlobId"),
            Property::Members => write!(f, "members"),
            Property::SpaceId => write!(f, "spaceId"),
            Property::CategoryId => write!(f, "categoryId"),
            Property::Position => write!(f, "position"),
            Property::Topic => write!(f, "topic"),
            Property::SlowModeSeconds => write!(f, "slowModeSeconds"),
            Property::PermissionOverrides => write!(f, "permissionOverrides"),
            Property::CreatedAt => write!(f, "createdAt"),
            Property::LastMessageAt => write!(f, "lastMessageAt"),
            Property::UnreadCount => write!(f, "unreadCount"),
            Property::PinnedMessageIds => write!(f, "pinnedMessageIds"),
            Property::Muted => write!(f, "muted"),
            Property::MuteUntil => write!(f, "muteUntil"),
            Property::ReceiveTypingIndicators => write!(f, "receiveTypingIndicators"),
            Property::ReceiptSharing => write!(f, "receiptSharing"),
            Property::MessageExpirySeconds => write!(f, "messageExpirySeconds"),
        }
    }
}

impl Object for Chat<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for Chat<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for Chat<Set> {
    type ChangesResponse = ();
}

impl ChangesObject for Chat<Get> {
    type ChangesResponse = ();
}

/// Request body for the non-standard Chat/typing method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "chatId")]
    pub chat_id: String,
    pub typing: bool,
}

/// Response body for the non-standard Chat/typing method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingResponse {
    #[serde(rename = "accountId")]
    pub account_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_get_deserialize_direct() {
        let json = r#"{
            "id": "chat-1",
            "kind": "direct",
            "contactId": "contact-42",
            "createdAt": "2026-01-01T00:00:00Z"
        }"#;
        let chat: Chat<Get> = serde_json::from_str(json).unwrap();
        assert_eq!(chat.id.as_deref(), Some("chat-1"));
        assert_eq!(chat.kind.as_deref(), Some("direct"));
        assert_eq!(chat.contact_id.as_deref(), Some("contact-42"));
        assert_eq!(chat.created_at.as_deref(), Some("2026-01-01T00:00:00Z"));
    }

    #[test]
    fn test_chat_get_deserialize_group() {
        let json = r#"{
            "id": "chat-2",
            "kind": "group",
            "name": "Team Alpha",
            "members": [
                {"id": "u1", "role": "admin", "joinedAt": "2026-01-01T00:00:00Z"},
                {"id": "u2", "role": "member", "joinedAt": "2026-01-02T00:00:00Z"}
            ]
        }"#;
        let chat: Chat<Get> = serde_json::from_str(json).unwrap();
        assert_eq!(chat.kind.as_deref(), Some("group"));
        assert_eq!(chat.name.as_deref(), Some("Team Alpha"));
        let members = chat.members.as_ref().unwrap();
        assert_eq!(members.len(), 2);
        assert_eq!(members[0].id, "u1");
        assert_eq!(members[1].role, "member");
    }

    #[test]
    fn test_chat_set_skip_server_fields() {
        use crate::core::set::SetObject;
        let obj: Chat<Set> = Chat::new(None);
        let json = serde_json::to_string(&obj).unwrap();
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"createdAt\""));
        assert!(!json.contains("\"lastMessageAt\""));
        assert!(!json.contains("\"unreadCount\""));
    }

    #[test]
    fn test_typing_request_serialize() {
        let req = TypingRequest {
            account_id: "a1".to_string(),
            chat_id: "c1".to_string(),
            typing: true,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"accountId\":\"a1\""));
        assert!(json.contains("\"chatId\":\"c1\""));
        assert!(json.contains("\"typing\":true"));
    }

    #[test]
    fn test_typing_response_deserialize() {
        let json = r#"{"accountId":"a1"}"#;
        let resp: TypingResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.account_id, "a1");
    }
}
