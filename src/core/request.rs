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
    blob::copy::CopyBlobRequest,
    chat::{Chat, TypingRequest},
    chat_contact::ChatContact,
    chat_message::Message as ChatMessage,
    client::Client,
    custom_emoji::CustomEmoji,
    email::{
        import::EmailImportRequest, parse::EmailParseRequest,
        search_snippet::SearchSnippetGetRequest, Email,
    },
    email_submission::EmailSubmission,
    identity::Identity,
    mailbox::Mailbox,
    presence_status::PresenceStatus,
    principal::Principal,
    push_subscription::PushSubscription,
    read_position::ReadPosition,
    sieve::{validate::SieveScriptValidateRequest, SieveScript},
    space::{Space, SpaceJoinRequest},
    space_ban::SpaceBan,
    space_invite::SpaceInvite,
    thread::Thread,
    vacation_response::VacationResponse,
    Error, Method, Set, URI,
};
use ahash::AHashMap;
use serde::{de::DeserializeOwned, Serialize};

use super::{
    changes::ChangesRequest,
    copy::CopyRequest,
    get::GetRequest,
    query::QueryRequest,
    query_changes::QueryChangesRequest,
    response::{Response, SingleMethodResponse, TaggedMethodResponse},
    set::SetRequest,
    RequestParams,
};

#[derive(Serialize)]
pub struct Request<'x> {
    #[serde(skip)]
    client: &'x Client,
    #[serde(skip)]
    account_id: String,

    pub using: Vec<URI>,

    #[serde(rename = "methodCalls")]
    pub method_calls: Vec<(Method, Arguments, String)>,

    #[serde(rename = "createdIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_ids: Option<AHashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResultReference {
    #[serde(rename = "resultOf")]
    result_of: String,
    name: Method,
    path: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Arguments {
    Changes(ChangesRequest),
    PushGet(GetRequest<PushSubscription<Set>>),
    PushSet(SetRequest<PushSubscription<Set>>),
    BlobCopy(CopyBlobRequest),
    MailboxGet(GetRequest<Mailbox<Set>>),
    MailboxQuery(QueryRequest<Mailbox<Set>>),
    MailboxQueryChanges(QueryChangesRequest<Mailbox<Set>>),
    MailboxSet(SetRequest<Mailbox<Set>>),
    ThreadGet(GetRequest<Thread>),
    EmailGet(GetRequest<Email<Set>>),
    EmailQuery(QueryRequest<Email<Set>>),
    EmailQueryChanges(QueryChangesRequest<Email<Set>>),
    EmailSet(SetRequest<Email<Set>>),
    EmailCopy(CopyRequest<Email<Set>>),
    EmailImport(EmailImportRequest),
    EmailParse(EmailParseRequest),
    SearchSnippetGet(SearchSnippetGetRequest),
    IdentityGet(GetRequest<Identity<Set>>),
    IdentitySet(SetRequest<Identity<Set>>),
    EmailSubmissionGet(GetRequest<EmailSubmission<Set>>),
    EmailSubmissionQuery(QueryRequest<EmailSubmission<Set>>),
    EmailSubmissionQueryChanges(QueryChangesRequest<EmailSubmission<Set>>),
    EmailSubmissionSet(SetRequest<EmailSubmission<Set>>),
    VacationResponseGet(GetRequest<VacationResponse<Set>>),
    VacationResponseSet(SetRequest<VacationResponse<Set>>),
    SieveScriptGet(GetRequest<SieveScript<Set>>),
    SieveScriptQuery(QueryRequest<SieveScript<Set>>),
    SieveScriptValidate(SieveScriptValidateRequest),
    SieveScriptSet(SetRequest<SieveScript<Set>>),
    PrincipalGet(GetRequest<Principal<Set>>),
    PrincipalQuery(QueryRequest<Principal<Set>>),
    PrincipalQueryChanges(QueryChangesRequest<Principal<Set>>),
    PrincipalSet(SetRequest<Principal<Set>>),
    // Chat
    ChatContactGet(GetRequest<ChatContact<Set>>),
    ChatContactSet(SetRequest<ChatContact<Set>>),
    ChatContactQuery(QueryRequest<ChatContact<Set>>),
    ChatContactQueryChanges(QueryChangesRequest<ChatContact<Set>>),
    ChatGet(GetRequest<Chat<Set>>),
    ChatSet(SetRequest<Chat<Set>>),
    ChatQuery(QueryRequest<Chat<Set>>),
    ChatQueryChanges(QueryChangesRequest<Chat<Set>>),
    TypingChat(TypingRequest),
    ChatMessageGet(GetRequest<ChatMessage<Set>>),
    ChatMessageSet(SetRequest<ChatMessage<Set>>),
    ChatMessageQuery(QueryRequest<ChatMessage<Set>>),
    ChatMessageQueryChanges(QueryChangesRequest<ChatMessage<Set>>),
    SpaceGet(GetRequest<Space<Set>>),
    SpaceSet(SetRequest<Space<Set>>),
    SpaceQuery(QueryRequest<Space<Set>>),
    SpaceQueryChanges(QueryChangesRequest<Space<Set>>),
    SpaceJoin(SpaceJoinRequest),
    ReadPositionGet(GetRequest<ReadPosition<Set>>),
    ReadPositionSet(SetRequest<ReadPosition<Set>>),
    PresenceStatusGet(GetRequest<PresenceStatus<Set>>),
    PresenceStatusSet(SetRequest<PresenceStatus<Set>>),
    CustomEmojiGet(GetRequest<CustomEmoji<Set>>),
    CustomEmojiSet(SetRequest<CustomEmoji<Set>>),
    CustomEmojiQuery(QueryRequest<CustomEmoji<Set>>),
    CustomEmojiQueryChanges(QueryChangesRequest<CustomEmoji<Set>>),
    SpaceInviteGet(GetRequest<SpaceInvite<Set>>),
    SpaceInviteSet(SetRequest<SpaceInvite<Set>>),
    SpaceBanGet(GetRequest<SpaceBan<Set>>),
    SpaceBanSet(SetRequest<SpaceBan<Set>>),
}

