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

use crate::{
    chat_types::{ChannelPermission, ChatMember},
    core::get::GetObject,
    Get, Set,
};

use super::Chat;

impl Chat<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn take_id(&mut self) -> String {
        self.id.take().unwrap_or_default()
    }

    pub fn kind(&self) -> Option<&str> {
        self.kind.as_deref()
    }

    pub fn contact_id(&self) -> Option<&str> {
        self.contact_id.as_deref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn avatar_blob_id(&self) -> Option<&str> {
        self.avatar_blob_id.as_deref()
    }

    pub fn members(&self) -> Option<&[ChatMember]> {
        self.members.as_deref()
    }

    pub fn space_id(&self) -> Option<&str> {
        self.space_id.as_deref()
    }

    pub fn category_id(&self) -> Option<&str> {
        self.category_id.as_deref()
    }

    pub fn position(&self) -> Option<u64> {
        self.position
    }

    pub fn topic(&self) -> Option<&str> {
        self.topic.as_deref()
    }

    pub fn slow_mode_seconds(&self) -> Option<u64> {
        self.slow_mode_seconds
    }

    pub fn permission_overrides(&self) -> Option<&[ChannelPermission]> {
        self.permission_overrides.as_deref()
    }

    pub fn created_at(&self) -> Option<&str> {
        self.created_at.as_deref()
    }

    pub fn last_message_at(&self) -> Option<&str> {
        self.last_message_at.as_deref()
    }

    pub fn unread_count(&self) -> Option<u64> {
        self.unread_count
    }

    pub fn pinned_message_ids(&self) -> Option<&[String]> {
        self.pinned_message_ids.as_deref()
    }

    pub fn muted(&self) -> Option<bool> {
        self.muted
    }

    pub fn mute_until(&self) -> Option<&str> {
        self.mute_until.as_deref()
    }

    pub fn receive_typing_indicators(&self) -> Option<bool> {
        self.receive_typing_indicators
    }

    pub fn receipt_sharing(&self) -> Option<bool> {
        self.receipt_sharing
    }

    pub fn message_expiry_seconds(&self) -> Option<u64> {
        self.message_expiry_seconds
    }
}

impl GetObject for Chat<Set> {
    type GetArguments = ();
}

impl GetObject for Chat<Get> {
    type GetArguments = ();
}
