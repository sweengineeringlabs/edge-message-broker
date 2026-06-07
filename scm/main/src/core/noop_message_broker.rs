//! `NoopMessageBroker` — the contract crate's reference no-op broker.
//!
//! Publishing discards the message; subscribing yields an immediately-empty
//! stream. Real backends live in `swe-edge-runtime` — this crate ships only the
//! contract plus this no-op so the [`MessageBroker`] trait has an in-tree
//! implementation for contract tests.

use futures::future::BoxFuture;

use crate::api::error::broker_error::BrokerError;
use crate::api::traits::message_broker::MessageBroker;
use crate::api::types::message::Message;
use crate::api::types::message_stream::MessageStream;

/// No-op [`MessageBroker`]: `publish` succeeds without delivery, `subscribe`
/// returns an empty stream, `health_check` always reports healthy.
pub(crate) struct NoopMessageBroker;

impl MessageBroker for NoopMessageBroker {
    fn publish<'a>(
        &'a self,
        _topic: &'a str,
        _msg: Message,
    ) -> BoxFuture<'a, Result<(), BrokerError>> {
        Box::pin(async { Ok(()) })
    }

    fn subscribe<'a>(
        &'a self,
        _topic: &'a str,
    ) -> BoxFuture<'a, Result<MessageStream, BrokerError>> {
        Box::pin(async {
            let stream: MessageStream = Box::pin(futures::stream::empty());
            Ok(stream)
        })
    }

    fn health_check(&self) -> BoxFuture<'_, Result<(), BrokerError>> {
        Box::pin(async { Ok(()) })
    }
}
