//! HeyForm Rust SDK
//!
//! A Rust client library for interacting with the HeyForm GraphQL API.
//! This SDK provides easy-to-use methods for:
//! - User authentication (signup/login)
//! - Workspace (team) management
//! - Form creation and publishing
//! - Custom CSS configuration

pub mod client;
pub mod error;
pub mod types;
pub mod queries;

pub use client::HeyFormClient;
pub use error::{HeyFormError, Result};
pub use types::*;