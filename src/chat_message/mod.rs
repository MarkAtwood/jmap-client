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

use ahash::AHashMap;

use crate::chat_types::{Attachment, Mention, MessageAction, MessageRevision, Reaction};
use crate::core::changes::ChangesObject;
use crate::core::Object;
use crate::{Get, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    // Server-set — no setter
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "senderMsgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_msg_id: Option<String>,

    #[serde(rename = "senderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,

    #[serde(rename = "receivedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<String>,

    #[serde(rename = "replyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<u64>,

    #[serde(rename = "unreadReplyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread_reply_count: Option<u64>,

    #[serde(rename = "deliveryState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_state: Option<String>,

    #[serde(rename = "deliveryReceipts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_receipts: Option<serde_json::Value>,

    #[serde(rename = "deliveredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<String>,

    #[serde(rename = "readAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_at: Option<String>,

    #[serde(rename = "editedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_at: Option<String>,

    #[serde(rename = "editHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_history: Option<Vec<MessageRevision>>,

    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,

    #[serde(rename = "deletedForAll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_for_all: Option<bool>,

    // Immutable — create-only setter
    #[serde(rename = "chatId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<String>,

    #[serde(rename = "senderExpiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_expires_at: Option<String>,

    #[serde(rename = "burnOnRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burn_on_read: Option<bool>,

    // Client-mutable — has setter
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    #[serde(rename = "bodyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_type: Option<String>,

    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,

    #[serde(rename = "mentions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<Mention>>,

    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<MessageAction>>,

    #[serde(rename = "reactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<AHashMap<String, Reaction>>,

    #[serde(rename = "replyTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,

    #[serde(rename = "threadRootId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_root_id: Option<String>,

    #[serde(rename = "sentAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "senderMsgId")]
    SenderMsgId,
    #[serde(rename = "chatId")]
    ChatId,
    #[serde(rename = "senderId")]
    SenderId,
    #[serde(rename = "body")]
    Body,
    #[serde(rename = "bodyType")]
    BodyType,
    #[serde(rename = "attachments")]
    Attachments,
    #[serde(rename = "mentions")]
    Mentions,
    #[serde(rename = "actions")]
    Actions,
    #[serde(rename = "reactions")]
    Reactions,
    #[serde(rename = "replyTo")]
    ReplyTo,
    #[serde(rename = "threadRootId")]
    ThreadRootId,
    #[serde(rename = "replyCount")]
    ReplyCount,
    #[serde(rename = "unreadReplyCount")]
    UnreadReplyCount,
    #[serde(rename = "sentAt")]
    SentAt,
    #[serde(rename = "receivedAt")]
    ReceivedAt,
    #[serde(rename = "senderExpiresAt")]
    SenderExpiresAt,
    #[serde(rename = "burnOnRead")]
    BurnOnRead,
    #[serde(rename = "deliveryState")]
    DeliveryState,
    #[serde(rename = "deliveryReceipts")]
    DeliveryReceipts,
    #[serde(rename = "deliveredAt")]
    DeliveredAt,
    #[serde(rename = "readAt")]
    ReadAt,
    #[serde(rename = "editedAt")]
    EditedAt,
    #[serde(rename = "editHistory")]
    EditHistory,
    #[serde(rename = "deletedAt")]
    DeletedAt,
    #[serde(rename = "deletedForAll")]
    DeletedForAll,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::SenderMsgId => write!(f, "senderMsgId"),
            Property::ChatId => write!(f, "chatId"),
            Property::SenderId => write!(f, "senderId"),
            Property::Body => write!(f, "body"),
            Property::BodyType => write!(f, "bodyType"),
            Property::Attachments => write!(f, "attachments"),
            Property::Mentions => write!(f, "mentions"),
            Property::Actions => write!(f, "actions"),
            Property::Reactions => write!(f, "reactions"),
            Property::ReplyTo => write!(f, "replyTo"),
            Property::ThreadRootId => write!(f, "threadRootId"),
            Property::ReplyCount => write!(f, "replyCount"),
            Property::UnreadReplyCount => write!(f, "unreadReplyCount"),
            Property::SentAt => write!(f, "sentAt"),
            Property::ReceivedAt => write!(f, "receivedAt"),
            Property::SenderExpiresAt => write!(f, "senderExpiresAt"),
            Property::BurnOnRead => write!(f, "burnOnRead"),
            Property::DeliveryState => write!(f, "deliveryState"),
            Property::DeliveryReceipts => write!(f, "deliveryReceipts"),
            Property::DeliveredAt => write!(f, "deliveredAt"),
            Property::ReadAt => write!(f, "readAt"),
            Property::EditedAt => write!(f, "editedAt"),
            Property::EditHistory => write!(f, "editHistory"),
            Property::DeletedAt => write!(f, "deletedAt"),
            Property::DeletedForAll => write!(f, "deletedForAll"),
        }
    }
}

impl Object for Message<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for Message<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for Message<Set> {
    type ChangesResponse = ();
}

impl ChangesObject for Message<Get> {
    type ChangesResponse = ();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::set::SetObject;

    #[test]
    fn test_message_get_deserialize() {
        let json = r#"{
            "id": "msg1",
            "chatId": "chat1",
            "senderId": "self",
            "body": "Hello world",
            "bodyType": "text/plain",
            "deliveryState": "delivered",
            "reactions": {
                "rxn1": {"emoji": "👍", "senderId": "self", "sentAt": "2026-01-01T00:00:00Z"}
            }
        }"#;
        let msg: Message<Get> = serde_json::from_str(json).unwrap();
        assert_eq!(msg.id.as_deref(), Some("msg1"));
        assert_eq!(msg.chat_id.as_deref(), Some("chat1"));
        assert_eq!(msg.sender_id.as_deref(), Some("self"));
        assert_eq!(msg.body.as_deref(), Some("Hello world"));
        assert_eq!(msg.body_type.as_deref(), Some("text/plain"));
        assert_eq!(msg.delivery_state.as_deref(), Some("delivered"));
        let reactions = msg.reactions.as_ref().unwrap();
        assert_eq!(reactions.len(), 1);
        let rxn = reactions.get("rxn1").unwrap();
        assert_eq!(rxn.emoji, "👍");
        assert_eq!(rxn.sender_id, "self");
    }

    #[test]
    fn test_message_set_skip_server_fields() {
        let msg = Message::<Set>::new(None);
        let json = serde_json::to_string(&msg).unwrap();
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"senderMsgId\""));
        assert!(!json.contains("\"senderId\""));
        assert!(!json.contains("\"receivedAt\""));
        assert!(!json.contains("\"deliveryState\""));
        assert!(!json.contains("\"editedAt\""));
        assert!(!json.contains("\"deletedAt\""));
    }

    #[test]
    fn test_message_reaction_deserialize() {
        let json = r#"{"emoji":"👍","senderId":"self","sentAt":"2026-01-01T00:00:00Z"}"#;
        let r: Reaction = serde_json::from_str(json).unwrap();
        assert_eq!(r.emoji, "👍");
        assert_eq!(r.sender_id, "self");
        assert_eq!(r.sent_at, "2026-01-01T00:00:00Z");
        assert_eq!(r.custom_emoji_id, None);
    }
}
