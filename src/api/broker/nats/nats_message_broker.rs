//! API interface surface for the NATS broker implementation.
//!
//! This is the SEA api/ counterpart for
//! `core/broker/nats/broker.rs`.
//! The implementation is accessed via [`crate::BrokerSvc::nats_broker`].

/// API marker type identifying the NATS broker.
///
/// Consumers use this type only as a type tag; the actual broker instance is
/// obtained via [`crate::BrokerSvc::nats_broker`] which returns `impl MessageBroker`.
pub struct NatsMessageBroker;
