use crate::core::get::GetObject;
use crate::{Get, Set};

use super::PresenceStatus;

impl PresenceStatus<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
    pub fn presence(&self) -> Option<&str> {
        self.presence.as_deref()
    }
    pub fn status_text(&self) -> Option<&str> {
        self.status_text.as_deref()
    }
    pub fn status_emoji(&self) -> Option<&str> {
        self.status_emoji.as_deref()
    }
    pub fn expires_at(&self) -> Option<&str> {
        self.expires_at.as_deref()
    }
    pub fn receipt_sharing(&self) -> Option<bool> {
        self.receipt_sharing
    }
    pub fn updated_at(&self) -> Option<&str> {
        self.updated_at.as_deref()
    }
}

impl GetObject for PresenceStatus<Set> {
    type GetArguments = ();
}
impl GetObject for PresenceStatus<Get> {
    type GetArguments = ();
}
