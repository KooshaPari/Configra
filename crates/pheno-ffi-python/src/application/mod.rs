//! Application layer for Python FFI.
//!
//! This layer contains:
//! - Use Cases: Application-specific business rules
//! - DTOs: Data transfer objects
//!
//! ## Dependency Rule
//! Application depends on Domain but NOT on Adapters.

pub mod dto;
pub mod use_cases;
