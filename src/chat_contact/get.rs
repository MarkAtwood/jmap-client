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

use crate::{chat_types::Endpoint, core::get::GetObject, Get, Set};

use super::ChatContact;

impl ChatContact<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn take_id(&mut self) -> String {
        self.id.take().unwrap_or_default()
    }

    pub fn login(&self) -> Option<&str> {
        self.login.as_deref()
    }

    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    pub fn first_seen_at(&self) -> Option<&str> {
        self.first_seen_at.as_deref()
    }

    pub fn last_seen_at(&self) -> Option<&str> {
        self.last_seen_at.as_deref()
    }

    pub fn presence(&self) -> Option<&str> {
        self.presence.as_deref()
    }

    pub fn last_active_at(&self) -> Option<&str> {
        self.last_active_at.as_deref()
    }

    pub fn status_text(&self) -> Option<&str> {
        self.status_text.as_deref()
    }

    pub fn status_emoji(&self) -> Option<&str> {
        self.status_emoji.as_deref()
    }

    pub fn endpoints(&self) -> Option<&[Endpoint]> {
        self.endpoints.as_deref()
    }

    pub fn blocked(&self) -> bool {
        self.blocked.unwrap_or(false)
    }
}

impl GetObject for ChatContact<Set> {
    type GetArguments = ();
}

impl GetObject for ChatContact<Get> {
    type GetArguments = ();
}
