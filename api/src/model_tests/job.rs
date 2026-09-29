//! Tests for [`crate::models::job`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, DataError, EventError, InviteError, ModelError,
        PermissionError, ReportError, UserError, ValidationError, WorkflowError,
    };
    use crate::models::job::*;
    #[allow(unused_imports)]
    use crate::models::moderation_status::ModerationStatus;
    #[allow(unused_imports)]
    use crate::models::pagination::{Order, PageOptions, PaginatedResults};
    #[allow(unused_imports)]
    use crate::models::request_context::{ClientIp, ClientUserAgent};
    #[allow(unused_imports)]
    use crate::models::user_progress::ProgressStatus;
    #[allow(unused_imports)]
    use crate::models::users::User;
    #[allow(unused_imports)]
    use crate::models::{proposal_section, user_progress, users};
    #[allow(unused_imports)]
    use ::std::collections::{HashMap, HashSet};
    #[allow(unused_imports)]
    use chrono::{DateTime, Utc};
    #[allow(unused_imports)]
    use schemars::JsonSchema;
    #[allow(unused_imports)]
    use sqlx::PgPool;
    #[allow(unused_imports)]
    use uuid::Uuid;

    use std::error::Error;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_job(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let job = create(
            &pool,
            CreateJob {
                ..Default::default()
            },
        )
        .await?;

        assert_eq!(
            job.status,
            Some("pending".to_string()),
            "incorrect default status"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_mark_job_as_completed(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let job = create(
            &pool,
            CreateJob {
                ..Default::default()
            },
        )
        .await?;

        assert_eq!(
            job.status,
            Some("pending".to_string()),
            "incorrect status before update"
        );
        assert!(
            job.finished_at.is_none(),
            "incorrect finished_at before update"
        );
        assert!(
            job.completion_message.is_none(),
            "incorrect completion_message before update"
        );

        let job = complete(&pool, job.id, "Completed successfully in a test").await?;

        assert_eq!(
            job.status,
            Some("completed".to_string()),
            "incorrect status after update"
        );
        assert!(
            job.finished_at.is_some(),
            "incorrect finished_at after update"
        );
        assert_eq!(
            job.completion_message,
            Some("Completed successfully in a test".to_string()),
            "incorrect completion_message after update"
        );

        Ok(())
    }
}
