//! Attested Workload — run code in a cloud TEE, prove it, serve it over attested TLS.
//!
//! Library surface for tests and embedders. The CLI lives in `main.rs` as `aw`.

pub mod eat;
pub mod net;
pub mod quote;
pub mod registry;
pub mod tee;
pub mod value_x;
