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

use crate::chat_types::{Category, SpaceMember, SpaceRole};
use crate::core::changes::ChangesObject;
use crate::core::Object;
use crate::{Get, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space<State = Get> {
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

    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "iconBlobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_blob_id: Option<String>,

    #[serde(rename = "roles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<SpaceRole>>,

    #[serde(rename = "members")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<SpaceMember>>,

    #[serde(rename = "categories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<Category>>,

    #[serde(rename = "uncategorizedChannelIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_channel_ids: Option<Vec<String>>,

    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    #[serde(rename = "isPublic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,

    #[serde(rename = "isPubliclyPreviewable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_publicly_previewable: Option<bool>,

    #[serde(rename = "memberCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "iconBlobId")]
    IconBlobId,
    #[serde(rename = "roles")]
    Roles,
    #[serde(rename = "members")]
    Members,
    #[serde(rename = "categories")]
    Categories,
    #[serde(rename = "uncategorizedChannelIds")]
    UncategorizedChannelIds,
    #[serde(rename = "createdAt")]
    CreatedAt,
    #[serde(rename = "isPublic")]
    IsPublic,
    #[serde(rename = "isPubliclyPreviewable")]
    IsPubliclyPreviewable,
    #[serde(rename = "memberCount")]
    MemberCount,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::Name => write!(f, "name"),
            Property::Description => write!(f, "description"),
            Property::IconBlobId => write!(f, "iconBlobId"),
            Property::Roles => write!(f, "roles"),
            Property::Members => write!(f, "members"),
            Property::Categories => write!(f, "categories"),
            Property::UncategorizedChannelIds => write!(f, "uncategorizedChannelIds"),
            Property::CreatedAt => write!(f, "createdAt"),
            Property::IsPublic => write!(f, "isPublic"),
            Property::IsPubliclyPreviewable => write!(f, "isPubliclyPreviewable"),
            Property::MemberCount => write!(f, "memberCount"),
        }
    }
}

impl Object for Space<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for Space<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for Space<Set> {
    type ChangesResponse = ();
}

impl ChangesObject for Space<Get> {
    type ChangesResponse = ();
}

/// Request body for Space/join.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceJoinRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "inviteCode", skip_serializing_if = "Option::is_none")]
    pub invite_code: Option<String>,
    #[serde(rename = "spaceId", skip_serializing_if = "Option::is_none")]
    pub space_id: Option<String>,
}

impl SpaceJoinRequest {
    /// Validates that exactly one of invite_code or space_id is Some.
    pub fn validate(&self) -> Result<(), String> {
        match (&self.invite_code, &self.space_id) {
            (Some(_), None) | (None, Some(_)) => Ok(()),
            (Some(_), Some(_)) => {
                Err("both inviteCode and spaceId provided; supply exactly one".to_string())
            }
            (None, None) => Err("either inviteCode or spaceId is required".to_string()),
        }
    }
}

/// Response body for Space/join.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceJoinResponse {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "spaceId")]
    pub space_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_space_get_deserialize() {
        let json = r#"{
            "id": "space1",
            "name": "My Space",
            "description": "A test space",
            "createdAt": "2026-01-01T00:00:00Z",
            "isPublic": false,
            "memberCount": 2,
            "roles": [],
            "members": [],
            "categories": []
        }"#;
        let s: Space<Get> = serde_json::from_str(json).unwrap();
        assert_eq!(s.id.as_deref(), Some("space1"));
        assert_eq!(s.name.as_deref(), Some("My Space"));
        assert_eq!(s.description.as_deref(), Some("A test space"));
        assert_eq!(s.created_at.as_deref(), Some("2026-01-01T00:00:00Z"));
        assert_eq!(s.is_public, Some(false));
        assert_eq!(s.member_count, Some(2));
        assert!(s.roles.as_ref().unwrap().is_empty());
        assert!(s.members.as_ref().unwrap().is_empty());
        assert!(s.categories.as_ref().unwrap().is_empty());
    }

    #[test]
    fn test_space_set_skip_server_fields() {
        use crate::core::set::SetObject;
        let obj = Space::<Set>::new(None);
        let json = serde_json::to_string(&obj).unwrap();
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"createdAt\""));
        assert!(!json.contains("\"memberCount\""));
    }

    #[test]
    fn test_space_join_request_validate_ok_invite() {
        let req = SpaceJoinRequest {
            account_id: "a1".to_string(),
            invite_code: Some("abc".to_string()),
            space_id: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_space_join_request_validate_ok_spaceid() {
        let req = SpaceJoinRequest {
            account_id: "a1".to_string(),
            invite_code: None,
            space_id: Some("sp1".to_string()),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_space_join_request_validate_both_err() {
        let req = SpaceJoinRequest {
            account_id: "a1".to_string(),
            invite_code: Some("abc".to_string()),
            space_id: Some("sp1".to_string()),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_space_join_request_validate_neither_err() {
        let req = SpaceJoinRequest {
            account_id: "a1".to_string(),
            invite_code: None,
            space_id: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_space_join_response_deserialize() {
        let json = r#"{"accountId":"a1","spaceId":"sp1"}"#;
        let r: SpaceJoinResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.account_id, "a1");
        assert_eq!(r.space_id, "sp1");
    }
}
