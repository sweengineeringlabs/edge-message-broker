//! SAF — message broker public factory surface.
//!
//! All factory functions are implemented as methods on [`BrokerSvc`].
//!
//! This crate is the **contract**: it ships the [`MessageBroker`] trait, the
//! message/stream/error value types, the configuration vocabulary, and a single
//! no-op reference broker. Real backends (in-memory tokio broadcast, NATS) and
//! the `from_config` construction factory live in `swe-edge-runtime`.

use crate::api::broker::message_broker::MessageBroker;
use crate::api::types::broker_svc::BrokerSvc;
use crate::core::broker::NoopMessageBroker;

impl BrokerSvc {
    /// Return a [`ConfigBuilderImpl`] pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Construct the no-op reference broker.
    ///
    /// Publishing discards the message and subscribing yields an empty stream.
    /// Intended for tests and as a safe default; production deployments inject a
    /// real backend from `swe-edge-runtime`.
    pub fn noop_broker() -> Box<dyn MessageBroker> {
        Box::new(NoopMessageBroker)
    }

    /// Validate a value that implements [`Validator`].
    pub fn validate<V: crate::api::traits::Validator>(v: &V) -> Result<(), String> {
        v.validate()
    }
}
