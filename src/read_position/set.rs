use super::ReadPosition;
use crate::core::set::SetObject;
use crate::{Get, Set};

impl ReadPosition<Set> {
    /// Set the most recently read message id. Server will update lastReadAt automatically.
    pub fn last_read_message_id(&mut self, id: impl Into<String>) -> &mut Self {
        self.last_read_message_id = Some(id.into());
        self
    }
}

impl SetObject for ReadPosition<Set> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        ReadPosition {
            _create_id: None,
            _state: Default::default(),
            id: None,
            chat_id: None,
            last_read_message_id: None,
            last_read_at: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}

impl SetObject for ReadPosition<Get> {
    type SetArguments = ();

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
