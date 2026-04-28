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
        response::{ChatContactGetResponse, ChatContactSetResponse},
        set::{SetObject, SetRequest},
    },
    Get, Method, Set, URI,
};

use super::{ChatContact, Property};

impl Client {
    #[maybe_async::maybe_async]
    pub async fn chat_contact_create(
        &self,
        display_name: impl Into<String>,
    ) -> crate::Result<ChatContact> {
        let mut request = self.build();
        let id = request
            .set_chat_contact()
            .create()
            .display_name(display_name)
            .create_id()
            .unwrap();
        request
            .send_single::<ChatContactSetResponse>()
            .await?
            .created(&id)
    }

    #[maybe_async::maybe_async]
    pub async fn chat_contact_update(
        &self,
        id: &str,
        display_name: Option<impl Into<String>>,
        blocked: Option<bool>,
    ) -> crate::Result<Option<ChatContact>> {
        let mut request = self.build();
        let obj = request.set_chat_contact().update(id);
        if let Some(name) = display_name {
            obj.display_name(name);
        }
        if let Some(b) = blocked {
            obj.blocked(b);
        }
        request
            .send_single::<ChatContactSetResponse>()
            .await?
            .updated(id)
    }

    #[maybe_async::maybe_async]
    pub async fn chat_contact_destroy(&self, id: &str) -> crate::Result<()> {
        let mut request = self.build();
        request.set_chat_contact().destroy([id]);
        request
            .send_single::<ChatContactSetResponse>()
            .await?
            .destroyed(id)
    }

    #[maybe_async::maybe_async]
    pub async fn chat_contact_get(
        &self,
        id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Option<ChatContact>> {
        let mut request = self.build();
        let get_request = request.get_chat_contact().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<ChatContactGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    #[maybe_async::maybe_async]
    pub async fn chat_contact_query(
        &self,
        filter: Option<impl Into<Filter<super::query::Filter>>>,
        sort: Option<impl IntoIterator<Item = Comparator<super::query::Comparator>>>,
    ) -> crate::Result<QueryResponse> {
        let mut request = self.build();
        let query_request = request.query_chat_contact();
        if let Some(filter) = filter {
            query_request.filter(filter);
        }
        if let Some(sort) = sort {
            query_request.sort(sort.into_iter());
        }
        request.send_single::<QueryResponse>().await
    }

    #[maybe_async::maybe_async]
    pub async fn chat_contact_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<ChatContact<Get>>> {
        let mut request = self.build();
        request
            .changes_chat_contact(since_state)
            .max_changes(max_changes);
        request.send_single().await
    }
}

impl Request<'_> {
    pub fn get_chat_contact(&mut self) -> &mut GetRequest<ChatContact<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::GetChatContact,
            Arguments::chat_contact_get(self.params(Method::GetChatContact)),
        )
        .chat_contact_get_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_get_chat_contact(self) -> crate::Result<ChatContactGetResponse> {
        self.send_single().await
    }

    pub fn changes_chat_contact(&mut self, since_state: impl Into<String>) -> &mut ChangesRequest {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::ChangesChatContact,
            Arguments::changes(self.params(Method::ChangesChatContact), since_state.into()),
        )
        .changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_changes_chat_contact(
        self,
    ) -> crate::Result<ChangesResponse<ChatContact<Get>>> {
        self.send_single().await
    }

    pub fn query_chat_contact(&mut self) -> &mut QueryRequest<ChatContact<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::QueryChatContact,
            Arguments::chat_contact_query(self.params(Method::QueryChatContact)),
        )
        .chat_contact_query_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_chat_contact(self) -> crate::Result<QueryResponse> {
        self.send_single().await
    }

    pub fn query_chat_contact_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<ChatContact<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::QueryChangesChatContact,
            Arguments::chat_contact_query_changes(
                self.params(Method::QueryChangesChatContact),
                since_query_state.into(),
            ),
        )
        .chat_contact_query_changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_chat_contact_changes(self) -> crate::Result<QueryChangesResponse> {
        self.send_single().await
    }

    pub fn set_chat_contact(&mut self) -> &mut SetRequest<ChatContact<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::SetChatContact,
            Arguments::chat_contact_set(self.params(Method::SetChatContact)),
        )
        .chat_contact_set_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_set_chat_contact(self) -> crate::Result<ChatContactSetResponse> {
        self.send_single().await
    }
}
