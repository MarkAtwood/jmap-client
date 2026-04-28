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

use super::Space;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Filter {
    Name {
        #[serde(rename = "name")]
        value: String,
    },
    IsPublic {
        #[serde(rename = "isPublic")]
        value: bool,
    },
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "property")]
pub enum Comparator {
    #[serde(rename = "name")]
    Name,
}

impl QueryObject for Space<Set> {
    type QueryArguments = ();
    type Filter = Filter;
    type Sort = Comparator;
}
