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

pub mod get;
pub mod helpers;
pub mod query;
pub mod set;

use std::fmt::Display;

use crate::core::changes::ChangesObject;
use crate::core::Object;
use crate::{Get, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomEmoji<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "blobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob_id: Option<String>,

    #[serde(rename = "spaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,

    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,

    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "blobId")]
    BlobId,
    #[serde(rename = "spaceId")]
    SpaceId,
    #[serde(rename = "createdBy")]
    CreatedBy,
    #[serde(rename = "createdAt")]
    CreatedAt,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::Name => write!(f, "name"),
            Property::BlobId => write!(f, "blobId"),
            Property::SpaceId => write!(f, "spaceId"),
            Property::CreatedBy => write!(f, "createdBy"),
            Property::CreatedAt => write!(f, "createdAt"),
        }
    }
}

impl Object for CustomEmoji<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for CustomEmoji<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for CustomEmoji<Set> {
    type ChangesResponse = ();
}

impl ChangesObject for CustomEmoji<Get> {
    type ChangesResponse = ();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::set::SetObject;
    use crate::Set;

    #[test]
    fn test_custom_emoji_get_deserialize() {
        let json = r#"{
            "id": "emoji-1",
            "name": "cat-jam",
            "blobId": "blob-abc",
            "spaceId": "space-xyz",
            "createdBy": "user-1",
            "createdAt": "2024-01-01T00:00:00Z"
        }"#;
        let emoji: CustomEmoji<Get> = serde_json::from_str(json).unwrap();
        assert_eq!(emoji.name(), Some("cat-jam"));
        assert_eq!(emoji.blob_id(), Some("blob-abc"));
        assert_eq!(emoji.space_id(), Some("space-xyz"));
    }

    #[test]
    fn test_custom_emoji_set_skip_server_fields() {
        let obj = CustomEmoji::<Set>::new(None);
        let json = serde_json::to_string(&obj).unwrap();
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"createdBy\""));
        assert!(!json.contains("\"createdAt\""));
    }

    #[test]
    fn test_validate_name_ok() {
        assert!(set::validate_name("cat-jam_2").is_ok());
    }

    #[test]
    fn test_validate_name_reject_uppercase() {
        assert!(set::validate_name("Cat").is_err());
    }

    #[test]
    fn test_validate_name_reject_space() {
        assert!(set::validate_name("cat jam").is_err());
    }

    #[test]
    fn test_validate_name_empty() {
        assert!(set::validate_name("").is_err());
    }
}
