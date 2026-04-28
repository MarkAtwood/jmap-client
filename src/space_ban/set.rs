use super::SpaceBan;
use crate::core::set::SetObject;
use crate::{Get, Set};

impl SpaceBan<Set> {
    pub fn space_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.space_id = Some(id.into());
        self
    }
    pub fn user_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.user_id = Some(id.into());
        self
    }
    pub fn reason(&mut self, reason: impl Into<String>) -> &mut Self {
        self.reason = Some(reason.into());
        self
    }
    pub fn expires_at(&mut self, expires_at: impl Into<String>) -> &mut Self {
        self.expires_at = Some(expires_at.into());
        self
    }
}

impl SetObject for SpaceBan<Set> {
    type SetArguments = ();
    fn new(_create_id: Option<usize>) -> Self {
        SpaceBan {
            _create_id,
            _state: Default::default(),
            id: None,
            space_id: None,
            user_id: None,
            banned_by: None,
            reason: None,
            created_at: None,
            expires_at: None,
        }
    }
    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for SpaceBan<Get> {
    type SetArguments = ();
    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }
    fn create_id(&self) -> Option<String> {
        None
    }
}
