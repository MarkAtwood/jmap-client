use super::ReadPosition;
use crate::core::get::GetObject;
use crate::{Get, Set};

impl ReadPosition<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
    pub fn chat_id(&self) -> Option<&str> {
        self.chat_id.as_deref()
    }
    pub fn last_read_message_id(&self) -> Option<&str> {
        self.last_read_message_id.as_deref()
    }
    pub fn last_read_at(&self) -> Option<&str> {
        self.last_read_at.as_deref()
    }
}

impl GetObject for ReadPosition<Set> {
    type GetArguments = ();
}
impl GetObject for ReadPosition<Get> {
    type GetArguments = ();
}
