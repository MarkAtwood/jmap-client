use crate::{
    client::Client,
    core::{
        changes::{ChangesRequest, ChangesResponse},
        get::GetRequest,
        request::{Arguments, Request},
        response::{ReadPositionGetResponse, ReadPositionSetResponse},
        set::SetRequest,
    },
    Get, Method, Set, URI,
};

use super::ReadPosition;

impl Client {
    #[maybe_async::maybe_async]
    pub async fn read_position_get(&self, chat_id: &str) -> crate::Result<Option<ReadPosition>> {
        let mut request = self.build();
        request.get_read_position().ids([chat_id]);
        request
            .send_single::<ReadPositionGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    #[maybe_async::maybe_async]
    pub async fn read_position_set(
        &self,
        id: &str,
        update: ReadPosition<crate::Set>,
    ) -> crate::Result<Option<ReadPosition>> {
        let mut request = self.build();
        request.set_read_position().update_item(id, update);
        request
            .send_single::<ReadPositionSetResponse>()
            .await?
            .updated(id)
    }
}

impl Request<'_> {
    pub fn get_read_position(&mut self) -> &mut GetRequest<ReadPosition<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::GetReadPosition,
            Arguments::read_position_get(self.params(Method::GetReadPosition)),
        )
        .read_position_get_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_get_read_position(self) -> crate::Result<ReadPositionGetResponse> {
        self.send_single().await
    }

    pub fn changes_read_position(&mut self, since_state: impl Into<String>) -> &mut ChangesRequest {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::ChangesReadPosition,
            Arguments::changes(self.params(Method::ChangesReadPosition), since_state.into()),
        )
        .changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_changes_read_position(
        self,
    ) -> crate::Result<ChangesResponse<ReadPosition<Get>>> {
        self.send_single().await
    }

    pub fn set_read_position(&mut self) -> &mut SetRequest<ReadPosition<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::SetReadPosition,
            Arguments::read_position_set(self.params(Method::SetReadPosition)),
        )
        .read_position_set_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_set_read_position(self) -> crate::Result<ReadPositionSetResponse> {
        self.send_single().await
    }
}
