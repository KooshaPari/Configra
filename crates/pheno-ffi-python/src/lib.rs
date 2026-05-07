//! Python bindings (PyO3) for Phenotype configuration.
//!
//! # Architecture
//!
//! This crate follows Hexagonal Architecture principles:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                 ADAPTERS (Outer)                       │
//! │   PyO3 Python Bindings (Inbound Adapter)                │
//! └───────────────────────────┬─────────────────────────────┘
//!                             │ depends on
//!                             ▼
//! ┌─────────────────────────────────────────────────────────┐
//! │                 APPLICATION (Middle)                    │
//! │         Use Cases, Command/Query Handlers               │
//! └───────────────────────────┬─────────────────────────────┘
//!                             │ depends on
//!                             ▼
//! ┌─────────────────────────────────────────────────────────┐
//! │                    DOMAIN (Inner)                      │
//! │         DTOs, Entity Converters (Pure Types)           │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! # Modules
//!
//! - `domain`: Pure domain types (DTOs, no PyO3 dependencies)
//! - `application`: Use cases that delegate to pheno-db
//! - `adapters`: PyO3 Python bindings

pub mod domain;
pub mod application;
pub mod adapters;
