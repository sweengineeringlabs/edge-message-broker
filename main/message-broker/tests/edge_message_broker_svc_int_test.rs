//! Public-API integration tests for the message broker SAF surface.

#![allow(clippy::unwrap_used, clippy::expect_used)]

// `BrokerSvc` is exercised by both the tokio-rt and nats tests; trait methods on
// the returned `Box<dyn MessageBroker>` resolve through the trait object, so the
// `MessageBroker` trait itself need not be imported.
#[cfg(any(feature = "tokio-rt", feature = "nats"))]
use swe_edge_message_broker::BrokerSvc;

/// @covers: in_memory_broker
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_in_memory_broker_health_check_returns_ok() {
    assert!(BrokerSvc::in_memory_broker().health_check().await.is_ok());
}

/// @covers: in_memory_broker
#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_in_memory_broker_pub_sub_roundtrip() {
    use bytes::Bytes;
    use futures::StreamExt as _;
    use swe_edge_message_broker::Message;

    let broker = BrokerSvc::in_memory_broker();
    let mut stream = broker.subscribe("svc-test").await.unwrap();
    broker
        .publish("svc-test", Message::new(b"ping".as_ref()))
        .await
        .unwrap();
    let msg = stream.next().await.unwrap().unwrap();
    assert_eq!(msg.payload, Bytes::from_static(b"ping"));
}

/// @covers: nats_broker
#[cfg(feature = "nats")]
#[tokio::test]
async fn test_nats_broker_returns_connection_error_for_unreachable_host() {
    use swe_edge_message_broker::BrokerError;
    let result = BrokerSvc::nats_broker("nats://127.0.0.1:4229").await;
    // Note: don't `{result:?}` — the Ok variant is `Box<dyn MessageBroker>`,
    // which is not `Debug`. A static message keeps the assertion compilable.
    assert!(
        matches!(result, Err(BrokerError::Connection(_))),
        "expected a Connection error from an unreachable NATS host"
    );
}
