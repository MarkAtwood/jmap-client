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

use crate::{core::query::QueryObject, Set};

use super::Chat;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Filter {
    Kind {
        #[serde(rename = "kind")]
        value: String,
    },
    Muted {
        #[serde(rename = "muted")]
        value: bool,
    },
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "property")]
pub enum Comparator {
    #[serde(rename = "lastMessageAt")]
    LastMessageAt,
}

impl QueryObject for Chat<Set> {
    type QueryArguments = ();
    type Filter = Filter;
    type Sort = Comparator;
}
