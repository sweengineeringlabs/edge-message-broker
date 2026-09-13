//! [`MessageBrokerConfig`] — backend-owned `[message_broker]` TOML contract.

use crate::api::BackendKind;

/// Canonical configuration for the `[message_broker]` TOML section.
///
/// Per ADR-006, this struct is owned by `swe-edge-message-broker`: the crate
/// that implements the broker also defines the section name, field shape, and
/// validation rules. Consumers opt in by adding this crate to `Cargo.toml` and
/// a `[message_broker]` section to `application.toml` — they never redefine the
/// struct.
///
/// # Enabling
///
/// The feature is enabled by the **presence** of the `[message_broker]`
/// section. To disable a section that is otherwise present, set
/// `enabled = false`. Do **not** write `enabled = true`: it is redundant, and
/// because this struct uses `#[serde(deny_unknown_fields)]` the loader would
/// reject `enabled` as an unknown field. (`enabled = false` is safe — the
/// loader interprets it and short-circuits before deserialization.)
///
/// # Examples
///
/// NATS:
/// ```toml
/// [message_broker]
/// backend = "nats"
/// url     = "nats://nats.internal:4222"
/// ```
///
/// Kafka:
/// ```toml
/// [message_broker]
/// backend  = "kafka"
/// url      = "kafka-broker-1:9092,kafka-broker-2:9092"
/// group_id = "my-service"
/// ```
///
/// Postgres (pgmq):
/// ```toml
/// [message_broker]
/// backend    = "postgres"
/// url        = "postgres://user:pass@localhost/app"
/// queue_name = "edge_events"
/// ```
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessageBrokerConfig {
    /// Which backend to construct.
    pub backend: BackendKind,

    /// Server URL for network backends.
    ///
    /// - `nats`: NATS server URL (e.g. `"nats://host:4222"`). Required.
    /// - `kafka`: Comma-separated bootstrap brokers (e.g. `"broker1:9092,broker2:9092"`). Required.
    /// - `postgres`: Postgres DSN (e.g. `"postgres://user:pass@host/db"`). Required.
    /// - `in_memory`: Must be absent.
    #[serde(default)]
    pub url: Option<String>,

    /// Consumer group identifier. Required when `backend = "kafka"`; must be absent otherwise.
    #[serde(default)]
    pub group_id: Option<String>,

    /// `pgmq` queue name. Required when `backend = "postgres"`; must be absent otherwise.
    #[serde(default)]
    pub queue_name: Option<String>,
}
