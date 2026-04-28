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

use super::Message;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Filter {
    ChatId {
        #[serde(rename = "chatId")]
        value: String,
    },
    Text {
        #[serde(rename = "text")]
        value: String,
    },
    ThreadRootId {
        #[serde(rename = "threadRootId")]
        value: String,
    },
    HasAttachment {
        #[serde(rename = "hasAttachment")]
        value: bool,
    },
    HasMention {
        #[serde(rename = "hasMention")]
        value: bool,
    },
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "property")]
pub enum Comparator {
    #[serde(rename = "receivedAt")]
    ReceivedAt,
}

impl QueryObject for Message<Set> {
    type QueryArguments = ();
    type Filter = Filter;
    type Sort = Comparator;
}
