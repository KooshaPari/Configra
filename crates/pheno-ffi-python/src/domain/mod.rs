//! Domain layer for Python FFI.
//!
//! This layer contains:
//! - DTOs: Data transfer objects for Python-Rust boundary
//! - Entity converters: Transform between Python and domain types
//!
//! ## Dependency Rule
//! Domain has NO external dependencies (only std).
//! No PyO3, no database libraries.

pub mod dto;
pub mod entities;
