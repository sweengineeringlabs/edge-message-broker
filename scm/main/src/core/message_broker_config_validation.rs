//! [`MessageBrokerConfig`]'s [`OptionalSection`] (configbuilder auto-configuration,
//! ADR-006) and [`Validator`] (this crate's own SEA contract) implementations.
//! Both delegate to the same cross-field validation rules. Kept in `core/`,
//! not `api/`: `api/` is a pure declaration layer (`api_no_inherent_impl`).

use configbuilder::{ConfigError, FeatureMetadata, OptionalSection};

use crate::api::{BackendKind, MessageBrokerConfig, ValidationError, ValidationRequest, Validator};

impl MessageBrokerConfig {
    fn validate_fields(&self) -> Result<(), String> {
        match self.backend {
            BackendKind::Nats => {
                let url_set = self.url.as_deref().is_some_and(|u| !u.trim().is_empty());
                if !url_set {
                    return Err("backend = \"nats\" requires a non-empty `url` \
                         (e.g. url = \"nats://host:4222\")"
                        .to_string());
                }
                if self.group_id.is_some() {
                    return Err(
                        "backend = \"nats\" does not accept a `group_id`; remove it".to_string()
                    );
                }
                if self.queue_name.is_some() {
                    return Err(
                        "backend = \"nats\" does not accept a `queue_name`; remove it".to_string(),
                    );
                }
            }
            BackendKind::InMemory => {
                if self.url.is_some() {
                    return Err("backend = \"in_memory\" does not accept a `url`; \
                         remove it or set backend = \"nats\", \"kafka\", or \"postgres\""
                        .to_string());
                }
                if self.group_id.is_some() {
                    return Err(
                        "backend = \"in_memory\" does not accept a `group_id`; remove it"
                            .to_string(),
                    );
                }
                if self.queue_name.is_some() {
                    return Err(
                        "backend = \"in_memory\" does not accept a `queue_name`; remove it"
                            .to_string(),
                    );
                }
            }
            BackendKind::Kafka => {
                let url_set = self.url.as_deref().is_some_and(|u| !u.trim().is_empty());
                if !url_set {
                    return Err("backend = \"kafka\" requires a non-empty `url` \
                         (bootstrap brokers, e.g. url = \"broker1:9092,broker2:9092\")"
                        .to_string());
                }
                let group_set = self
                    .group_id
                    .as_deref()
                    .is_some_and(|g| !g.trim().is_empty());
                if !group_set {
                    return Err("backend = \"kafka\" requires a non-empty `group_id`".to_string());
                }
                if self.queue_name.is_some() {
                    return Err(
                        "backend = \"kafka\" does not accept a `queue_name`; remove it".to_string(),
                    );
                }
            }
            BackendKind::Postgres => {
                let url_set = self.url.as_deref().is_some_and(|u| !u.trim().is_empty());
                if !url_set {
                    return Err("backend = \"postgres\" requires a non-empty `url` \
                         (Postgres DSN, e.g. url = \"postgres://user:pass@host/db\")"
                        .to_string());
                }
                let queue_set = self
                    .queue_name
                    .as_deref()
                    .is_some_and(|q| !q.trim().is_empty());
                if !queue_set {
                    return Err(
                        "backend = \"postgres\" requires a non-empty `queue_name`".to_string()
                    );
                }
                if self.group_id.is_some() {
                    return Err(
                        "backend = \"postgres\" does not accept a `group_id`; remove it"
                            .to_string(),
                    );
                }
            }
        }
        Ok(())
    }
}

impl Validator for MessageBrokerConfig {
    fn validate(&self, _request: ValidationRequest) -> Result<(), ValidationError> {
        self.validate_fields().map_err(|reason| ValidationError {
            violations: vec![reason],
        })
    }
}

impl OptionalSection for MessageBrokerConfig {
    // @allow: no_stub_fn_bodies — returns the canonical section key, not a stub
    fn section_name() -> &'static str {
        "message_broker"
    }

    fn validate_enabled(&self) -> Result<(), ConfigError> {
        self.validate_fields()
            .map_err(|reason| ConfigError::Validation {
                section: Self::section_name().to_string(),
                reason,
            })
    }

    fn metadata() -> FeatureMetadata {
        FeatureMetadata {
            description: "cross-process pub/sub message broker",
            owner: "platform-team",
            deprecated_since: None,
        }
    }
}
