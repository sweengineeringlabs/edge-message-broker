#[cfg(feature = "tokio-rt")]
pub(crate) mod r#in;
#[cfg(feature = "nats")]
pub(crate) mod nats;

#[cfg(feature = "tokio-rt")]
pub(crate) use self::r#in::in_memory_message_broker::InMemoryMessageBroker;
#[cfg(feature = "nats")]
pub(crate) use nats::nats_message_broker::NatsMessageBroker;
