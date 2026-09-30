//! Tests for [`crate::models::moderation_policy`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, DataError, EventError, InviteError, ModelError,
        PermissionError, ReportError, UserError, ValidationError, WorkflowError,
    };
    use crate::models::moderation_policy::*;
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

    use crate::models::model_test_helpers::{
        get_random_conversation_id, setup_default_app_and_session,
    };

    fn new_reason(label: &str) -> (Option<Uuid>, &str, Option<&str>) {
        (None, label, None)
    }

    #[test]
    fn should_trim_labels_and_drop_blank_descriptions() -> Result<(), Box<dyn Error>> {
        let cleaned = clean_reasons([(None, "  Duplicate ", Some("   "))])?;

        assert_eq!(cleaned[0].label, "Duplicate", "label not trimmed");
        assert_eq!(cleaned[0].description, None, "blank description kept");

        Ok(())
    }

    #[test]
    fn should_reject_blank_label() {
        let result = clean_reasons([new_reason("   ")]);

        assert!(matches!(
            result,
            Err(ModelError::Validation(ValidationError::BadRequest(_)))
        ));
    }

    #[test]
    fn should_reject_label_containing_the_separator() {
        let result = clean_reasons([new_reason("Off-topic: unclear")]);

        assert!(matches!(
            result,
            Err(ModelError::Validation(ValidationError::BadRequest(_)))
        ));
    }

    #[test]
    fn should_reject_labels_repeated_ignoring_case() {
        let result = clean_reasons([new_reason("Duplicate"), new_reason(" duplicate")]);

        assert!(matches!(
            result,
            Err(ModelError::Validation(ValidationError::BadRequest(_)))
        ));
    }

    #[test]
    fn should_reject_repeated_reason_ids() {
        let id = Some(Uuid::new_v4());
        let result = clean_reasons([(id, "Duplicate", None), (id, "Harmful", None)]);

        assert!(matches!(
            result,
            Err(ModelError::Validation(ValidationError::BadRequest(_)))
        ));
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_policy_with_default_reasons(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let created = create(
            &pool,
            conversation_id,
            &CreateModerationPolicy {
                name: "Default".into(),
                reasons: None,
            },
        )
        .await?;

        let labels: Vec<&str> = created
            .reasons
            .iter()
            .map(|reason| reason.label.as_str())
            .collect();
        let default_labels: Vec<&str> = DEFAULT_REASONS.iter().map(|(label, _)| *label).collect();
        assert_eq!(labels, default_labels, "reasons don't match the defaults");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_reasons_keeping_ids(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let created = create(
            &pool,
            conversation_id,
            &CreateModerationPolicy {
                name: "Policy".into(),
                reasons: Some(vec![
                    NewModerationPolicyReason {
                        label: "Spam".into(),
                        description: None,
                    },
                    NewModerationPolicyReason {
                        label: "Rude".into(),
                        description: None,
                    },
                ]),
            },
        )
        .await?;
        let rude = &created.reasons[1];

        let updated = update(
            &pool,
            conversation_id,
            created.policy.id,
            &UpdateModerationPolicy {
                name: "Renamed".into(),
                reasons: vec![
                    UpdateModerationPolicyReason {
                        id: Some(rude.id),
                        label: "Abusive".into(),
                        description: Some("Insults or threats".into()),
                    },
                    UpdateModerationPolicyReason {
                        id: None,
                        label: "Off-topic".into(),
                        description: None,
                    },
                ],
            },
        )
        .await?;

        assert_eq!(updated.policy.name, "Renamed", "name not updated");
        assert_eq!(updated.reasons.len(), 2, "dropped reason not deleted");
        assert_eq!(updated.reasons[0].id, rude.id, "kept reason got a new id");
        assert_eq!(updated.reasons[0].label, "Abusive", "label not updated");
        assert_eq!(updated.reasons[0].position, 0, "kept reason not reordered");
        assert_eq!(updated.reasons[1].label, "Off-topic", "new reason missing");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_swap_two_reasons_in_one_save(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;

        let created = create(
            &pool,
            conversation_id,
            &CreateModerationPolicy {
                name: "Policy".into(),
                reasons: Some(vec![
                    NewModerationPolicyReason {
                        label: "Spam".into(),
                        description: None,
                    },
                    NewModerationPolicyReason {
                        label: "Rude".into(),
                        description: None,
                    },
                ]),
            },
        )
        .await?;
        let (spam, rude) = (&created.reasons[0], &created.reasons[1]);

        // Rude is written first with Spam's label and position, so both constraints would
        // fail mid-save if they weren't deferred to commit.
        let updated = update(
            &pool,
            conversation_id,
            created.policy.id,
            &UpdateModerationPolicy {
                name: "Policy".into(),
                reasons: vec![
                    UpdateModerationPolicyReason {
                        id: Some(rude.id),
                        label: "Spam".into(),
                        description: None,
                    },
                    UpdateModerationPolicyReason {
                        id: Some(spam.id),
                        label: "Rude".into(),
                        description: None,
                    },
                ],
            },
        )
        .await?;

        assert_eq!(updated.reasons[0].id, rude.id, "reasons not reordered");
        assert_eq!(updated.reasons[0].label, "Spam", "labels not swapped");
        assert_eq!(updated.reasons[1].id, spam.id, "reasons not reordered");
        assert_eq!(updated.reasons[1].label, "Rude", "labels not swapped");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_refuse_repeated_label_or_position_in_database(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let created = create(
            &pool,
            conversation_id,
            &CreateModerationPolicy {
                name: "Policy".into(),
                reasons: None,
            },
        )
        .await?;
        let insert_reason_sql = "INSERT INTO moderation_policy_reason
            (moderation_policy_id, label, position) VALUES ($1, $2, $3)";

        let repeated_label = sqlx::query(insert_reason_sql)
            .bind(created.policy.id)
            .bind(created.reasons[0].label.to_uppercase())
            .bind(99)
            .execute(&pool)
            .await;
        assert!(repeated_label.is_err(), "repeated label saved");

        let repeated_position = sqlx::query(insert_reason_sql)
            .bind(created.policy.id)
            .bind("Something else")
            .bind(0)
            .execute(&pool)
            .await;
        assert!(repeated_position.is_err(), "repeated position saved");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_not_update_reason_from_another_policy(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let conversation_id = get_random_conversation_id(&app, &mut session).await?;
        let default_policy = CreateModerationPolicy {
            name: "Policy".into(),
            reasons: None,
        };
        let first = create(&pool, conversation_id, &default_policy).await?;
        let second = create(&pool, conversation_id, &default_policy).await?;

        let result = update(
            &pool,
            conversation_id,
            first.policy.id,
            &UpdateModerationPolicy {
                name: "Policy".into(),
                reasons: vec![UpdateModerationPolicyReason {
                    id: Some(second.reasons[0].id),
                    label: "Stolen".into(),
                    description: None,
                }],
            },
        )
        .await;

        assert!(
            matches!(
                result,
                Err(ModelError::Data(DataError::ResourceNotFound(_)))
            ),
            "reason from another policy was accepted"
        );
        let first_after = get_by_id(&pool, conversation_id, first.policy.id).await?;
        assert_eq!(
            first_after.reasons.len(),
            DEFAULT_REASONS.len(),
            "failed update wasn't rolled back"
        );

        Ok(())
    }
}
