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

use ahash::AHashMap;

use crate::chat_types::{Attachment, Mention, MessageAction, Reaction};
use crate::core::set::SetObject;
use crate::{Get, Set};

use super::Message;

impl Message<Set> {
    // Client-mutable setters

    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        self.body = Some(body.into());
        self
    }

    pub fn body_type(&mut self, body_type: impl Into<String>) -> &mut Self {
        self.body_type = Some(body_type.into());
        self
    }

    pub fn attachments(&mut self, attachments: Vec<Attachment>) -> &mut Self {
        self.attachments = Some(attachments);
        self
    }

    pub fn mentions(&mut self, mentions: Vec<Mention>) -> &mut Self {
        self.mentions = Some(mentions);
        self
    }

    pub fn actions(&mut self, actions: Vec<MessageAction>) -> &mut Self {
        self.actions = Some(actions);
        self
    }

    pub fn reactions(&mut self, reactions: AHashMap<String, Reaction>) -> &mut Self {
        self.reactions = Some(reactions);
        self
    }

    pub fn reply_to(&mut self, reply_to: impl Into<String>) -> &mut Self {
        self.reply_to = Some(reply_to.into());
        self
    }

    pub fn thread_root_id(&mut self, thread_root_id: impl Into<String>) -> &mut Self {
        self.thread_root_id = Some(thread_root_id.into());
        self
    }

    pub fn sent_at(&mut self, sent_at: impl Into<String>) -> &mut Self {
        self.sent_at = Some(sent_at.into());
        self
    }

    // Immutable (create-only) setters

    pub fn chat_id(&mut self, chat_id: impl Into<String>) -> &mut Self {
        self.chat_id = Some(chat_id.into());
        self
    }

    pub fn sender_expires_at(&mut self, sender_expires_at: impl Into<String>) -> &mut Self {
        self.sender_expires_at = Some(sender_expires_at.into());
        self
    }

    pub fn burn_on_read(&mut self, burn_on_read: bool) -> &mut Self {
        self.burn_on_read = Some(burn_on_read);
        self
    }
}

impl SetObject for Message<Set> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        Message {
            _create_id,
            _state: Default::default(),
            id: None,
            sender_msg_id: None,
            sender_id: None,
            received_at: None,
            reply_count: None,
            unread_reply_count: None,
            delivery_state: None,
            delivery_receipts: None,
            delivered_at: None,
            read_at: None,
            edited_at: None,
            edit_history: None,
            deleted_at: None,
            deleted_for_all: None,
            chat_id: None,
            sender_expires_at: None,
            burn_on_read: None,
            body: None,
            body_type: None,
            attachments: None,
            mentions: None,
            actions: None,
            reactions: None,
            reply_to: None,
            thread_root_id: None,
            sent_at: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for Message<Get> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
