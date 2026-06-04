//! Extension point marker for downstream broker substitution.

use crate::api::broker::broker_error::BrokerError;
use crate::api::broker::message::message::Message;
use crate::api::broker::message_broker::MessageBroker;
use crate::api::broker::stream::MessageStream;
use futures::future::BoxFuture;

/// Extension point marker for downstream broker substitution.
///
/// Downstream consumers may implement `MessageBroker` on this type to signal
/// compatibility with the `swe-edge-message-broker` SPI surface.
/// Extension point marker for downstream broker substitution.
pub(crate) struct BrokerBackend;

// Silence dead_code: BrokerBackend IS constructed in tests to verify the SPI contract.
const _: () = {
    let _ = std::mem::size_of::<BrokerBackend>();
};

impl MessageBroker for BrokerBackend {
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
