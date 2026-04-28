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

use super::CustomEmoji;

pub fn validate_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

impl CustomEmoji<Set> {
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn blob_id(&mut self, blob_id: impl Into<String>) -> &mut Self {
        self.blob_id = Some(blob_id.into());
        self
    }

    pub fn space_id(&mut self, space_id: impl Into<String>) -> &mut Self {
        self.space_id = Some(space_id.into());
        self
    }
}

impl SetObject for CustomEmoji<Set> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        CustomEmoji {
            _create_id,
            _state: Default::default(),
            id: None,
            name: None,
            blob_id: None,
            space_id: None,
            created_by: None,
            created_at: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for CustomEmoji<Get> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
