//! SAF — message broker public factory surface.
//!
//! All factory functions are implemented as methods on [`BrokerSvc`].

use crate::api::broker::broker_error::BrokerError;
use crate::api::broker::message_broker::MessageBroker;
use crate::api::types::backend_kind::BackendKind;
use crate::api::types::broker_svc::BrokerSvc;
use crate::api::types::message_broker_config::MessageBrokerConfig;
#[cfg(feature = "tokio-rt")]
use crate::core::broker::InMemoryMessageBroker;
#[cfg(feature = "nats")]
use crate::core::broker::NatsMessageBroker;

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
    pub fn in_memory_broker() -> Box<dyn MessageBroker> {
        Box::new(InMemoryMessageBroker::new())
    }

    /// Connect to a NATS server and return a broker handle.
    ///
    /// # Errors
    ///
    /// Returns [`BrokerError::Connection`] if the NATS server is unreachable.
    ///
    /// Requires the `nats` feature.
    #[cfg(feature = "nats")]
    pub async fn nats_broker(url: &str) -> Result<Box<dyn MessageBroker>, BrokerError> {
        NatsMessageBroker::connect(url)
            .await
            .map(|b| Box::new(b) as Box<dyn MessageBroker>)
    }

    /// Construct and wire a broker from a loaded [`MessageBrokerConfig`].
    ///
    /// This is the factory referenced by ADR-006: a `FeatureState::Enabled(cfg)`
    /// obtained from the configbuilder feature registry flows directly into this
    /// method to produce a ready-to-use broker.
    ///
    /// The backend is selected by [`MessageBrokerConfig::backend`]:
    /// - [`BackendKind::InMemory`] builds an in-process broadcast broker
    ///   (requires the `tokio-rt` feature).
    /// - [`BackendKind::Nats`] connects to the configured `url`
    ///   (requires the `nats` feature).
    ///
    /// # Errors
    ///
    /// - [`BrokerError::Unavailable`] if the requested backend's Cargo feature
    ///   is not compiled in.
    /// - [`BrokerError::Connection`] if a NATS connection cannot be established,
    ///   or if `backend = "nats"` was loaded without a `url`.
    pub async fn from_config(
        config: &MessageBrokerConfig,
    ) -> Result<Box<dyn MessageBroker>, BrokerError> {
        match config.backend {
            BackendKind::InMemory => {
                #[cfg(feature = "tokio-rt")]
                {
                    Ok(Box::new(InMemoryMessageBroker::new()) as Box<dyn MessageBroker>)
                }
                #[cfg(not(feature = "tokio-rt"))]
                {
                    Err(BrokerError::Unavailable(
                        "in_memory backend requires the `tokio-rt` feature".to_owned(),
                    ))
                }
            }
            BackendKind::Nats => {
                #[cfg(feature = "nats")]
                {
                    let url = config.url.as_deref().ok_or_else(|| {
                        BrokerError::Connection(
                            "nats backend requires a `url` but none was configured".to_owned(),
                        )
                    })?;
                    NatsMessageBroker::connect(url)
                        .await
                        .map(|b| Box::new(b) as Box<dyn MessageBroker>)
                }
                #[cfg(not(feature = "nats"))]
                {
                    Err(BrokerError::Unavailable(
                        "nats backend requires the `nats` feature".to_owned(),
                    ))
                }
            }
        }
    }

    /// Validate a value that implements [`Validator`].
    pub fn validate<V: crate::api::traits::Validator>(v: &V) -> Result<(), String> {
        v.validate()
    }
}
