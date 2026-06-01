//! SAF — message broker public factory surface.
//!
//! All factory functions are implemented as methods on [`BrokerSvc`].

#[cfg(feature = "nats")]
use crate::api::broker::broker_error::BrokerError;
#[cfg(any(feature = "tokio-rt", feature = "nats"))]
use crate::api::broker::message_broker::MessageBroker;
#[cfg(feature = "tokio-rt")]
use crate::core::broker::InMemoryMessageBroker;
#[cfg(feature = "nats")]
use crate::core::broker::NatsMessageBroker;
use crate::api::types::broker_svc::BrokerSvc;

impl BrokerSvc {
    /// Return a [`ConfigBuilderImpl`] pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Construct an in-memory broker backed by [`tokio::sync::broadcast`].
    ///
    /// Topics are created lazily on first subscription.  All subscribers on the
    /// same topic receive every message published after they subscribed.
    ///
    /// Requires the `tokio-rt` feature.
    #[cfg(feature = "tokio-rt")]
    pub fn in_memory_broker() -> impl MessageBroker + Clone {
        InMemoryMessageBroker::new()
    }

    /// Connect to a NATS server and return a broker handle.
    ///
    /// # Errors
    ///
    /// Returns [`BrokerError::Connection`] if the NATS server is unreachable.
    ///
    /// Requires the `nats` feature.
    #[cfg(feature = "nats")]
    pub async fn nats_broker(url: &str) -> Result<impl MessageBroker, BrokerError> {
        NatsMessageBroker::connect(url).await
    }

    /// Validate a value that implements [`Validator`].
    pub fn validate<V: crate::api::traits::Validator>(v: &V) -> Result<(), String> {
        v.validate()
    }
}
