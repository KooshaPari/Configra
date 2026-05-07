//! Adapters layer for Python FFI.
//!
//! This layer contains:
//! - Inbound Adapters: PyO3 Python bindings
//!
//! ## Dependency Rule
//! Adapters depend on Application and Domain but NOT vice versa.

pub mod inbound;
