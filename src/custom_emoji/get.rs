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

use crate::{core::get::GetObject, Get, Set};

use super::CustomEmoji;

impl CustomEmoji<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn take_id(&mut self) -> String {
        self.id.take().unwrap_or_default()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn blob_id(&self) -> Option<&str> {
        self.blob_id.as_deref()
    }

    pub fn space_id(&self) -> Option<&str> {
        self.space_id.as_deref()
    }

    pub fn created_by(&self) -> Option<&str> {
        self.created_by.as_deref()
    }

    pub fn created_at(&self) -> Option<&str> {
        self.created_at.as_deref()
    }
}

impl GetObject for CustomEmoji<Set> {
    type GetArguments = ();
}

impl GetObject for CustomEmoji<Get> {
    type GetArguments = ();
}
