use super::SpaceBan;
use crate::core::get::GetObject;
use crate::{Get, Set};

impl SpaceBan<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
    pub fn space_id(&self) -> Option<&str> {
        self.space_id.as_deref()
    }
    pub fn user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }
    pub fn banned_by(&self) -> Option<&str> {
        self.banned_by.as_deref()
    }
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
    pub fn created_at(&self) -> Option<&str> {
        self.created_at.as_deref()
    }
    pub fn expires_at(&self) -> Option<&str> {
        self.expires_at.as_deref()
    }
}

impl GetObject for SpaceBan<Set> {
    type GetArguments = ();
}
impl GetObject for SpaceBan<Get> {
    type GetArguments = ();
}
