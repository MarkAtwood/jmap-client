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

use crate::{core::set::SetObject, Get, Set};

use super::ChatContact;

impl ChatContact<Set> {
    pub fn display_name(&mut self, display_name: impl Into<String>) -> &mut Self {
        self.display_name = Some(display_name.into());
        self
    }

    pub fn blocked(&mut self, blocked: bool) -> &mut Self {
        self.blocked = Some(blocked);
        self
    }
}

impl SetObject for ChatContact<Set> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        ChatContact {
            _create_id,
            _state: Default::default(),
            id: None,
            login: None,
            display_name: None,
            first_seen_at: None,
            last_seen_at: None,
            presence: None,
            last_active_at: None,
            status_text: None,
            status_emoji: None,
            endpoints: None,
            blocked: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for ChatContact<Get> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
