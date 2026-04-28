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
        query::{Filter, QueryRequest, QueryResponse},
        query_changes::{QueryChangesRequest, QueryChangesResponse},
        request::{Arguments, Request},
        response::{CustomEmojiGetResponse, CustomEmojiSetResponse},
        set::SetRequest,
    },
    Get, Method, Set, URI,
};

use super::{CustomEmoji, Property};

impl Client {
    #[maybe_async::maybe_async]
    pub async fn custom_emoji_get(
        &self,
        id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Option<CustomEmoji>> {
        let mut request = self.build();
        let get_request = request.get_custom_emoji().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<CustomEmojiGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    #[maybe_async::maybe_async]
    pub async fn custom_emoji_query(
        &self,
        filter: Option<impl Into<Filter<super::query::Filter>>>,
    ) -> crate::Result<QueryResponse> {
        let mut request = self.build();
        let query_request = request.query_custom_emoji();
        if let Some(filter) = filter {
            query_request.filter(filter);
        }
        request.send_single::<QueryResponse>().await
    }

    #[maybe_async::maybe_async]
    pub async fn custom_emoji_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<CustomEmoji<Get>>> {
        let mut request = self.build();
        request
            .changes_custom_emoji(since_state)
            .max_changes(max_changes);
        request.send_single().await
    }
}

impl Request<'_> {
    pub fn get_custom_emoji(&mut self) -> &mut GetRequest<CustomEmoji<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::GetCustomEmoji,
            Arguments::custom_emoji_get(self.params(Method::GetCustomEmoji)),
        )
        .custom_emoji_get_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_get_custom_emoji(self) -> crate::Result<CustomEmojiGetResponse> {
        self.send_single().await
    }

    pub fn changes_custom_emoji(&mut self, since_state: impl Into<String>) -> &mut ChangesRequest {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::ChangesCustomEmoji,
            Arguments::changes(self.params(Method::ChangesCustomEmoji), since_state.into()),
        )
        .changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_changes_custom_emoji(
        self,
    ) -> crate::Result<ChangesResponse<CustomEmoji<Get>>> {
        self.send_single().await
    }

    pub fn query_custom_emoji(&mut self) -> &mut QueryRequest<CustomEmoji<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::QueryCustomEmoji,
            Arguments::custom_emoji_query(self.params(Method::QueryCustomEmoji)),
        )
        .custom_emoji_query_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_custom_emoji(self) -> crate::Result<QueryResponse> {
        self.send_single().await
    }

    pub fn query_custom_emoji_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<CustomEmoji<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::QueryChangesCustomEmoji,
            Arguments::custom_emoji_query_changes(
                self.params(Method::QueryChangesCustomEmoji),
                since_query_state.into(),
            ),
        )
        .custom_emoji_query_changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_custom_emoji_changes(self) -> crate::Result<QueryChangesResponse> {
        self.send_single().await
    }

    pub fn set_custom_emoji(&mut self) -> &mut SetRequest<CustomEmoji<Set>> {
        self.add_capability(URI::Chat);
        self.add_method_call(
            Method::SetCustomEmoji,
            Arguments::custom_emoji_set(self.params(Method::SetCustomEmoji)),
        )
        .custom_emoji_set_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_set_custom_emoji(self) -> crate::Result<CustomEmojiSetResponse> {
        self.send_single().await
    }
}
