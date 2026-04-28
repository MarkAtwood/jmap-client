use crate::core::set::SetObject;
use crate::{Get, Set};

use super::PresenceStatus;

impl PresenceStatus<Set> {
    pub fn presence(&mut self, presence: impl Into<String>) -> &mut Self {
        self.presence = Some(presence.into());
        self
    }
    pub fn status_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.status_text = Some(text.into());
        self
    }
    pub fn status_emoji(&mut self, emoji: impl Into<String>) -> &mut Self {
        self.status_emoji = Some(emoji.into());
        self
    }
    pub fn expires_at(&mut self, expires_at: impl Into<String>) -> &mut Self {
        self.expires_at = Some(expires_at.into());
        self
    }
    pub fn receipt_sharing(&mut self, sharing: bool) -> &mut Self {
        self.receipt_sharing = Some(sharing);
        self
    }
}

impl SetObject for PresenceStatus<Set> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        PresenceStatus {
            _create_id: None,
            _state: Default::default(),
            id: None,
            presence: None,
            status_text: None,
            status_emoji: None,
            expires_at: None,
            receipt_sharing: None,
            updated_at: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}

impl SetObject for PresenceStatus<Get> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
