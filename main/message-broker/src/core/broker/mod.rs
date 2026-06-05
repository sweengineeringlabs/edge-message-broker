//! Broker implementations.
//!
//! Only the no-op reference broker lives here; real backends (in-memory tokio
//! broadcast, NATS) are owned by `swe-edge-runtime`.

pub(crate) mod noop_message_broker;

pub(crate) use noop_message_broker::NoopMessageBroker;
