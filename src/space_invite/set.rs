use crate::core::set::SetObject;
use crate::{Get, Set};

use super::SpaceInvite;

impl SpaceInvite<Set> {
    pub fn space_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.space_id = Some(id.into());
        self
    }
    pub fn default_channel_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.default_channel_id = Some(id.into());
        self
    }
    pub fn expires_at(&mut self, t: impl Into<String>) -> &mut Self {
        self.expires_at = Some(t.into());
        self
    }
    pub fn max_uses(&mut self, n: u64) -> &mut Self {
        self.max_uses = Some(n);
        self
    }
}

impl SetObject for SpaceInvite<Set> {
    type SetArguments = ();
    fn new(_create_id: Option<usize>) -> Self {
        SpaceInvite {
            _create_id,
            _state: Default::default(),
            id: None,
            code: None,
            space_id: None,
            default_channel_id: None,
            created_by: None,
            expires_at: None,
            max_uses: None,
            uses: None,
            created_at: None,
        }
    }
    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for SpaceInvite<Get> {
    type SetArguments = ();
    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }
    fn create_id(&self) -> Option<String> {
        None
    }
}
