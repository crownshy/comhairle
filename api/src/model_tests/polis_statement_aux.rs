//! Tests for [`crate::models::polis_statement_aux`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use std::error::Error;

    use serde_json::json;
    use sqlx::PgPool;

    use crate::{
        models::model_test_helpers::setup_default_app_and_session,
        test_helpers::{extract, polis_tool_config},
    };

    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, DataError, EventError, InviteError, ModelError,
        PermissionError, ReportError, UserError, ValidationError, WorkflowError,
    };
    #[allow(unused_imports)]
    use crate::models::moderation_status::ModerationStatus;
    #[allow(unused_imports)]
    use crate::models::pagination::{Order, PageOptions, PaginatedResults};
    use crate::models::polis_statement_aux::*;
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
    #[allow(unused_imports)]
    use uuid::Uuid;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn upsert_from_polis_refreshes_moderation_status(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation_id)
            .await?;
        let workflow_id: String = extract("id", &workflow);

        let (_, workflow_step, _) = session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                    "name": "Polis step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "polis step",
                    "is_offline": false,
                    "required": true,
                    "tool_setup": polis_tool_config(),
                })
                .to_string()
                .into(),
            )
            .await?;
        let workflow_step_id: String = extract("id", &workflow_step);
        let workflow_step_id = Uuid::parse_str(&workflow_step_id)?;

        let record = UpsertFromPolis {
            workflow_step_id,
            user_id: None,
            zid: 7,
            polis_conversation_id: "test-poll".into(),
            polis_statement_id: 42,
            statement_text: "first text".into(),
            is_seed: false,
            moderation_status: ModerationStatus::Pending,
        };

        let initial = upsert_from_polis(&pool, &record).await?;
        assert_eq!(initial.moderation_status, ModerationStatus::Pending);

        let accepted = upsert_from_polis(
            &pool,
            &UpsertFromPolis {
                statement_text: "updated text".into(),
                moderation_status: ModerationStatus::Accepted,
                ..record
            },
        )
        .await?;

        assert_eq!(
            accepted.id, initial.id,
            "should have hit ON CONFLICT, not inserted a new row"
        );
        assert_eq!(
            accepted.moderation_status,
            ModerationStatus::Accepted,
            "moderation_status should refresh from polis on re-sync"
        );
        assert_eq!(accepted.statement_text, "updated text");
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn record_split_rejects_original_and_links_derived(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;

        let (_, conversation, _) = session.create_random_conversation(&app).await?;
        let conversation_id: String = extract("id", &conversation);

        let (_, workflow, _) = session
            .create_random_workflow(&app, &conversation_id)
            .await?;
        let workflow_id: String = extract("id", &workflow);

        let (_, workflow_step, _) = session
            .post(
                &app,
                &format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step"),
                json!({
                    "name": "Polis step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "polis step",
                    "is_offline": false,
                    "required": true,
                    "tool_setup": polis_tool_config(),
                })
                .to_string()
                .into(),
            )
            .await?;
        let workflow_step_id = Uuid::parse_str(&extract::<String>("id", &workflow_step))?;

        // A participant composite statement, pending.
        let original = upsert_from_polis(
            &pool,
            &UpsertFromPolis {
                workflow_step_id,
                user_id: None,
                zid: 5,
                polis_conversation_id: "test-poll".into(),
                polis_statement_id: 10,
                statement_text: "cats are great and dogs are great".into(),
                is_seed: false,
                moderation_status: ModerationStatus::Pending,
            },
        )
        .await?;

        let derived = [
            CreateDerivedStatement {
                workflow_step_id,
                zid: 0,
                polis_conversation_id: "test-poll".into(),
                polis_statement_id: 11,
                statement_text: "cats are great".into(),
                original_statement_id: original.id,
            },
            CreateDerivedStatement {
                workflow_step_id,
                zid: 0,
                polis_conversation_id: "test-poll".into(),
                polis_statement_id: 12,
                statement_text: "dogs are great".into(),
                original_statement_id: original.id,
            },
        ];

        let (rejected_original, replacements) =
            record_split(&pool, original.id, &derived, "Reworded/split by moderator").await?;

        assert_eq!(rejected_original.id, original.id);
        assert_eq!(
            rejected_original.moderation_status,
            ModerationStatus::Rejected
        );
        assert_eq!(
            rejected_original.moderation_reason.as_deref(),
            Some("Reworded/split by moderator")
        );

        assert_eq!(replacements.len(), 2);
        for row in &replacements {
            assert!(!row.is_seed, "derived statements are not seeds");
            assert_eq!(
                row.moderation_status,
                ModerationStatus::Accepted,
                "derived statements are auto-accepted"
            );
            assert_eq!(
                row.original_statement_id,
                Some(original.id),
                "derived statements point back at the original"
            );
            assert_eq!(
                row.user_id, None,
                "derived statements are not attributed to a participant"
            );
        }
        Ok(())
    }
}
