//! `swe_edge_message_broker` — cross-process pub/sub broker.
//!
//! Provides a runtime-agnostic [`MessageBroker`] trait for cross-process
//! publish/subscribe messaging, the message/stream/error value types, and the
//! configuration vocabulary ([`MessageBrokerConfig`], [`BackendKind`]).
//!
//! This crate is the **contract**. It ships a single no-op reference broker
//! ([`BrokerSvc::noop_broker`]); real backends (in-memory tokio broadcast, NATS)
//! and the `from_config` construction factory live in `swe-edge-runtime`.

// `unwrap`/`expect` are denied in production code (Cargo.toml `[lints.clippy]`)
// but are the idiomatic assertion mechanism in inline `#[cfg(test)]` modules.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod gateway;
mod saf;
mod spi;

pub use gateway::*;
