use dialkit::{
    AccountSid, ApiError, Call, CallInstructions, CallSid, CallStatus, Calls, Client, CreateCall,
    CreateMessage, Credentials, Error, ListCalls, ListMessages, Message, MessageSender, MessageSid,
    MessageStatus, Messages, Page, Pager, PhoneEndpoint, RetryPolicy,
};
use std::fmt::Debug;
use url::Url;

#[test]
fn existing_public_symbols_and_signatures_remain_available() {
    fn assert_send_sync<T: Send + Sync>() {}
    fn assert_debug<T: Debug>() {}
    assert_send_sync::<Client>();
    assert_debug::<Credentials>();
    assert_debug::<Error>();
    assert_debug::<ApiError>();
    let _: Option<Calls> = None;
    let _: Option<Messages> = None;
    let _: Option<Call> = None;
    let _: Option<Message> = None;
    let _: Option<Pager<Call>> = None;
    let _: Option<Page<Message>> = None;
    let _: Option<CallStatus> = None;
    let _: Option<MessageStatus> = None;
    let _ = std::any::TypeId::of::<ListCalls>();
    let _ = std::any::TypeId::of::<ListMessages>();
    let _ = std::any::TypeId::of::<CreateCall>();
    let _ = std::any::TypeId::of::<CreateMessage>();
    let _ = std::any::TypeId::of::<CallInstructions>();
    let _ = std::any::TypeId::of::<MessageSender>();
    let _ = std::any::TypeId::of::<PhoneEndpoint>();
    let _ = std::any::TypeId::of::<CallSid>();
    let _ = std::any::TypeId::of::<MessageSid>();
}

#[test]
fn base_url_and_messaging_base_url_are_independent_builder_settings() {
    let builder = Client::builder(Credentials::account_token(
        AccountSid::new("AC00000000000000000000000000000000").unwrap(),
        "synthetic-compatibility-token",
    ))
    .base_url(Url::parse("http://127.0.0.1:4101/").unwrap())
    .messaging_base_url(Url::parse("http://127.0.0.1:4102/").unwrap())
    .retry_policy(RetryPolicy::conservative());
    assert!(builder.build().is_ok());
}
