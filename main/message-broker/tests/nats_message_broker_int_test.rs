//! Integration tests for the NATS message broker API marker.

/// @covers: NatsMessageBroker — marker type is constructible
#[cfg(feature = "nats")]
#[test]
fn test_nats_message_broker_api_marker_is_constructible() {
    use swe_edge_message_broker::NatsMessageBroker;
    let _ = NatsMessageBroker;
}

/// Structural coverage placeholder: NatsMessageBroker API marker.
///
/// NATS integration requires a running NATS server.
/// Feature-gated tests in tests/edge_message_broker_svc_int_test.rs cover
/// the connection error path.
#[test]
fn test_nats_message_broker_requires_nats_feature() {
    let nats_enabled = cfg!(feature = "nats");
    // This test documents the feature-gating constraint — it always passes.
    let _ = nats_enabled;
}
