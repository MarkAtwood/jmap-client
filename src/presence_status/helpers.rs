use crate::{
    client::Client,
    core::{
        changes::{ChangesRequest, ChangesResponse},
        get::GetRequest,
        request::{Arguments, Request},
        response::{PresenceStatusGetResponse, PresenceStatusSetResponse},
        set::SetRequest,
    },
    Get, Method, Set, URI,
};

use super::PresenceStatus;

impl Client {
    #[maybe_async::maybe_async]
    pub async fn presence_status_get(&self) -> crate::Result<Option<PresenceStatus>> {
        let mut request = self.build();
        request.get_presence_status();
        request
            .send_single::<PresenceStatusGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    #[maybe_async::maybe_async]
    pub async fn presence_status_set(
        &self,
        id: &str,
        update: PresenceStatus<crate::Set>,
    ) -> crate::Result<Option<PresenceStatus>> {
        let mut request = self.build();
        request.set_presence_status().update_item(id, update);
        request
            .send_single::<PresenceStatusSetResponse>()
            .await?
            .updated(id)
    }
}

impl Request<'_> {
    pub fn get_presence_status(&mut self) -> &mut GetRequest<PresenceStatus<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::GetPresenceStatus,
            Arguments::presence_status_get(self.params(Method::GetPresenceStatus)),
        )
        .presence_status_get_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_get_presence_status(self) -> crate::Result<PresenceStatusGetResponse> {
        self.send_single().await
    }

    pub fn changes_presence_status(
        &mut self,
        since_state: impl Into<String>,
    ) -> &mut ChangesRequest {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::ChangesPresenceStatus,
            Arguments::changes(
                self.params(Method::ChangesPresenceStatus),
                since_state.into(),
            ),
        )
        .changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_changes_presence_status(
        self,
    ) -> crate::Result<ChangesResponse<PresenceStatus<Get>>> {
        self.send_single().await
    }

    pub fn set_presence_status(&mut self) -> &mut SetRequest<PresenceStatus<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::SetPresenceStatus,
            Arguments::presence_status_set(self.params(Method::SetPresenceStatus)),
        )
        .presence_status_set_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_set_presence_status(self) -> crate::Result<PresenceStatusSetResponse> {
        self.send_single().await
    }
}
