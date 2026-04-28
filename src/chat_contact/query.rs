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

use super::ChatContact;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Filter {
    Blocked {
        #[serde(rename = "blocked")]
        value: bool,
    },
    Presence {
        #[serde(rename = "presence")]
        value: String,
    },
}

impl Filter {
    pub fn blocked(value: bool) -> Self {
        Filter::Blocked { value }
    }

    pub fn presence(value: impl Into<String>) -> Self {
        Filter::Presence {
            value: value.into(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "property")]
pub enum Comparator {
    #[serde(rename = "lastSeenAt")]
    LastSeenAt,
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "lastActiveAt")]
    LastActiveAt,
}

impl QueryObject for ChatContact<Set> {
    type QueryArguments = ();
    type Filter = Filter;
    type Sort = Comparator;
}
