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
        response::{ChatGetResponse, ChatSetResponse},
        set::{SetObject, SetRequest},
    },
    Get, Method, Set, URI,
};

use super::{Chat, Property, TypingResponse};

impl Client {
    #[maybe_async::maybe_async]
    pub async fn chat_create(
        &self,
        name: impl Into<String>,
        kind: impl Into<String>,
    ) -> crate::Result<Chat> {
        let mut request = self.build();
        let id = request
            .set_chat()
            .create()
            .name(name)
            .kind(kind)
            .create_id()
            .unwrap();
        request.send_single::<ChatSetResponse>().await?.created(&id)
    }

    #[maybe_async::maybe_async]
    pub async fn chat_update_name(
        &self,
        id: &str,
        name: impl Into<String>,
    ) -> crate::Result<Option<Chat>> {
        let mut request = self.build();
        request.set_chat().update(id).name(name);
        request.send_single::<ChatSetResponse>().await?.updated(id)
    }

    #[maybe_async::maybe_async]
    pub async fn chat_destroy(&self, id: &str) -> crate::Result<()> {
        let mut request = self.build();
        request.set_chat().destroy([id]);
        request
            .send_single::<ChatSetResponse>()
            .await?
            .destroyed(id)
    }

    #[maybe_async::maybe_async]
    pub async fn chat_get(
        &self,
        id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Option<Chat>> {
        let mut request = self.build();
        let get_request = request.get_chat().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<ChatGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    #[maybe_async::maybe_async]
    pub async fn chat_query(
        &self,
        filter: Option<impl Into<Filter<super::query::Filter>>>,
        sort: Option<impl IntoIterator<Item = Comparator<super::query::Comparator>>>,
    ) -> crate::Result<QueryResponse> {
        let mut request = self.build();
        let query_request = request.query_chat();
        if let Some(filter) = filter {
            query_request.filter(filter);
        }
        if let Some(sort) = sort {
            query_request.sort(sort.into_iter());
        }
        request.send_single::<QueryResponse>().await
    }

    #[maybe_async::maybe_async]
    pub async fn chat_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<Chat<Get>>> {
        let mut request = self.build();
        request.changes_chat(since_state).max_changes(max_changes);
        request.send_single().await
    }

    #[maybe_async::maybe_async]
    pub async fn chat_typing(&self, chat_id: &str, typing: bool) -> crate::Result<TypingResponse> {
        let mut request = self.build();
        request.typing_chat(chat_id, typing);
        request.send_single::<TypingResponse>().await
    }
}

impl Request<'_> {
    pub fn get_chat(&mut self) -> &mut GetRequest<Chat<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::GetChat,
            Arguments::chat_get(self.params(Method::GetChat)),
        )
        .chat_get_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_get_chat(self) -> crate::Result<ChatGetResponse> {
        self.send_single().await
    }

    pub fn changes_chat(&mut self, since_state: impl Into<String>) -> &mut ChangesRequest {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::ChangesChat,
            Arguments::changes(self.params(Method::ChangesChat), since_state.into()),
        )
        .changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_changes_chat(self) -> crate::Result<ChangesResponse<Chat<Get>>> {
        self.send_single().await
    }

    pub fn query_chat(&mut self) -> &mut QueryRequest<Chat<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::QueryChat,
            Arguments::chat_query(self.params(Method::QueryChat)),
        )
        .chat_query_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_chat(self) -> crate::Result<QueryResponse> {
        self.send_single().await
    }

    pub fn query_chat_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<Chat<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::QueryChangesChat,
            Arguments::chat_query_changes(
                self.params(Method::QueryChangesChat),
                since_query_state.into(),
            ),
        )
        .chat_query_changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_chat_changes(self) -> crate::Result<QueryChangesResponse> {
        self.send_single().await
    }

    pub fn set_chat(&mut self) -> &mut SetRequest<Chat<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::SetChat,
            Arguments::chat_set(self.params(Method::SetChat)),
        )
        .chat_set_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_set_chat(self) -> crate::Result<ChatSetResponse> {
        self.send_single().await
    }

    pub fn typing_chat(&mut self, chat_id: impl Into<String>, typing: bool) {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::TypingChat,
            Arguments::typing_chat(self.params(Method::TypingChat), chat_id.into(), typing),
        );
    }

    #[maybe_async::maybe_async]
    pub async fn send_typing_chat(self) -> crate::Result<TypingResponse> {
        self.send_single().await
    }
}
