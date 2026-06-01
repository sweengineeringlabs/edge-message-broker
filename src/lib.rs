//! `swe_edge_message_broker` — cross-process pub/sub broker.
//!
//! Provides a runtime-agnostic [`MessageBroker`] trait for cross-process
//! publish/subscribe messaging.  Use [`BrokerSvc::in_memory_broker`] for testing and
//! local services, [`BrokerSvc::nats_broker`] for NATS-backed production deployments.

mod api;
mod core;
mod gateway;
mod saf;
mod spi;

pub use gateway::*;
