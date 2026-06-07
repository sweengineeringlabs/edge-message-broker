# Architecture — edge-message-broker

Contract crate — defines `MessageBroker` trait, `Message` value type, and `BrokerSvc::noop_broker`. Real backends (in-memory, NATS, Kafka) live in `swe-edge-runtime-message-broker`.

---

## Sequence

> A publisher sends a `Message` to a topic; a subscriber receives it via a `MessageStream`; the broker contract is backend-agnostic.

```mermaid
sequenceDiagram
    participant Publisher
    participant MessageBroker
    participant Backend
    participant Subscriber

    Publisher->>MessageBroker: publish(topic, Message)
    MessageBroker->>Backend: route to topic
    Backend-->>MessageBroker: ack
    MessageBroker-->>Publisher: Result<(), BrokerError>

    Subscriber->>MessageBroker: subscribe(topic)
    MessageBroker->>Backend: open subscription
    Backend-->>MessageBroker: MessageStream
    MessageBroker-->>Subscriber: MessageStream

    loop continuous
        Backend->>Subscriber: Message (via stream)
    end
```

## Data Flow

> A `Message` enters via `publish`; `subscribe` produces a `MessageStream` of matching messages routed by topic.

```mermaid
flowchart LR
    A["Message\n───────────\ntopic: String\npayload: Bytes\nheaders: HashMap\nmessage_id: Uuid"] --> B["MessageBroker::publish\n(topic, message)"]

    B --> C["Backend\n(noop / in-memory\n/ NATS / Kafka)"]

    D["topic: &str"] --> E["MessageBroker::subscribe\n(topic)"]
    E --> C
    C --> F["MessageStream\n(Stream<Item=Message>)"]
    F --> G["Subscriber fn\n|msg| async { … }"]

    B -->|Ok| H["()  — published"]
    B -->|Err| I["BrokerError\n::NotConnected\n::Publish(reason)\n::Subscribe(reason)"]
```
