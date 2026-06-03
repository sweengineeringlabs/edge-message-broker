//! Integration tests for the backend-owned `[message_broker]` section (ADR-006).
//!
//! Exercises `MessageBrokerConfig` as an `OptionalSection`: presence-based
//! enabling, the `enabled = false` disable toggle, `deny_unknown_fields`
//! strictness, cross-field validation, and the `BrokerSvc::from_config`
//! factory wiring.

use swe_edge_configbuilder::{ConfigError, ConfigLoaderFactory, OptionalSection};
use swe_edge_message_broker::{BackendKind, BrokerSvc, MessageBrokerConfig};
use tempfile::TempDir;

/// Write `content` to `application.toml` in a fresh temp dir and return a loader
/// rooted at that dir, along with the dir guard (kept alive by the caller).
fn loader_with(content: &str) -> (TempDir, swe_edge_configbuilder::SectionLoaderImpl) {
    let dir = TempDir::new().expect("create temp dir");
    std::fs::write(dir.path().join("application.toml"), content).expect("write application.toml");
    let loader = ConfigLoaderFactory::create_loader_for_dir(dir.path());
    (dir, loader)
}

// ── canonical contract ──────────────────────────────────────────────────────

/// @covers: section_name — the canonical TOML key is owned by this crate.
#[test]
fn test_section_name_is_message_broker() {
    assert_eq!(MessageBrokerConfig::section_name(), "message_broker");
}

/// @covers: metadata — backend annotates the feature for startup summaries.
#[test]
fn test_metadata_describes_feature_and_owner() {
    let meta = MessageBrokerConfig::metadata();
    assert!(
        !meta.description.is_empty(),
        "metadata must carry a human-readable description"
    );
    assert_eq!(meta.owner, "platform-team");
    assert_eq!(meta.deprecated_since, None);
}

// ── presence-based enabling ───────────────────────────────────────────────────

/// @covers: load_optional — an absent section resolves to Disabled, not an error.
#[test]
fn test_load_absent_section_returns_disabled() {
    let (_dir, loader) = loader_with("[unrelated]\nkey = \"value\"");
    let state =
        MessageBrokerConfig::load_optional(&loader).expect("absent section is not an error");
    assert!(state.is_disabled());
}

/// @covers: load_optional — presence of the section enables it; fields parse.
#[test]
fn test_load_in_memory_present_returns_enabled() {
    let (_dir, loader) = loader_with("[message_broker]\nbackend = \"in_memory\"");
    let state = MessageBrokerConfig::load_optional(&loader).expect("valid section loads");
    let cfg = state.into_option().expect("section present => Enabled");
    assert_eq!(cfg.backend, BackendKind::InMemory);
    assert_eq!(cfg.url, None);
}

/// @covers: load_optional — nats backend with a url parses and validates.
#[test]
fn test_load_nats_with_url_returns_enabled() {
    let (_dir, loader) =
        loader_with("[message_broker]\nbackend = \"nats\"\nurl = \"nats://nats.internal:4222\"");
    let state = MessageBrokerConfig::load_optional(&loader).expect("valid nats section loads");
    let cfg = state.into_option().expect("section present => Enabled");
    assert_eq!(cfg.backend, BackendKind::Nats);
    assert_eq!(cfg.url.as_deref(), Some("nats://nats.internal:4222"));
}

// ── disable toggle ────────────────────────────────────────────────────────────

/// @covers: enabled = false — disables a section that is otherwise present,
/// and is accepted despite `deny_unknown_fields` (loader short-circuits first).
#[test]
fn test_enabled_false_disables_present_section() {
    let (_dir, loader) = loader_with("[message_broker]\nbackend = \"in_memory\"\nenabled = false");
    let state = MessageBrokerConfig::load_optional(&loader).expect("enabled=false is not an error");
    assert!(
        state.is_disabled(),
        "enabled = false must disable a present section"
    );
}

// ── deny_unknown_fields strictness ────────────────────────────────────────────

/// @covers: deny_unknown_fields — `enabled = true` is rejected as an unknown
/// field. This is the ADR-006 gotcha: presence enables, so `enabled = true` is
/// both redundant and (under deny_unknown_fields) a parse error.
#[test]
fn test_enabled_true_is_rejected_by_deny_unknown_fields() {
    let (_dir, loader) = loader_with("[message_broker]\nbackend = \"in_memory\"\nenabled = true");
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("enabled = true must be rejected by deny_unknown_fields");
    assert!(
        matches!(err, ConfigError::Parse(_)),
        "expected a Parse error for the unknown `enabled` key, got {err:?}"
    );
}

