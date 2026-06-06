//! Coverage for the no-op reference broker (`core::broker::noop_message_broker`),
//! exercised through the public `BrokerSvc::noop_broker()` factory.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use futures::StreamExt;
use swe_edge_message_broker::{BrokerSvc, Message};

/// @covers: noop_message_broker — publishing always succeeds (fire-and-forget).
#[tokio::test]
async fn test_noop_publish_returns_ok() {
    let broker = BrokerSvc::noop_broker();
    let result = broker
        .publish("any.topic", Message::new(b"payload".as_ref()))
        .await;
    assert!(result.is_ok(), "noop publish must succeed");
}

/// @covers: noop_message_broker — subscribing yields an immediately-empty stream.
#[tokio::test]
async fn test_noop_subscribe_yields_empty_stream() {
    let broker = BrokerSvc::noop_broker();
    let mut stream = broker.subscribe("any.topic").await.unwrap();
    assert!(
        stream.next().await.is_none(),
        "noop subscribe must yield no messages"
    );
}

/// @covers: noop_message_broker — health_check always reports healthy.
#[tokio::test]
async fn test_noop_health_check_returns_ok() {
    let broker = BrokerSvc::noop_broker();
    assert!(broker.health_check().await.is_ok());
}
