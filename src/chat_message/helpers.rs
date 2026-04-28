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
        response::{ChatMessageGetResponse, ChatMessageSetResponse},
        set::{SetObject, SetRequest},
    },
    Get, Method, Set, URI,
};

use super::{Message, Property};

impl Client {
    #[maybe_async::maybe_async]
    pub async fn chat_message_send(
        &self,
        chat_id: impl Into<String>,
        body: impl Into<String>,
    ) -> crate::Result<Message> {
        let mut request = self.build();
        let id = request
            .set_chat_message()
            .create()
            .chat_id(chat_id)
            .body(body)
            .create_id()
            .unwrap();
        request
            .send_single::<ChatMessageSetResponse>()
            .await?
            .created(&id)
    }

    #[maybe_async::maybe_async]
    pub async fn chat_message_update(
        &self,
        id: &str,
        body: impl Into<String>,
    ) -> crate::Result<Option<Message>> {
        let mut request = self.build();
        request.set_chat_message().update(id).body(body);
        request
            .send_single::<ChatMessageSetResponse>()
            .await?
            .updated(id)
    }

    #[maybe_async::maybe_async]
    pub async fn chat_message_destroy(&self, id: &str) -> crate::Result<()> {
        let mut request = self.build();
        request.set_chat_message().destroy([id]);
        request
            .send_single::<ChatMessageSetResponse>()
            .await?
            .destroyed(id)
    }

    #[maybe_async::maybe_async]
    pub async fn chat_message_get(
        &self,
        id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Option<Message>> {
        let mut request = self.build();
        let get_request = request.get_chat_message().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<ChatMessageGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    #[maybe_async::maybe_async]
    pub async fn chat_message_query(
        &self,
        filter: Option<impl Into<Filter<super::query::Filter>>>,
        sort: Option<impl IntoIterator<Item = Comparator<super::query::Comparator>>>,
    ) -> crate::Result<QueryResponse> {
        let mut request = self.build();
        let query_request = request.query_chat_message();
        if let Some(filter) = filter {
            query_request.filter(filter);
        }
        if let Some(sort) = sort {
            query_request.sort(sort.into_iter());
        }
        request.send_single::<QueryResponse>().await
    }

    #[maybe_async::maybe_async]
    pub async fn chat_message_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<Message<Get>>> {
        let mut request = self.build();
        request
            .changes_chat_message(since_state)
            .max_changes(max_changes);
        request.send_single().await
    }
}

impl Request<'_> {
    pub fn get_chat_message(&mut self) -> &mut GetRequest<Message<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::GetChatMessage,
            Arguments::chat_message_get(self.params(Method::GetChatMessage)),
        )
        .chat_message_get_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_get_chat_message(self) -> crate::Result<ChatMessageGetResponse> {
        self.send_single().await
    }

    pub fn changes_chat_message(&mut self, since_state: impl Into<String>) -> &mut ChangesRequest {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::ChangesChatMessage,
            Arguments::changes(self.params(Method::ChangesChatMessage), since_state.into()),
        )
        .changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_changes_chat_message(self) -> crate::Result<ChangesResponse<Message<Get>>> {
        self.send_single().await
    }

    pub fn query_chat_message(&mut self) -> &mut QueryRequest<Message<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::QueryChatMessage,
            Arguments::chat_message_query(self.params(Method::QueryChatMessage)),
        )
        .chat_message_query_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_chat_message(self) -> crate::Result<QueryResponse> {
        self.send_single().await
    }

    pub fn query_chat_message_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<Message<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::QueryChangesChatMessage,
            Arguments::chat_message_query_changes(
                self.params(Method::QueryChangesChatMessage),
                since_query_state.into(),
            ),
        )
        .chat_message_query_changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_chat_message_changes(self) -> crate::Result<QueryChangesResponse> {
        self.send_single().await
    }

    pub fn set_chat_message(&mut self) -> &mut SetRequest<Message<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::SetChatMessage,
            Arguments::chat_message_set(self.params(Method::SetChatMessage)),
        )
        .chat_message_set_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_set_chat_message(self) -> crate::Result<ChatMessageSetResponse> {
        self.send_single().await
    }
}
