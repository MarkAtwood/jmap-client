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

use serde::Serialize;

use crate::core::query::QueryObject;
use crate::Set;

use super::CustomEmoji;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Filter {
    SpaceId {
        #[serde(rename = "spaceId")]
        value: Option<String>,
    },
}

impl Filter {
    pub fn space_id(value: Option<impl Into<String>>) -> Self {
        Filter::SpaceId {
            value: value.map(Into::into),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "property")]
pub enum Comparator {
    #[serde(rename = "name")]
    Name,
}

impl QueryObject for CustomEmoji<Set> {
    type QueryArguments = ();
    type Filter = Filter;
    type Sort = Comparator;
}
