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

//! Embedded sub-types for the JMAP Chat extension.
//!
//! These types are embedded within top-level JMAP Chat objects (Message, Chat,
//! Space, Channel, ChatContact) and are not top-level JMAP objects themselves.

use serde::{Deserialize, Serialize};

/// Metadata for a file blob associated with a Message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Attachment {
    #[serde(rename = "blobId")]
    pub blob_id: String,
    pub filename: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub size: u64,
    pub sha256: String,
}

/// Out-of-band capability reachable at a URI (e.g. a video-call room).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Endpoint {
    #[serde(rename = "type")]
    pub endpoint_type: String,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Per-message out-of-band action invitation (e.g. an approval button).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Identifies a user referenced within a message body.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Mention {
    pub id: String,
    pub offset: u64,
    pub length: u64,
}

/// One historical version of a Message body (edit history entry).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageRevision {
    pub body: String,
    #[serde(rename = "bodyType")]
    pub body_type: String,
    #[serde(rename = "editedAt")]
    pub edited_at: String,
}

/// Emoji response to a Message, keyed by senderReactionId in the reactions map.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reaction {
    pub emoji: String,
    #[serde(rename = "customEmojiId", skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    #[serde(rename = "senderId")]
    pub sender_id: String,
    #[serde(rename = "sentAt")]
    pub sent_at: String,
}

/// One participant in a group Chat.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMember {
    pub id: String,
    /// "admin" or "member" — open string for forward compatibility.
    pub role: String,
    #[serde(rename = "joinedAt")]
    pub joined_at: String,
    #[serde(rename = "invitedBy", skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<String>,
}

/// A named set of permissions within a Space.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpaceRole {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    pub permissions: Vec<String>,
    pub position: u64,
}

/// One participant in a Space.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpaceMember {
    pub id: String,
    #[serde(rename = "roleIds")]
    pub role_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    #[serde(rename = "joinedAt")]
    pub joined_at: String,
}

/// A named grouping of channels within a Space.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub position: u64,
    #[serde(rename = "channelIds")]
    pub channel_ids: Vec<String>,
}

/// Overrides Space-level role permissions for a specific channel.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelPermission {
    #[serde(rename = "targetId")]
    pub target_id: String,
    /// "role" or "member" — open string for forward compatibility.
    #[serde(rename = "targetType")]
    pub target_type: String,
    pub allow: Vec<String>,
    pub deny: Vec<String>,
}

/// One segment in a rich-body message (`application/jmap-chat-rich`).
///
/// Clients MUST silently ignore unknown `span_type` values.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Span {
    /// Segment type: "text", "bold", "italic", "bold-italic", "code",
    /// "codeblock", "blockquote", "mention", or "link".
    /// Open string: clients MUST silently ignore unknown types.
    #[serde(rename = "type")]
    pub span_type: String,
    pub text: String,
    /// For "codeblock": syntax highlighting hint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// For "mention": the mentioned user's ChatContact.id.
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// For "link": the target URI (treat as untrusted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// Top-level container for `application/jmap-chat-rich` body content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RichBody {
    pub spans: Vec<Span>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attachment_deserialize() {
        // Oracle: JSON with all fields matching spec field names exactly
        let json = r#"{
            "blobId": "blob1",
            "filename": "photo.jpg",
            "contentType": "image/jpeg",
            "size": 12345,
            "sha256": "aabbcc"
        }"#;
        let a: Attachment = serde_json::from_str(json).unwrap();
        assert_eq!(a.blob_id, "blob1");
        assert_eq!(a.filename, "photo.jpg");
        assert_eq!(a.size, 12345);
    }

    #[test]
    fn test_endpoint_optional_fields_absent() {
        let json = r#"{"type":"urn:jmap:chat:cap:vtc","uri":"https://meet.example.com/room1"}"#;
        let e: Endpoint = serde_json::from_str(json).unwrap();
        assert_eq!(e.endpoint_type, "urn:jmap:chat:cap:vtc");
        assert_eq!(e.label, None);
        assert_eq!(e.metadata, None);
    }

    #[test]
    fn test_mention_fields() {
        let json = r#"{"id":"user1","offset":5,"length":6}"#;
        let m: Mention = serde_json::from_str(json).unwrap();
        assert_eq!(m.id, "user1");
        assert_eq!(m.offset, 5);
        assert_eq!(m.length, 6);
    }

    #[test]
    fn test_reaction_deserialize() {
        let json = r#"{"emoji":"👍","senderId":"self","sentAt":"2026-01-01T00:00:00Z"}"#;
        let r: Reaction = serde_json::from_str(json).unwrap();
        assert_eq!(r.emoji, "👍");
        assert_eq!(r.sender_id, "self");
        assert_eq!(r.custom_emoji_id, None);
    }

    #[test]
    fn test_channel_permission_roundtrip() {
        let cp = ChannelPermission {
            target_id: "role1".to_string(),
            target_type: "role".to_string(),
            allow: vec!["send".to_string()],
            deny: vec!["pin".to_string()],
        };
        let json = serde_json::to_string(&cp).unwrap();
        let cp2: ChannelPermission = serde_json::from_str(&json).unwrap();
        assert_eq!(cp, cp2);
    }

    #[test]
    fn test_rich_body_deserialize() {
        let json = r#"{
            "spans": [
                {"type": "text", "text": "Hello "},
                {"type": "mention", "text": "@alice", "userId": "user:alice@example.com"},
                {"type": "codeblock", "text": "fn main() {}", "lang": "rust"}
            ]
        }"#;
        let rb: RichBody = serde_json::from_str(json).unwrap();
        assert_eq!(rb.spans.len(), 3);
        assert_eq!(
            rb.spans[1].user_id.as_deref(),
            Some("user:alice@example.com")
        );
        assert_eq!(rb.spans[2].lang.as_deref(), Some("rust"));
    }

    #[test]
    fn test_span_skip_none_fields() {
        // Optional fields must be absent from serialized form when None
        let span = Span {
            span_type: "text".to_string(),
            text: "hello".to_string(),
            lang: None,
            user_id: None,
            uri: None,
        };
        let json = serde_json::to_string(&span).unwrap();
        assert!(!json.contains("lang"));
        assert!(!json.contains("userId"));
        assert!(!json.contains("uri"));
    }
}
