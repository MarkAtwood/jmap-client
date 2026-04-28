use crate::core::get::GetObject;
use crate::{Get, Set};

use super::SpaceInvite;

impl SpaceInvite<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }
    pub fn space_id(&self) -> Option<&str> {
        self.space_id.as_deref()
    }
    pub fn default_channel_id(&self) -> Option<&str> {
        self.default_channel_id.as_deref()
    }
    pub fn created_by(&self) -> Option<&str> {
        self.created_by.as_deref()
    }
    pub fn expires_at(&self) -> Option<&str> {
        self.expires_at.as_deref()
    }
    pub fn max_uses(&self) -> Option<u64> {
        self.max_uses
    }
    pub fn uses(&self) -> Option<u64> {
        self.uses
    }
    pub fn created_at(&self) -> Option<&str> {
        self.created_at.as_deref()
    }
}

impl GetObject for SpaceInvite<Set> {
    type GetArguments = ();
}
impl GetObject for SpaceInvite<Get> {
    type GetArguments = ();
}
