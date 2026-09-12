//! SAF — message broker public factory surface.
//!
//! All factory functions are implemented as methods on [`BrokerSvc`].
//!
//! This crate is the **contract**: it ships the [`MessageBroker`] trait, the
//! message/stream/error value types, the configuration vocabulary, and a single
//! no-op reference broker. Real backends (in-memory tokio broadcast, NATS) and
//! the `from_config` construction factory live in `swe-edge-runtime`.

use configbuilder::ConfigBuilder;

use crate::api::{BrokerSvc, MessageBroker, ValidationError, ValidationRequest, Validator};
use crate::core::{NoopMessageBroker, NoopValidator};

impl BrokerSvc {
    /// Return a [`ConfigBuilderImpl`](configbuilder::ConfigBuilderImpl) pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> configbuilder::ConfigBuilderImpl {
        configbuilder::ConfigLoaderFactory::create_config_builder()
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
    pub fn validate<V: Validator>(v: &V) -> Result<(), ValidationError> {
        v.validate(ValidationRequest)
    }

    /// Construct the no-op reference [`MessageBroker`] (an alias for
    /// [`BrokerSvc::noop_broker`], named for `saf/message_broker_svc_factory.rs`'s
    /// composition-site convention).
    pub fn create_message_broker() -> Box<dyn MessageBroker> {
        Self::noop_broker()
    }

    /// Construct the always-valid reference [`Validator`], for consumers with
    /// no domain value of their own to validate yet.
    pub fn create_validator() -> Box<dyn Validator> {
        Box::new(NoopValidator)
    }
}
