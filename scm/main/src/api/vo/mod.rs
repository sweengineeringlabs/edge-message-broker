//! API value objects — plain data, no impl details.

mod backend_kind;
#[allow(clippy::module_inception)]
mod message;

pub use backend_kind::BackendKind;
pub use message::Message;