/// @covers: deny_unknown_fields — an arbitrary unknown key is rejected.
#[test]
fn test_unknown_field_is_rejected() {
    let (_dir, loader) = loader_with("[message_broker]\nbackend = \"in_memory\"\nbogus = 1");
    let err =
        MessageBrokerConfig::load_optional(&loader).expect_err("unknown field must be rejected");
    assert!(
        matches!(err, ConfigError::Parse(_)),
        "expected a Parse error for the unknown `bogus` key, got {err:?}"
    );
}

// ── cross-field validation ────────────────────────────────────────────────────

/// @covers: validate_enabled — nats without a url is a validation error.
#[test]
fn test_nats_without_url_returns_validation_error() {
    let (_dir, loader) = loader_with("[message_broker]\nbackend = \"nats\"");
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("nats without url must fail validation");
    assert!(
        matches!(err, ConfigError::Validation { .. }),
        "expected Validation error, got {err:?}"
    );
    let msg = err.to_string();
    assert!(
        msg.contains("url"),
        "error must name the offending field: {msg}"
    );
    assert!(
        msg.contains("message_broker"),
        "error must name the section: {msg}"
    );
}

/// @covers: validate_enabled — nats with an empty url is rejected (not just None).
#[test]
fn test_nats_with_blank_url_returns_validation_error() {
    let (_dir, loader) = loader_with("[message_broker]\nbackend = \"nats\"\nurl = \"   \"");
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("nats with blank url must fail validation");
    assert!(matches!(err, ConfigError::Validation { .. }), "got {err:?}");
}

/// @covers: validate_enabled — in_memory with a url is a misconfiguration.
#[test]
fn test_in_memory_with_url_returns_validation_error() {
    let (_dir, loader) =
        loader_with("[message_broker]\nbackend = \"in_memory\"\nurl = \"nats://x:4222\"");
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("in_memory with url must fail validation");
    assert!(matches!(err, ConfigError::Validation { .. }), "got {err:?}");
}

/// @covers: backend deserialization — an unknown backend value is rejected.
#[test]
fn test_unknown_backend_value_is_rejected() {
    let (_dir, loader) = loader_with("[message_broker]\nbackend = \"kafka\"");
    let err = MessageBrokerConfig::load_optional(&loader)
        .expect_err("unknown backend variant must fail to parse");
    assert!(matches!(err, ConfigError::Parse(_)), "got {err:?}");
}

// ── factory wiring (feature-dependent) ────────────────────────────────────────

/// @covers: from_config — with the `tokio-rt` feature, an in_memory config
/// produces a real, usable broker (round-trips a published message).
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_from_config_in_memory_builds_usable_broker() {
    use futures::StreamExt as _;
    use swe_edge_message_broker::Message;

    let cfg = MessageBrokerConfig {
        backend: BackendKind::InMemory,
        url: None,
    };
    let broker = BrokerSvc::from_config(&cfg)
        .await
        .expect("in_memory broker builds with tokio-rt");

    // Prove it is a real wired broker: subscribe, publish, receive.
    let mut sub = broker.subscribe("topic").await.expect("subscribe");
    broker
        .publish("topic", Message::new("hello"))
        .await
        .expect("publish");
    let received = sub
        .next()
        .await
        .expect("a message is delivered")
        .expect("delivery is not an error");
    assert_eq!(&received.payload[..], b"hello");
}

/// @covers: from_config — without the `tokio-rt` feature, requesting the
/// in_memory backend reports the missing feature instead of silently failing.
#[cfg(not(feature = "tokio-rt"))]
#[tokio::test]
async fn test_from_config_in_memory_without_feature_returns_unavailable() {
    use swe_edge_message_broker::BrokerError;

    let cfg = MessageBrokerConfig {
        backend: BackendKind::InMemory,
        url: None,
    };
    // `Box<dyn MessageBroker>` is not `Debug`, so match instead of `expect_err`.
    let result = BrokerSvc::from_config(&cfg).await;
    assert!(
        matches!(result, Err(BrokerError::Unavailable(_))),
        "expected Unavailable error when the tokio-rt feature is absent"
    );
}
