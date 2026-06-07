# swe-edge-message-broker

## WHAT

Runtime-agnostic pub/sub message broker for swe-edge services — pluggable backends, config-driven
backend selection, and a provider-neutral contract for publish/subscribe operations.

Key capabilities:

- **`MessageBroker`** — core trait: `publish(topic, msg)`, `subscribe(topic)`, `health_check()`; object-safe; pluggable backend (in-memory, NATS, Kafka)
- **`Message`** — value type carrying raw `Bytes` payload and optional `HashMap<String, String>` headers; serializable
- **`MessageStream`** — async stream of `Message`; return type of `subscribe()`
- **`BrokerError`** — enum for broker-level failures (connection, timeout, backend-specific)
- **`MessageBrokerConfig`** — config VO from `[message_broker]` TOML; selects backend and supplies connection URLs
- **`BrokerSvc`** — SAF factory: `from_config()` → `Arc<dyn MessageBroker>`; callers never name the concrete type

## WHY

| Problem | Solution |
|---------|----------|
| Application code tied to a specific message transport (NATS, Kafka, in-process) | `MessageBroker` trait decouples publish/subscribe from transport; swapping backends requires only a config change |
| In-process channel replaced by NATS in prod requires handler rewrites | Same `publish`/`subscribe` API across all backends; handler code unchanged |
| Backend selection hardcoded at compile time | `from_config()` reads `[message_broker.backend]` from TOML and constructs the correct backend at startup |
| Diamond dep conflicts when broker types change | One crate, one tag — all consumers pin the same version; kgraph detects conflicts pre-commit |
