//! [`MessageBroker`] — runtime-agnostic cross-process pub/sub contract.

use futures::future::BoxFuture;

use crate::api::error::broker_error::BrokerError;
use crate::api::types::message_stream::MessageStream;
use crate::api::vo::message::Message;

/// Cross-process publish/subscribe broker contract.
///
/// Implement this trait to plug in any broker backend — an in-process broker,
/// a network message bus, or a custom transport. This crate ships the no-op
/// reference broker via [`crate::BrokerSvc::noop_broker`]; production backends
/// are constructed by `swe-edge-runtime`.
///
/// # Publish semantics
///
/// Publishing to a topic with no active subscribers silently succeeds (fire and
/// forget).  Implementations that require a subscriber to exist before publishing
/// should document this constraint explicitly.
///
/// # Subscribe semantics
///
/// Each call to [`subscribe`] returns an independent stream.  All active
/// subscribers receive every message published after the subscription was
/// established.  Messages published before [`subscribe`] is called are not
/// delivered.
///
/// [`subscribe`]: MessageBroker::subscribe
pub trait MessageBroker: Send + Sync {
    /// Publish `msg` to `topic`, delivering it to all active subscribers.
    fn publish<'a>(
        &'a self,
        topic: &'a str,
        msg: Message,
    ) -> BoxFuture<'a, Result<(), BrokerError>>;

    /// Subscribe to `topic`, returning a stream of incoming messages.
    fn subscribe<'a>(&'a self, topic: &'a str)
        -> BoxFuture<'a, Result<MessageStream, BrokerError>>;

    /// Probe broker connectivity. Returns `Ok(())` if the broker is reachable.
    fn health_check(&self) -> BoxFuture<'_, Result<(), BrokerError>>;
}
