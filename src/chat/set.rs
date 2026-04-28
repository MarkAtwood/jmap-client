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
    core::set::SetObject,
    Get, Set,
};

use super::Chat;

impl Chat<Set> {
    /// Set the kind at create time only ("direct", "group", or "channel").
    pub fn kind(&mut self, kind: impl Into<String>) -> &mut Self {
        self.kind = Some(kind.into());
        self
    }

    /// Set contactId (direct chats only, immutable after create).
    pub fn contact_id(&mut self, contact_id: impl Into<String>) -> &mut Self {
        self.contact_id = Some(contact_id.into());
        self
    }

    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    pub fn avatar_blob_id(&mut self, avatar_blob_id: impl Into<String>) -> &mut Self {
        self.avatar_blob_id = Some(avatar_blob_id.into());
        self
    }

    pub fn members(&mut self, members: Vec<ChatMember>) -> &mut Self {
        self.members = Some(members);
        self
    }

    /// Set spaceId at create time only (channel chats only, immutable after create).
    pub fn space_id(&mut self, space_id: impl Into<String>) -> &mut Self {
        self.space_id = Some(space_id.into());
        self
    }

    pub fn category_id(&mut self, category_id: impl Into<String>) -> &mut Self {
        self.category_id = Some(category_id.into());
        self
    }

    pub fn position(&mut self, position: u64) -> &mut Self {
        self.position = Some(position);
        self
    }

    pub fn topic(&mut self, topic: impl Into<String>) -> &mut Self {
        self.topic = Some(topic.into());
        self
    }

    pub fn slow_mode_seconds(&mut self, slow_mode_seconds: u64) -> &mut Self {
        self.slow_mode_seconds = Some(slow_mode_seconds);
        self
    }

    pub fn permission_overrides(
        &mut self,
        permission_overrides: Vec<ChannelPermission>,
    ) -> &mut Self {
        self.permission_overrides = Some(permission_overrides);
        self
    }

    pub fn pinned_message_ids(&mut self, pinned_message_ids: Vec<String>) -> &mut Self {
        self.pinned_message_ids = Some(pinned_message_ids);
        self
    }

    pub fn muted(&mut self, muted: bool) -> &mut Self {
        self.muted = Some(muted);
        self
    }

    pub fn mute_until(&mut self, mute_until: impl Into<String>) -> &mut Self {
        self.mute_until = Some(mute_until.into());
        self
    }

    pub fn receive_typing_indicators(&mut self, receive_typing_indicators: bool) -> &mut Self {
        self.receive_typing_indicators = Some(receive_typing_indicators);
        self
    }

    pub fn receipt_sharing(&mut self, receipt_sharing: bool) -> &mut Self {
        self.receipt_sharing = Some(receipt_sharing);
        self
    }

    pub fn message_expiry_seconds(&mut self, message_expiry_seconds: u64) -> &mut Self {
        self.message_expiry_seconds = Some(message_expiry_seconds);
        self
    }
}

impl SetObject for Chat<Set> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        Chat {
            _create_id,
            _state: Default::default(),
            id: None,
            kind: None,
            contact_id: None,
            name: None,
            description: None,
            avatar_blob_id: None,
            members: None,
            space_id: None,
            category_id: None,
            position: None,
            topic: None,
            slow_mode_seconds: None,
            permission_overrides: None,
            created_at: None,
            last_message_at: None,
            unread_count: None,
            pinned_message_ids: None,
            muted: None,
            mute_until: None,
            receive_typing_indicators: None,
            receipt_sharing: None,
            message_expiry_seconds: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for Chat<Get> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
