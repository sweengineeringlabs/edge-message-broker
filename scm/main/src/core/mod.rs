//! Core layer — in-house [`crate::api::traits::MessageBroker`] implementations.
//!
//! Only the no-op reference broker lives here; real backends are owned by
//! `swe-edge-runtime`.

pub(crate) mod noop_message_broker;

pub(crate) use noop_message_broker::NoopMessageBroker;
