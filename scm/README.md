# swe-edge-message-broker

> **TLDR:** Runtime-agnostic pub/sub broker for swe-edge — in-memory, NATS, and Kafka backends behind one `MessageBroker` trait; config-driven backend selection. See [Overview](docs/README.md) for details.

Runtime-agnostic cross-process pub/sub broker for `swe-edge` services. Ships an
in-memory tokio broadcast backend (`tokio-rt`), a NATS backend (`nats`), and
a Kafka backend (`kafka`). Bring your own backend by implementing `MessageBroker`.

## Quick Start

This crate owns its `[message_broker]` TOML contract (ADR-006): the backend
defines the section name, field shape, and validation rules. Consumers opt in
by adding the crate to `Cargo.toml` and a `[message_broker]` section to their
`application.toml` — they never redefine the config struct.

```toml
# In-memory broker (requires the `tokio-rt` feature)
[message_broker]
backend = "in_memory"

# NATS broker (requires the `nats` feature)
[message_broker]
backend = "nats"
url     = "nats://nats.internal:4222"
```

**Enabling:** the feature is enabled by the **presence** of the section. To
disable a section that is otherwise present, set `enabled = false`. Do **not**
write `enabled = true` — it is redundant, and because `MessageBrokerConfig` uses
`#[serde(deny_unknown_fields)]` the loader rejects `enabled` as an unknown field.

**Fields:**

| Key       | Type     | Required            | Notes                                          |
|-----------|----------|---------------------|------------------------------------------------|
| `backend` | string   | yes                 | `"in_memory"` or `"nats"`.                     |
| `url`     | string   | nats only           | NATS server URL; must be absent for in_memory. |

### Loading at startup

```rust
use swe_edge_configbuilder::{FeatureRegistry, FeatureState};
use swe_edge_message_broker::{BrokerSvc, MessageBrokerConfig};

let mut registry = FeatureRegistry::new();
let state = registry.load::<MessageBrokerConfig>(&loader)?;
registry.validate_dependencies()?;

if let FeatureState::Enabled(cfg) = state {
    let broker = BrokerSvc::from_config(&cfg).await?;
    // ... wire `broker` into the runtime
}
```

## Documentation

| Document | Description |
|----------|-------------|
| [Overview](docs/README.md) | WHAT + WHY — capabilities and design rationale |
