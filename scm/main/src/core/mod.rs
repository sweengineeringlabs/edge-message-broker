//! Core layer — in-house [`crate::api::traits::MessageBroker`] implementations.
//!
//! Only the no-op reference broker lives here; real backends are owned by
//! `swe-edge-runtime`.

pub(crate) mod broker_future_ops;
pub(crate) mod message_broker_config_validation;
pub(crate) mod message_factory;
pub(crate) mod noop_message_broker;
pub(crate) mod noop_validator;

pub(crate) use noop_message_broker::NoopMessageBroker;
pub(crate) use noop_validator::NoopValidator;
