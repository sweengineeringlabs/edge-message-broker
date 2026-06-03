//! `swe_edge_message_broker` — cross-process pub/sub broker.
//!
//! Provides a runtime-agnostic [`MessageBroker`] trait for cross-process
//! publish/subscribe messaging.  Use [`BrokerSvc::in_memory_broker`] for testing and
//! local services, [`BrokerSvc::nats_broker`] for NATS-backed production deployments.

// `unwrap`/`expect` are denied in production code (Cargo.toml `[lints.clippy]`)
// but are the idiomatic assertion mechanism in inline `#[cfg(test)]` modules.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod gateway;
mod saf;
mod spi;

pub use gateway::*;
