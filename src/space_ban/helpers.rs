/*
 * Copyright Stalwart Labs LLC See the COPYING
 * file at the top-level directory of this distribution.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use crate::{
    client::Client,
    core::{
        changes::{ChangesRequest, ChangesResponse},
        get::GetRequest,
        request::{Arguments, Request},
        response::{SpaceBanGetResponse, SpaceBanSetResponse},
        set::SetRequest,
    },
    Get, Method, Set, URI,
};

use super::{Property, SpaceBan};

impl Client {
    #[maybe_async::maybe_async]
    pub async fn space_ban_create(&self, ban: SpaceBan<Set>) -> crate::Result<SpaceBan> {
        let mut request = self.build();
        let id = request.set_space_ban().create_item(ban);
        request
            .send_single::<SpaceBanSetResponse>()
            .await?
            .created(&id)
    }

    #[maybe_async::maybe_async]
    pub async fn space_ban_destroy(&self, id: &str) -> crate::Result<()> {
        let mut request = self.build();
        request.set_space_ban().destroy([id]);
        request
            .send_single::<SpaceBanSetResponse>()
            .await?
            .destroyed(id)
    }

    #[maybe_async::maybe_async]
    pub async fn space_ban_get(
        &self,
        id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Option<SpaceBan>> {
        let mut request = self.build();
        let get_request = request.get_space_ban().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<SpaceBanGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    #[maybe_async::maybe_async]
    pub async fn space_ban_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<SpaceBan<Get>>> {
        let mut request = self.build();
        request
            .changes_space_ban(since_state)
            .max_changes(max_changes);
        request.send_single().await
    }
}

impl Request<'_> {
    pub fn get_space_ban(&mut self) -> &mut GetRequest<SpaceBan<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::GetSpaceBan,
            Arguments::space_ban_get(self.params(Method::GetSpaceBan)),
        )
        .space_ban_get_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_get_space_ban(self) -> crate::Result<SpaceBanGetResponse> {
        self.send_single().await
    }

    pub fn changes_space_ban(&mut self, since_state: impl Into<String>) -> &mut ChangesRequest {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::ChangesSpaceBan,
            Arguments::changes(self.params(Method::ChangesSpaceBan), since_state.into()),
        )
        .changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_changes_space_ban(self) -> crate::Result<ChangesResponse<SpaceBan<Get>>> {
        self.send_single().await
    }

    pub fn set_space_ban(&mut self) -> &mut SetRequest<SpaceBan<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::SetSpaceBan,
            Arguments::space_ban_set(self.params(Method::SetSpaceBan)),
        )
        .space_ban_set_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_set_space_ban(self) -> crate::Result<SpaceBanSetResponse> {
        self.send_single().await
    }
}
