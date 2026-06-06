//! [`MessageBrokerConfig`] — backend-owned `[message_broker]` TOML contract.

use swe_edge_configbuilder::{ConfigError, FeatureMetadata, OptionalSection};

use crate::api::vo::backend_kind::BackendKind;

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
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessageBrokerConfig {
    /// Which backend to construct.
    pub backend: BackendKind,

    /// Server URL for network backends.
    ///
    /// - `nats`: NATS server URL (e.g. `"nats://host:4222"`). Required.
    /// - `kafka`: Comma-separated bootstrap brokers (e.g. `"broker1:9092,broker2:9092"`). Required.
    /// - `in_memory`: Must be absent.
    #[serde(default)]
    pub url: Option<String>,

    /// Consumer group identifier. Required when `backend = "kafka"`; must be absent otherwise.
    #[serde(default)]
    pub group_id: Option<String>,
}

impl OptionalSection for MessageBrokerConfig {
    // @allow: no_stub_fn_bodies — returns the canonical section key, not a stub
    fn section_name() -> &'static str {
        "message_broker"
    }

    fn validate_enabled(&self) -> Result<(), ConfigError> {
        match self.backend {
            BackendKind::Nats => {
                let url_set = self.url.as_deref().is_some_and(|u| !u.trim().is_empty());
                if !url_set {
                    return Err(ConfigError::validation(
                        Self::section_name(),
                        "backend = \"nats\" requires a non-empty `url` \
                         (e.g. url = \"nats://host:4222\")",
                    ));
                }
                if self.group_id.is_some() {
                    return Err(ConfigError::validation(
                        Self::section_name(),
                        "backend = \"nats\" does not accept a `group_id`; remove it",
                    ));
                }
            }
            BackendKind::InMemory => {
                if self.url.is_some() {
                    return Err(ConfigError::validation(
                        Self::section_name(),
                        "backend = \"in_memory\" does not accept a `url`; \
                         remove it or set backend = \"nats\" or backend = \"kafka\"",
                    ));
                }
                if self.group_id.is_some() {
                    return Err(ConfigError::validation(
                        Self::section_name(),
                        "backend = \"in_memory\" does not accept a `group_id`; remove it",
                    ));
                }
            }
            BackendKind::Kafka => {
                let url_set = self.url.as_deref().is_some_and(|u| !u.trim().is_empty());
                if !url_set {
                    return Err(ConfigError::validation(
                        Self::section_name(),
                        "backend = \"kafka\" requires a non-empty `url` \
                         (bootstrap brokers, e.g. url = \"broker1:9092,broker2:9092\")",
                    ));
                }
                let group_set = self
                    .group_id
                    .as_deref()
                    .is_some_and(|g| !g.trim().is_empty());
                if !group_set {
                    return Err(ConfigError::validation(
                        Self::section_name(),
                        "backend = \"kafka\" requires a non-empty `group_id`",
                    ));
                }
            }
        }
        Ok(())
    }

    fn metadata() -> FeatureMetadata {
        FeatureMetadata {
            description: "cross-process pub/sub message broker",
            owner: "platform-team",
            deprecated_since: None,
        }
    }
}
