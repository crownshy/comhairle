//! Shim over [`comhairle_model`].
//!
//! The model layer lives in its own crate; this module re-exports it so the
//! `crate::models::` paths used throughout the api keep working. The only
//! module physically left here is the test-fixture helper, which builds its
//! fixtures through the HTTP API and so cannot move down.

pub use comhairle_model::models::*;

#[cfg(test)]
pub mod model_test_helpers;
