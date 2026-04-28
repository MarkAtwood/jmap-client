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

use crate::chat_types::{Attachment, Mention, MessageAction, MessageRevision, Reaction};
use crate::core::get::GetObject;
use crate::{Get, Set};

use super::Message;

impl Message<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn take_id(&mut self) -> String {
        self.id.take().unwrap_or_default()
    }

    pub fn sender_msg_id(&self) -> Option<&str> {
        self.sender_msg_id.as_deref()
    }

    pub fn chat_id(&self) -> Option<&str> {
        self.chat_id.as_deref()
    }

    pub fn sender_id(&self) -> Option<&str> {
        self.sender_id.as_deref()
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    pub fn body_type(&self) -> Option<&str> {
        self.body_type.as_deref()
    }

    pub fn attachments(&self) -> Option<&[Attachment]> {
        self.attachments.as_deref()
    }

    pub fn mentions(&self) -> Option<&[Mention]> {
        self.mentions.as_deref()
    }

    pub fn actions(&self) -> Option<&[MessageAction]> {
        self.actions.as_deref()
    }

    pub fn reactions(&self) -> Option<&AHashMap<String, Reaction>> {
        self.reactions.as_ref()
    }

    pub fn reply_to(&self) -> Option<&str> {
        self.reply_to.as_deref()
    }

    pub fn thread_root_id(&self) -> Option<&str> {
        self.thread_root_id.as_deref()
    }

    pub fn reply_count(&self) -> Option<u64> {
        self.reply_count
    }

    pub fn unread_reply_count(&self) -> Option<u64> {
        self.unread_reply_count
    }

    pub fn sent_at(&self) -> Option<&str> {
        self.sent_at.as_deref()
    }

    pub fn received_at(&self) -> Option<&str> {
        self.received_at.as_deref()
    }

    pub fn sender_expires_at(&self) -> Option<&str> {
        self.sender_expires_at.as_deref()
    }

    pub fn burn_on_read(&self) -> Option<bool> {
        self.burn_on_read
    }

    pub fn delivery_state(&self) -> Option<&str> {
        self.delivery_state.as_deref()
    }

    pub fn delivery_receipts(&self) -> Option<&serde_json::Value> {
        self.delivery_receipts.as_ref()
    }

    pub fn delivered_at(&self) -> Option<&str> {
        self.delivered_at.as_deref()
    }

    pub fn read_at(&self) -> Option<&str> {
        self.read_at.as_deref()
    }

    pub fn edited_at(&self) -> Option<&str> {
        self.edited_at.as_deref()
    }

    pub fn edit_history(&self) -> Option<&[MessageRevision]> {
        self.edit_history.as_deref()
    }

    pub fn deleted_at(&self) -> Option<&str> {
        self.deleted_at.as_deref()
    }

    pub fn deleted_for_all(&self) -> Option<bool> {
        self.deleted_for_all
    }
}

impl GetObject for Message<Set> {
    type GetArguments = ();
}

impl GetObject for Message<Get> {
    type GetArguments = ();
}
