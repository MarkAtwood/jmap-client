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
        query::{Comparator, Filter, QueryRequest, QueryResponse},
        query_changes::{QueryChangesRequest, QueryChangesResponse},
        request::{Arguments, Request},
        response::{SpaceGetResponse, SpaceSetResponse},
        set::SetRequest,
    },
    Get, Method, Set, URI,
};

use super::{Space, SpaceJoinRequest, SpaceJoinResponse};

impl Client {
    #[maybe_async::maybe_async]
    pub async fn space_create(&self, space: Space<Set>) -> crate::Result<Space> {
        let mut request = self.build();
        let id = request.set_space().create_item(space);
        request
            .send_single::<SpaceSetResponse>()
            .await?
            .created(&id)
    }

    #[maybe_async::maybe_async]
    pub async fn space_update(&self, id: &str, space: Space<Set>) -> crate::Result<Option<Space>> {
        let mut request = self.build();
        request.set_space().update_item(id, space);
        request.send_single::<SpaceSetResponse>().await?.updated(id)
    }

    #[maybe_async::maybe_async]
    pub async fn space_destroy(&self, id: &str) -> crate::Result<()> {
        let mut request = self.build();
        request.set_space().destroy([id]);
        request
            .send_single::<SpaceSetResponse>()
            .await?
            .destroyed(id)
    }

    #[maybe_async::maybe_async]
    pub async fn space_get(
        &self,
        id: &str,
        properties: Option<impl IntoIterator<Item = super::Property>>,
    ) -> crate::Result<Option<Space>> {
        let mut request = self.build();
        let get_request = request.get_space().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<SpaceGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    #[maybe_async::maybe_async]
    pub async fn space_query(
        &self,
        filter: Option<impl Into<Filter<super::query::Filter>>>,
        sort: Option<impl IntoIterator<Item = Comparator<super::query::Comparator>>>,
    ) -> crate::Result<QueryResponse> {
        let mut request = self.build();
        let query_request = request.query_space();
        if let Some(filter) = filter {
            query_request.filter(filter);
        }
        if let Some(sort) = sort {
            query_request.sort(sort.into_iter());
        }
        request.send_single::<QueryResponse>().await
    }

    #[maybe_async::maybe_async]
    pub async fn space_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<Space<Get>>> {
        let mut request = self.build();
        request.changes_space(since_state).max_changes(max_changes);
        request.send_single().await
    }

    #[maybe_async::maybe_async]
    pub async fn space_join(&self, request: SpaceJoinRequest) -> crate::Result<SpaceJoinResponse> {
        let mut req = self.build();
        req.join_space(request)?;
        req.send_single::<SpaceJoinResponse>().await
    }
}

impl Request<'_> {
    pub fn get_space(&mut self) -> &mut GetRequest<Space<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::GetSpace,
            Arguments::space_get(self.params(Method::GetSpace)),
        )
        .space_get_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_get_space(self) -> crate::Result<SpaceGetResponse> {
        self.send_single().await
    }

    pub fn changes_space(&mut self, since_state: impl Into<String>) -> &mut ChangesRequest {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::ChangesSpace,
            Arguments::changes(self.params(Method::ChangesSpace), since_state.into()),
        )
        .changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_changes_space(self) -> crate::Result<ChangesResponse<Space<Get>>> {
        self.send_single().await
    }

    pub fn query_space(&mut self) -> &mut QueryRequest<Space<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::QuerySpace,
            Arguments::space_query(self.params(Method::QuerySpace)),
        )
        .space_query_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_space(self) -> crate::Result<QueryResponse> {
        self.send_single().await
    }

    pub fn query_space_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<Space<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::QueryChangesSpace,
            Arguments::space_query_changes(
                self.params(Method::QueryChangesSpace),
                since_query_state.into(),
            ),
        )
        .space_query_changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_space_changes(self) -> crate::Result<QueryChangesResponse> {
        self.send_single().await
    }

    pub fn set_space(&mut self) -> &mut SetRequest<Space<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::SetSpace,
            Arguments::space_set(self.params(Method::SetSpace)),
        )
        .space_set_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_set_space(self) -> crate::Result<SpaceSetResponse> {
        self.send_single().await
    }

    pub fn join_space(&mut self, request: SpaceJoinRequest) -> crate::Result<()> {
        self.add_capability(URI::Chat);
        request.validate().map_err(crate::Error::Internal)?;
        self.add_method_call(Method::JoinSpace, Arguments::space_join(request));
        Ok(())
    }

    #[maybe_async::maybe_async]
    pub async fn send_join_space(self) -> crate::Result<SpaceJoinResponse> {
        self.send_single().await
    }
}
