//! The Comhairle data model: rows, query builders, domain errors, and the
//! database migrations.
//!
//! This crate is deliberately thin. It knows about Postgres (sqlx + sea-query)
//! and the domain vocabulary, and nothing else: no HTTP, no application state,
//! no redis. Errors report an abstract `ErrorKind`
//! ([`models::error::DomainError`]); mapping kinds onto status codes is the api
//! crate's job.
//!
//! The module tree lives under [`models`] rather than at the crate root so that
//! macro-generated code (the `Translatable` derives emit `crate::models::...`
//! paths) resolves identically whether it expands here or in `comhairle_api`,
//! which re-exports this module as `crate::models`.

pub mod models;

/// The embedded database migrations (`./migrations`). The api crate re-exports
/// this as `crate::SQLX_MIGRATOR` for its `#[sqlx::test]` attributes.
pub static SQLX_MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();
