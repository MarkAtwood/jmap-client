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

use crate::chat_types::{Category, SpaceMember, SpaceRole};
use crate::core::get::GetObject;
use crate::{Get, Set};

use super::Space;

impl Space<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn take_id(&mut self) -> String {
        self.id.take().unwrap_or_default()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn icon_blob_id(&self) -> Option<&str> {
        self.icon_blob_id.as_deref()
    }

    pub fn roles(&self) -> Option<&[SpaceRole]> {
        self.roles.as_deref()
    }

    pub fn members(&self) -> Option<&[SpaceMember]> {
        self.members.as_deref()
    }

    pub fn categories(&self) -> Option<&[Category]> {
        self.categories.as_deref()
    }

    pub fn uncategorized_channel_ids(&self) -> Option<&[String]> {
        self.uncategorized_channel_ids.as_deref()
    }

    pub fn created_at(&self) -> Option<&str> {
        self.created_at.as_deref()
    }

    pub fn is_public(&self) -> Option<bool> {
        self.is_public
    }

    pub fn is_publicly_previewable(&self) -> Option<bool> {
        self.is_publicly_previewable
    }

    pub fn member_count(&self) -> Option<u64> {
        self.member_count
    }
}

impl GetObject for Space<Set> {
    type GetArguments = ();
}

impl GetObject for Space<Get> {
    type GetArguments = ();
}
