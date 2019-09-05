use crate::types;
use futures::prelude::*;
use std::{error, io};

pub use self::http::HttpClientPool;

pub mod http;

/// References to objects that can act as clients.
pub trait RawClientRef<'a> {
    /// Future returned by `request`.
    type Request: Future<Output = Result<types::Response, Self::Error>> + 'a;
    /// Error that can happen during a request.
    type Error: error::Error;

    // TODO: decide proper type for `target`
    fn request(self, target: &str, request: types::Request) -> Self::Request;
}

pub trait RawClientRefPubSub<'a> {
    type Subscription: Stream<Item = types::Response> + 'a;
    type Request: Future<Output = Result<(types::Response, Self::Subscription), io::Error>> + 'a;

    // TODO: decide proper type for `target`
    fn request_subscribe(self, target: &str, request: types::Request) -> Self::Request;
}