impl Arguments {
    pub fn changes(params: RequestParams, since_state: String) -> Self {
        Arguments::Changes(ChangesRequest::new(params, since_state))
    }

    pub fn push_get(params: RequestParams) -> Self {
        Arguments::PushGet(GetRequest::new(params))
    }

    pub fn push_set(params: RequestParams) -> Self {
        Arguments::PushSet(SetRequest::new(params))
    }

    pub fn blob_copy(params: RequestParams, from_account_id: String) -> Self {
        Arguments::BlobCopy(CopyBlobRequest::new(params, from_account_id))
    }

    pub fn mailbox_get(params: RequestParams) -> Self {
        Arguments::MailboxGet(GetRequest::new(params))
    }

    pub fn mailbox_query(params: RequestParams) -> Self {
        Arguments::MailboxQuery(QueryRequest::new(params))
    }

    pub fn mailbox_query_changes(params: RequestParams, since_query_state: String) -> Self {
        Arguments::MailboxQueryChanges(QueryChangesRequest::new(params, since_query_state))
    }

    pub fn mailbox_set(params: RequestParams) -> Self {
        Arguments::MailboxSet(SetRequest::new(params))
    }

    pub fn thread_get(params: RequestParams) -> Self {
        Arguments::ThreadGet(GetRequest::new(params))
    }

    pub fn email_get(params: RequestParams) -> Self {
        Arguments::EmailGet(GetRequest::new(params))
    }

    pub fn email_query(params: RequestParams) -> Self {
        Arguments::EmailQuery(QueryRequest::new(params))
    }

    pub fn email_query_changes(params: RequestParams, since_query_state: String) -> Self {
        Arguments::EmailQueryChanges(QueryChangesRequest::new(params, since_query_state))
    }

    pub fn email_set(params: RequestParams) -> Self {
        Arguments::EmailSet(SetRequest::new(params))
    }

    pub fn email_copy(params: RequestParams, from_account_id: String) -> Self {
        Arguments::EmailCopy(CopyRequest::new(params, from_account_id))
    }

    pub fn email_import(params: RequestParams) -> Self {
        Arguments::EmailImport(EmailImportRequest::new(params))
    }

    pub fn email_parse(params: RequestParams) -> Self {
        Arguments::EmailParse(EmailParseRequest::new(params))
    }

    pub fn search_snippet_get(params: RequestParams) -> Self {
        Arguments::SearchSnippetGet(SearchSnippetGetRequest::new(params))
    }

    pub fn identity_get(params: RequestParams) -> Self {
        Arguments::IdentityGet(GetRequest::new(params))
    }

    pub fn identity_set(params: RequestParams) -> Self {
        Arguments::IdentitySet(SetRequest::new(params))
    }

    pub fn email_submission_get(params: RequestParams) -> Self {
        Arguments::EmailSubmissionGet(GetRequest::new(params))
    }

