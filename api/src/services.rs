//! Orchestration that sits above the model layer.
//!
//! Anything here needs [`crate::ComhairleState`]: it coordinates the database with
//! one or more services (mailer, bot service, tools, worker). Keeping it out of
//! `models/` is what lets the model layer stay a leaf that depends only on a
//! `PgPool`.

pub mod bot_session;
pub mod conversation;
pub mod permissions;
pub mod polis_moderation;
pub mod resource;
pub mod scheduled_email;
pub mod translations;
pub mod user;
pub mod workflow;
pub mod workflow_step;
