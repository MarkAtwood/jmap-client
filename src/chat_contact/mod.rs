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

use crate::chat_types::Endpoint;
use crate::core::changes::ChangesObject;
use crate::core::Object;
use crate::{Get, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatContact<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(rename = "login")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,

    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    #[serde(rename = "firstSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_at: Option<String>,

    #[serde(rename = "lastSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,

    #[serde(rename = "presence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<String>,

    #[serde(rename = "lastActiveAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_at: Option<String>,

    #[serde(rename = "statusText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,

    #[serde(rename = "statusEmoji")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_emoji: Option<String>,

    #[serde(rename = "endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,

    #[serde(rename = "blocked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "login")]
    Login,
    #[serde(rename = "displayName")]
    DisplayName,
    #[serde(rename = "firstSeenAt")]
    FirstSeenAt,
    #[serde(rename = "lastSeenAt")]
    LastSeenAt,
    #[serde(rename = "presence")]
    Presence,
    #[serde(rename = "lastActiveAt")]
    LastActiveAt,
    #[serde(rename = "statusText")]
    StatusText,
    #[serde(rename = "statusEmoji")]
    StatusEmoji,
    #[serde(rename = "endpoints")]
    Endpoints,
    #[serde(rename = "blocked")]
    Blocked,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::Login => write!(f, "login"),
            Property::DisplayName => write!(f, "displayName"),
            Property::FirstSeenAt => write!(f, "firstSeenAt"),
            Property::LastSeenAt => write!(f, "lastSeenAt"),
            Property::Presence => write!(f, "presence"),
            Property::LastActiveAt => write!(f, "lastActiveAt"),
            Property::StatusText => write!(f, "statusText"),
            Property::StatusEmoji => write!(f, "statusEmoji"),
            Property::Endpoints => write!(f, "endpoints"),
            Property::Blocked => write!(f, "blocked"),
        }
    }
}

impl Object for ChatContact<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for ChatContact<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for ChatContact<Set> {
    type ChangesResponse = ();
}

impl ChangesObject for ChatContact<Get> {
    type ChangesResponse = ();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat_contact::query::Filter;
    use crate::core::set::SetObject;

    #[test]
    fn test_chat_contact_get_deserialize() {
        let json = r#"{
            "id": "contact1",
            "login": "alice@example.com",
            "displayName": "Alice",
            "firstSeenAt": "2026-01-01T00:00:00Z",
            "lastSeenAt": "2026-04-01T12:00:00Z",
            "presence": "online",
            "lastActiveAt": "2026-04-01T11:59:00Z",
            "statusText": "Working from home",
            "statusEmoji": "🏠",
            "blocked": false
        }"#;
        let contact: ChatContact<Get> = serde_json::from_str(json).unwrap();
        assert_eq!(contact.id.as_deref(), Some("contact1"));
        assert_eq!(contact.login.as_deref(), Some("alice@example.com"));
        assert_eq!(contact.display_name.as_deref(), Some("Alice"));
        assert_eq!(contact.presence.as_deref(), Some("online"));
        assert_eq!(contact.blocked, Some(false));
    }

    #[test]
    fn test_chat_contact_set_skip_server_fields() {
        let contact = ChatContact::<Set>::new(None);
        let json = serde_json::to_string(&contact).unwrap();
        assert!(!json.contains("\"id\""));
        assert!(!json.contains("\"login\""));
        assert!(!json.contains("\"firstSeenAt\""));
        assert!(!json.contains("\"lastSeenAt\""));
    }

    #[test]
    fn test_chat_contact_query_filter_blocked() {
        let filter = Filter::blocked(true);
        let json = serde_json::to_string(&filter).unwrap();
        assert!(json.contains("\"blocked\":true"));
    }
}