    pub fn email_submission_query(params: RequestParams) -> Self {
        Arguments::EmailSubmissionQuery(QueryRequest::new(params))
    }

    pub fn email_submission_query_changes(
        params: RequestParams,
        since_query_state: String,
    ) -> Self {
        Arguments::EmailSubmissionQueryChanges(QueryChangesRequest::new(params, since_query_state))
    }

    pub fn email_submission_set(params: RequestParams) -> Self {
        Arguments::EmailSubmissionSet(SetRequest::new(params))
    }

    pub fn vacation_response_get(params: RequestParams) -> Self {
        Arguments::VacationResponseGet(GetRequest::new(params))
    }

    pub fn vacation_response_set(params: RequestParams) -> Self {
        Arguments::VacationResponseSet(SetRequest::new(params))
    }

    pub fn sieve_script_get(params: RequestParams) -> Self {
        Arguments::SieveScriptGet(GetRequest::new(params))
    }

    pub fn sieve_script_query(params: RequestParams) -> Self {
        Arguments::SieveScriptQuery(QueryRequest::new(params))
    }

    pub fn sieve_script_validate(params: RequestParams, blob_id: impl Into<String>) -> Self {
        Arguments::SieveScriptValidate(SieveScriptValidateRequest::new(params, blob_id))
    }

    pub fn sieve_script_set(params: RequestParams) -> Self {
        Arguments::SieveScriptSet(SetRequest::new(params))
    }

    pub fn principal_get(params: RequestParams) -> Self {
        Arguments::PrincipalGet(GetRequest::new(params))
    }

    pub fn principal_query(params: RequestParams) -> Self {
        Arguments::PrincipalQuery(QueryRequest::new(params))
    }

    pub fn principal_query_changes(params: RequestParams, since_query_state: String) -> Self {
        Arguments::PrincipalQueryChanges(QueryChangesRequest::new(params, since_query_state))
    }

    pub fn principal_set(params: RequestParams) -> Self {
        Arguments::PrincipalSet(SetRequest::new(params))
    }

