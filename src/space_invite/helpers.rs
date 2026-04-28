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
        response::{SpaceInviteGetResponse, SpaceInviteSetResponse},
        set::SetRequest,
    },
    Get, Method, Set, URI,
};

use super::{Property, SpaceInvite};

impl Client {
    #[maybe_async::maybe_async]
    pub async fn space_invite_create(
        &self,
        invite: SpaceInvite<Set>,
    ) -> crate::Result<SpaceInvite> {
        let mut request = self.build();
        let id = request.set_space_invite().create_item(invite);
        request
            .send_single::<SpaceInviteSetResponse>()
            .await?
            .created(&id)
    }

    #[maybe_async::maybe_async]
    pub async fn space_invite_destroy(&self, id: &str) -> crate::Result<()> {
        let mut request = self.build();
        request.set_space_invite().destroy([id]);
        request
            .send_single::<SpaceInviteSetResponse>()
            .await?
            .destroyed(id)
    }

    #[maybe_async::maybe_async]
    pub async fn space_invite_get(
        &self,
        id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Option<SpaceInvite>> {
        let mut request = self.build();
        let get_request = request.get_space_invite().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<SpaceInviteGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    #[maybe_async::maybe_async]
    pub async fn space_invite_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<SpaceInvite<Get>>> {
        let mut request = self.build();
        request
            .changes_space_invite(since_state)
            .max_changes(max_changes);
        request.send_single().await
    }
}

impl Request<'_> {
    pub fn get_space_invite(&mut self) -> &mut GetRequest<SpaceInvite<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::GetSpaceInvite,
            Arguments::space_invite_get(self.params(Method::GetSpaceInvite)),
        )
        .space_invite_get_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_get_space_invite(self) -> crate::Result<SpaceInviteGetResponse> {
        self.send_single().await
    }

    pub fn changes_space_invite(&mut self, since_state: impl Into<String>) -> &mut ChangesRequest {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::ChangesSpaceInvite,
            Arguments::changes(self.params(Method::ChangesSpaceInvite), since_state.into()),
        )
        .changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_changes_space_invite(
        self,
    ) -> crate::Result<ChangesResponse<SpaceInvite<Get>>> {
        self.send_single().await
    }

    pub fn set_space_invite(&mut self) -> &mut SetRequest<SpaceInvite<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::SetSpaceInvite,
            Arguments::space_invite_set(self.params(Method::SetSpaceInvite)),
        )
        .space_invite_set_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_set_space_invite(self) -> crate::Result<SpaceInviteSetResponse> {
        self.send_single().await
    }
}
