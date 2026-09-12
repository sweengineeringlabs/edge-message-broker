//! Extension point marker for downstream broker substitution.

use std::sync::Arc;

use crate::api::{
    BrokerError, BrokerFuture, HealthCheckRequest, MessageBroker, MessageStream, PublishRequest,
    SubscribeRequest, SubscribeResponse, ValidatorRequest, ValidatorResponse,
};
use crate::core::noop_validator::NoopValidator;

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
        _request: PublishRequest,
    ) -> BrokerFuture<'a, Result<(), BrokerError>> {
        BrokerFuture::new(async { Ok(()) })
    }

    fn subscribe<'a>(
        &'a self,
        _request: SubscribeRequest,
    ) -> BrokerFuture<'a, Result<SubscribeResponse, BrokerError>> {
        BrokerFuture::new(async {
            let stream: MessageStream = Box::pin(futures::stream::empty());
            Ok(SubscribeResponse { stream })
        })
    }

    fn health_check(
        &self,
        _request: HealthCheckRequest,
    ) -> BrokerFuture<'_, Result<(), BrokerError>> {
        BrokerFuture::new(async { Ok(()) })
    }

    fn validator(&self, _request: ValidatorRequest) -> Result<ValidatorResponse, BrokerError> {
        Ok(ValidatorResponse {
            validator: Arc::new(NoopValidator),
        })
    }
}