    pub fn changes_mut(&mut self) -> &mut ChangesRequest {
        match self {
            Arguments::Changes(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn push_get_mut(&mut self) -> &mut GetRequest<PushSubscription<Set>> {
        match self {
            Arguments::PushGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn push_set_mut(&mut self) -> &mut SetRequest<PushSubscription<Set>> {
        match self {
            Arguments::PushSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn blob_copy_mut(&mut self) -> &mut CopyBlobRequest {
        match self {
            Arguments::BlobCopy(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn mailbox_get_mut(&mut self) -> &mut GetRequest<Mailbox<Set>> {
        match self {
            Arguments::MailboxGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn mailbox_query_mut(&mut self) -> &mut QueryRequest<Mailbox<Set>> {
        match self {
            Arguments::MailboxQuery(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn mailbox_query_changes_mut(&mut self) -> &mut QueryChangesRequest<Mailbox<Set>> {
        match self {
            Arguments::MailboxQueryChanges(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn mailbox_set_mut(&mut self) -> &mut SetRequest<Mailbox<Set>> {
        match self {
            Arguments::MailboxSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn thread_get_mut(&mut self) -> &mut GetRequest<Thread> {
        match self {
            Arguments::ThreadGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_get_mut(&mut self) -> &mut GetRequest<Email<Set>> {
        match self {
            Arguments::EmailGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_query_mut(&mut self) -> &mut QueryRequest<Email<Set>> {
        match self {
            Arguments::EmailQuery(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_query_changes_mut(&mut self) -> &mut QueryChangesRequest<Email<Set>> {
        match self {
            Arguments::EmailQueryChanges(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_set_mut(&mut self) -> &mut SetRequest<Email<Set>> {
        match self {
            Arguments::EmailSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_copy_mut(&mut self) -> &mut CopyRequest<Email<Set>> {
        match self {
            Arguments::EmailCopy(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_import_mut(&mut self) -> &mut EmailImportRequest {
        match self {
            Arguments::EmailImport(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_parse_mut(&mut self) -> &mut EmailParseRequest {
        match self {
            Arguments::EmailParse(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn search_snippet_get_mut(&mut self) -> &mut SearchSnippetGetRequest {
        match self {
            Arguments::SearchSnippetGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn identity_get_mut(&mut self) -> &mut GetRequest<Identity<Set>> {
        match self {
            Arguments::IdentityGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn identity_set_mut(&mut self) -> &mut SetRequest<Identity<Set>> {
        match self {
            Arguments::IdentitySet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_submission_get_mut(&mut self) -> &mut GetRequest<EmailSubmission<Set>> {
        match self {
            Arguments::EmailSubmissionGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_submission_query_mut(&mut self) -> &mut QueryRequest<EmailSubmission<Set>> {
        match self {
            Arguments::EmailSubmissionQuery(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_submission_query_changes_mut(
        &mut self,
    ) -> &mut QueryChangesRequest<EmailSubmission<Set>> {
        match self {
            Arguments::EmailSubmissionQueryChanges(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn email_submission_set_mut(&mut self) -> &mut SetRequest<EmailSubmission<Set>> {
        match self {
            Arguments::EmailSubmissionSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn vacation_response_get_mut(&mut self) -> &mut GetRequest<VacationResponse<Set>> {
        match self {
            Arguments::VacationResponseGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn vacation_response_set_mut(&mut self) -> &mut SetRequest<VacationResponse<Set>> {
        match self {
            Arguments::VacationResponseSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn sieve_script_get_mut(&mut self) -> &mut GetRequest<SieveScript<Set>> {
        match self {
            Arguments::SieveScriptGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn sieve_script_query_mut(&mut self) -> &mut QueryRequest<SieveScript<Set>> {
        match self {
            Arguments::SieveScriptQuery(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn sieve_script_validate_mut(&mut self) -> &mut SieveScriptValidateRequest {
        match self {
            Arguments::SieveScriptValidate(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn sieve_script_set_mut(&mut self) -> &mut SetRequest<SieveScript<Set>> {
        match self {
            Arguments::SieveScriptSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn principal_get_mut(&mut self) -> &mut GetRequest<Principal<Set>> {
        match self {
            Arguments::PrincipalGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn principal_query_mut(&mut self) -> &mut QueryRequest<Principal<Set>> {
        match self {
            Arguments::PrincipalQuery(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn principal_query_changes_mut(&mut self) -> &mut QueryChangesRequest<Principal<Set>> {
        match self {
            Arguments::PrincipalQueryChanges(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn principal_set_mut(&mut self) -> &mut SetRequest<Principal<Set>> {
        match self {
            Arguments::PrincipalSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn chat_contact_get(params: RequestParams) -> Self {
        Arguments::ChatContactGet(GetRequest::new(params))
    }

    pub fn chat_contact_set(params: RequestParams) -> Self {
        Arguments::ChatContactSet(SetRequest::new(params))
    }

    pub fn chat_contact_query(params: RequestParams) -> Self {
        Arguments::ChatContactQuery(QueryRequest::new(params))
    }

    pub fn chat_contact_query_changes(params: RequestParams, since_query_state: String) -> Self {
        Arguments::ChatContactQueryChanges(QueryChangesRequest::new(params, since_query_state))
    }

    pub fn chat_get(params: RequestParams) -> Self {
        Arguments::ChatGet(GetRequest::new(params))
    }

    pub fn chat_set(params: RequestParams) -> Self {
        Arguments::ChatSet(SetRequest::new(params))
    }

    pub fn chat_query(params: RequestParams) -> Self {
        Arguments::ChatQuery(QueryRequest::new(params))
    }

    pub fn chat_query_changes(params: RequestParams, since_query_state: String) -> Self {
        Arguments::ChatQueryChanges(QueryChangesRequest::new(params, since_query_state))
    }

    pub fn typing_chat(params: RequestParams, chat_id: String, typing: bool) -> Self {
        Arguments::TypingChat(TypingRequest {
            account_id: params.account_id.clone(),
            chat_id,
            typing,
        })
    }

    pub fn chat_message_get(params: RequestParams) -> Self {
        Arguments::ChatMessageGet(GetRequest::new(params))
    }

    pub fn chat_message_set(params: RequestParams) -> Self {
        Arguments::ChatMessageSet(SetRequest::new(params))
    }

    pub fn chat_message_query(params: RequestParams) -> Self {
        Arguments::ChatMessageQuery(QueryRequest::new(params))
    }

    pub fn chat_message_query_changes(params: RequestParams, since_query_state: String) -> Self {
        Arguments::ChatMessageQueryChanges(QueryChangesRequest::new(params, since_query_state))
    }

    pub fn space_get(params: RequestParams) -> Self {
        Arguments::SpaceGet(GetRequest::new(params))
    }

    pub fn space_set(params: RequestParams) -> Self {
        Arguments::SpaceSet(SetRequest::new(params))
    }

    pub fn space_query(params: RequestParams) -> Self {
        Arguments::SpaceQuery(QueryRequest::new(params))
    }

    pub fn space_query_changes(params: RequestParams, since_query_state: String) -> Self {
        Arguments::SpaceQueryChanges(QueryChangesRequest::new(params, since_query_state))
    }

    pub fn space_join(request: SpaceJoinRequest) -> Self {
        Arguments::SpaceJoin(request)
    }

    pub fn read_position_get(params: RequestParams) -> Self {
        Arguments::ReadPositionGet(GetRequest::new(params))
    }

    pub fn read_position_set(params: RequestParams) -> Self {
        Arguments::ReadPositionSet(SetRequest::new(params))
    }

    pub fn presence_status_get(params: RequestParams) -> Self {
        Arguments::PresenceStatusGet(GetRequest::new(params))
    }

    pub fn presence_status_set(params: RequestParams) -> Self {
        Arguments::PresenceStatusSet(SetRequest::new(params))
    }

    pub fn custom_emoji_get(params: RequestParams) -> Self {
        Arguments::CustomEmojiGet(GetRequest::new(params))
    }

    pub fn custom_emoji_set(params: RequestParams) -> Self {
        Arguments::CustomEmojiSet(SetRequest::new(params))
    }

    pub fn custom_emoji_query(params: RequestParams) -> Self {
        Arguments::CustomEmojiQuery(QueryRequest::new(params))
    }

    pub fn custom_emoji_query_changes(params: RequestParams, since_query_state: String) -> Self {
        Arguments::CustomEmojiQueryChanges(QueryChangesRequest::new(params, since_query_state))
    }

    pub fn space_invite_get(params: RequestParams) -> Self {
        Arguments::SpaceInviteGet(GetRequest::new(params))
    }

    pub fn space_invite_set(params: RequestParams) -> Self {
        Arguments::SpaceInviteSet(SetRequest::new(params))
    }

    pub fn space_ban_get(params: RequestParams) -> Self {
        Arguments::SpaceBanGet(GetRequest::new(params))
    }

    pub fn space_ban_set(params: RequestParams) -> Self {
        Arguments::SpaceBanSet(SetRequest::new(params))
    }

    pub fn chat_contact_get_mut(&mut self) -> &mut GetRequest<ChatContact<Set>> {
        match self {
            Arguments::ChatContactGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn chat_contact_set_mut(&mut self) -> &mut SetRequest<ChatContact<Set>> {
        match self {
            Arguments::ChatContactSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn chat_contact_query_mut(&mut self) -> &mut QueryRequest<ChatContact<Set>> {
        match self {
            Arguments::ChatContactQuery(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn chat_contact_query_changes_mut(&mut self) -> &mut QueryChangesRequest<ChatContact<Set>> {
        match self {
            Arguments::ChatContactQueryChanges(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn chat_get_mut(&mut self) -> &mut GetRequest<Chat<Set>> {
        match self {
            Arguments::ChatGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn chat_set_mut(&mut self) -> &mut SetRequest<Chat<Set>> {
        match self {
            Arguments::ChatSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn chat_query_mut(&mut self) -> &mut QueryRequest<Chat<Set>> {
        match self {
            Arguments::ChatQuery(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn chat_query_changes_mut(&mut self) -> &mut QueryChangesRequest<Chat<Set>> {
        match self {
            Arguments::ChatQueryChanges(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn chat_message_get_mut(&mut self) -> &mut GetRequest<ChatMessage<Set>> {
        match self {
            Arguments::ChatMessageGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn chat_message_set_mut(&mut self) -> &mut SetRequest<ChatMessage<Set>> {
        match self {
            Arguments::ChatMessageSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn chat_message_query_mut(&mut self) -> &mut QueryRequest<ChatMessage<Set>> {
        match self {
            Arguments::ChatMessageQuery(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn chat_message_query_changes_mut(&mut self) -> &mut QueryChangesRequest<ChatMessage<Set>> {
        match self {
            Arguments::ChatMessageQueryChanges(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn space_get_mut(&mut self) -> &mut GetRequest<Space<Set>> {
        match self {
            Arguments::SpaceGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn space_set_mut(&mut self) -> &mut SetRequest<Space<Set>> {
        match self {
            Arguments::SpaceSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn space_query_mut(&mut self) -> &mut QueryRequest<Space<Set>> {
        match self {
            Arguments::SpaceQuery(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn space_query_changes_mut(&mut self) -> &mut QueryChangesRequest<Space<Set>> {
        match self {
            Arguments::SpaceQueryChanges(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn read_position_get_mut(&mut self) -> &mut GetRequest<ReadPosition<Set>> {
        match self {
            Arguments::ReadPositionGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn read_position_set_mut(&mut self) -> &mut SetRequest<ReadPosition<Set>> {
        match self {
            Arguments::ReadPositionSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn presence_status_get_mut(&mut self) -> &mut GetRequest<PresenceStatus<Set>> {
        match self {
            Arguments::PresenceStatusGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn presence_status_set_mut(&mut self) -> &mut SetRequest<PresenceStatus<Set>> {
        match self {
            Arguments::PresenceStatusSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn custom_emoji_get_mut(&mut self) -> &mut GetRequest<CustomEmoji<Set>> {
        match self {
            Arguments::CustomEmojiGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn custom_emoji_set_mut(&mut self) -> &mut SetRequest<CustomEmoji<Set>> {
        match self {
            Arguments::CustomEmojiSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn custom_emoji_query_mut(&mut self) -> &mut QueryRequest<CustomEmoji<Set>> {
        match self {
            Arguments::CustomEmojiQuery(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn custom_emoji_query_changes_mut(&mut self) -> &mut QueryChangesRequest<CustomEmoji<Set>> {
        match self {
            Arguments::CustomEmojiQueryChanges(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn space_invite_get_mut(&mut self) -> &mut GetRequest<SpaceInvite<Set>> {
        match self {
            Arguments::SpaceInviteGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn space_invite_set_mut(&mut self) -> &mut SetRequest<SpaceInvite<Set>> {
        match self {
            Arguments::SpaceInviteSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn space_ban_get_mut(&mut self) -> &mut GetRequest<SpaceBan<Set>> {
        match self {
            Arguments::SpaceBanGet(ref mut r) => r,
            _ => unreachable!(),
        }
    }

    pub fn space_ban_set_mut(&mut self) -> &mut SetRequest<SpaceBan<Set>> {
        match self {
            Arguments::SpaceBanSet(ref mut r) => r,
            _ => unreachable!(),
        }
    }
}

impl<'x> Request<'x> {
    pub fn new(client: &'x Client) -> Self {
        Request {
            using: vec![URI::Core, URI::Mail],
            method_calls: vec![],
            created_ids: None,
            account_id: client.default_account_id().to_string(),
            client,
        }
    }

    pub fn account_id(mut self, account_id: impl Into<String>) -> Self {
        self.account_id = account_id.into();
        self
    }

    #[maybe_async::maybe_async]
    pub async fn send(self) -> crate::Result<Response<TaggedMethodResponse>> {
        self.client.send(&self).await
    }

    #[cfg(feature = "websockets")]
    pub async fn send_ws(self) -> crate::Result<String> {
        self.client.send_ws(self).await
    }

    #[maybe_async::maybe_async]
    pub async fn send_single<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        let response: Response<SingleMethodResponse<T>> = self.client.send(&self).await?;
        match response
            .unwrap_method_responses()
            .pop()
            .ok_or_else(|| Error::Internal("Server returned no results".to_string()))?
        {
            SingleMethodResponse::Ok((_, response, _)) => Ok(response),
            SingleMethodResponse::Error((_, err, _)) => Err(err.into()),
        }
    }

    pub fn params(&self, method: Method) -> RequestParams {
        RequestParams {
            account_id: self.account_id.clone(),
            method,
            call_id: self.method_calls.len(),
        }
    }

    pub fn add_method_call(&mut self, method: Method, arguments: Arguments) -> &mut Arguments {
        let call_id = format!("s{}", self.method_calls.len());
        self.method_calls.push((method, arguments, call_id));
        &mut self.method_calls.last_mut().unwrap().1
    }

    pub fn add_capability(&mut self, uri: URI) {
        if !self.using.contains(&uri) {
            self.using.push(uri);
        }
    }

    pub fn last_result_reference(&self, path: impl Into<String>) -> ResultReference {
        let last_method = self.method_calls.last().unwrap();
        ResultReference {
            result_of: last_method.2.clone(),
            name: last_method.0,
            path: path.into(),
        }
    }
}

impl ResultReference {
    pub fn new(method: Method, call_id: usize, path: impl Into<String>) -> Self {
        ResultReference {
            result_of: format!("s{}", call_id),
            name: method,
            path: path.into(),
        }
    }
}
