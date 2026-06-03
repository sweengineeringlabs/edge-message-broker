//! [`BackendKind`] — selects which broker backend a config activates.

/// Which broker backend [`MessageBrokerConfig`] activates.
///
/// Deserialized from the `backend` key of the `[message_broker]` TOML section
/// using snake_case spellings: `"in_memory"` and `"nats"`.
///
/// [`MessageBrokerConfig`]: crate::MessageBrokerConfig
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendKind {
    /// In-process `tokio::sync::broadcast` broker. Requires the `tokio-rt` feature.
    InMemory,
    /// NATS server connection. Requires the `nats` feature and a `url`.
    Nats,
}
