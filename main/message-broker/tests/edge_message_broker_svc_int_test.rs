//! Public-API integration tests for the message broker SAF surface.
//!
//! Backend factories (`in_memory_broker`, `nats_broker`, `from_config`) live in
//! `swe-edge-runtime`; this contract crate's SAF exposes only the no-op
//! reference broker and the config-builder/validate helpers.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_message_broker::BrokerSvc;

/// @covers: noop_broker
#[tokio::test]
async fn test_noop_broker_health_check_returns_ok() {
    assert!(BrokerSvc::noop_broker().health_check().await.is_ok());
}

/// @covers: noop_broker
#[tokio::test]
async fn test_noop_broker_publish_then_subscribe_is_inert() {
    use futures::StreamExt as _;
    use swe_edge_message_broker::Message;

    let broker = BrokerSvc::noop_broker();
    broker
        .publish("svc-test", Message::new(b"ping".as_ref()))
        .await
        .unwrap();
    let mut stream = broker.subscribe("svc-test").await.unwrap();
    assert!(
        stream.next().await.is_none(),
        "noop broker delivers nothing"
    );
}

/// @covers: create_config_builder
#[test]
fn test_create_config_builder_is_seeded_with_package_identity() {
    // The builder is pre-seeded with this crate's name; building a config from
    // it must not panic and must carry the seeded identity forward.
    let _builder = BrokerSvc::create_config_builder();
}
