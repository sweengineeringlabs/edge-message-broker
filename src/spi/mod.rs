//! SPI — extension hooks for downstream consumers.
//!
//! The presence of `spi/` signals that `saf/` intentionally returns
//! `impl Trait` — consumers may substitute their own broker implementations.

pub(crate) mod broker_backend;
