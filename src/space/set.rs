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

use crate::core::set::SetObject;
use crate::{Get, Set};

use super::Space;

impl Space<Set> {
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    pub fn icon_blob_id(&mut self, icon_blob_id: impl Into<String>) -> &mut Self {
        self.icon_blob_id = Some(icon_blob_id.into());
        self
    }

    pub fn is_public(&mut self, is_public: bool) -> &mut Self {
        self.is_public = Some(is_public);
        self
    }

    pub fn is_publicly_previewable(&mut self, is_publicly_previewable: bool) -> &mut Self {
        self.is_publicly_previewable = Some(is_publicly_previewable);
        self
    }
}

impl SetObject for Space<Set> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        Space {
            _create_id,
            _state: Default::default(),
            id: None,
            name: None,
            description: None,
            icon_blob_id: None,
            roles: None,
            members: None,
            categories: None,
            uncategorized_channel_ids: None,
            created_at: None,
            is_public: None,
            is_publicly_previewable: None,
            member_count: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for Space<Get> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
