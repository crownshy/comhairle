//! Test suites for the model layer. They live here rather than next to the
//! models because most of them boot the full HTTP app and build their fixtures
//! through the API, which the model crate cannot depend on.

mod api_key;
mod audio_recording;
mod chat_instructions;
mod conversation;
mod demographics;
mod email_template_config;
mod event;
mod event_attendance;
mod invites;
mod job;
mod media;
mod moderation_policy;
mod organization;
mod otp;
mod polis_statement_aux;
mod proposal;
mod proposal_response;
mod recruitment_target;
mod refresh_token;
mod region;
mod report;
mod scheduled_email;
mod thinking_space_answer;
mod thinking_space_follow_up_question;
mod thinking_space_summary;
mod translations;
mod user_profile;
mod user_progress;
mod users;
mod workflow;
mod workflow_step;
