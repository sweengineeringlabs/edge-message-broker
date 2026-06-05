//! [`BackendKind`] — selects which broker backend a config activates.

/// Which broker backend [`MessageBrokerConfig`] activates.
///
/// Deserialized from the `backend` key of the `[message_broker]` TOML section
/// using snake_case spellings: `"in_memory"`, `"nats"`, and `"kafka"`.
///
/// [`MessageBrokerConfig`]: crate::MessageBrokerConfig
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendKind {
    /// In-process `tokio::sync::broadcast` broker. Requires the `tokio-rt` feature.
    InMemory,
    /// NATS server connection. Requires the `nats` feature and a `url`.
    Nats,
    /// Apache Kafka connection. Requires the `kafka` feature, a `url` (bootstrap brokers
    /// e.g. `"localhost:9092"`), and a `group_id`.
    Kafka,
}
