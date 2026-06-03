//! [`MessageBrokerConfig`] — backend-owned `[message_broker]` TOML contract.

use swe_edge_configbuilder::{ConfigError, FeatureMetadata, OptionalSection};

use crate::api::types::backend_kind::BackendKind;

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
/// # Example
///
/// ```toml
/// [message_broker]
/// backend = "nats"
/// url     = "nats://nats.internal:4222"
/// ```
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessageBrokerConfig {
    /// Which backend to construct.
    pub backend: BackendKind,

    /// Server URL for network backends. Required when `backend = "nats"`;
    /// must be absent when `backend = "in_memory"`.
    #[serde(default)]
    pub url: Option<String>,
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
            }
            BackendKind::InMemory => {
                if self.url.is_some() {
                    return Err(ConfigError::validation(
                        Self::section_name(),
                        "backend = \"in_memory\" does not accept a `url`; \
                         remove it or set backend = \"nats\"",
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
