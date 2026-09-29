//! Tests for [`crate::models::media`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::models::error::DataError;
    use crate::{
        models::{model_test_helpers::setup_default_app_and_session, users},
        test_helpers::TEST_PASSWORD,
    };

    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, EventError, InviteError, ModelError, PermissionError,
        ReportError, UserError, ValidationError, WorkflowError,
    };
    use crate::models::media::*;
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
    use crate::models::{proposal_section, user_progress};
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
    async fn should_create_media_record(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        session.signup(&app).await?;
        session
            .login(&app, "admin@crown-shy.com", TEST_PASSWORD)
            .await?;

        let (_, user, _) = session.current_user(&app).await?;

        let params = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/test-image.jpg".to_string(),
            filename: "test-image.jpg".to_string(),
            name: "test-image".to_string(),
            alt: "test alt text".to_string(),
            content_type: MediaContentType::Jpeg,
        };

        let media = create(&pool, &params, &user.id).await?;

        assert_eq!(
            media.filename,
            "test-image.jpg".to_string(),
            "incorrect filename"
        );
        assert_eq!(
            media.content_type.to_string(),
            "image/jpeg".to_string(),
            "incorrect deserialized content_type"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_media_record_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        session.signup(&app).await?;
        session
            .login(&app, "admin@crown-shy.com", TEST_PASSWORD)
            .await?;

        let (_, user, _) = session.current_user(&app).await?;

        let params = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/test-image.jpg".to_string(),
            filename: "test-image.jpg".to_string(),
            name: "test-image".to_string(),
            alt: "test alt text".to_string(),
            content_type: MediaContentType::Jpeg,
        };

        let created_media = create(&pool, &params, &user.id).await?;

        let media = get_by_id(&pool, &created_media.id).await?;

        assert_eq!(
            media.filename,
            "test-image.jpg".to_string(),
            "incorrect filename"
        );
        assert_eq!(media.id, created_media.id, "mis-matching ids");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_paginated_list_of_media_records(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        session.signup(&app).await?;
        session
            .login(&app, "admin@crown-shy.com", TEST_PASSWORD)
            .await?;

        let (_, user, _) = session.current_user(&app).await?;

        let params_1 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-b.jpg".to_string(),
            filename: "image-b.jpg".to_string(),
            name: "image-b".to_string(),
            alt: "alt text b".to_string(),
            content_type: MediaContentType::Jpeg,
        };
        let params_2 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-a.jpg".to_string(),
            filename: "image-a.jpg".to_string(),
            name: "image-a".to_string(),
            alt: "alt text a".to_string(),
            content_type: MediaContentType::Jpeg,
        };
        let params_3 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-d.jpg".to_string(),
            filename: "image-d.jpg".to_string(),
            name: "image-d".to_string(),
            alt: "alt text d".to_string(),
            content_type: MediaContentType::Jpeg,
        };
        let params_4 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-c.jpg".to_string(),
            filename: "image-c.jpg".to_string(),
            name: "image-c".to_string(),
            alt: "alt text c".to_string(),
            content_type: MediaContentType::Jpeg,
        };
        let params_5 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-e.jpg".to_string(),
            filename: "image-e.jpg".to_string(),
            name: "image-e".to_string(),
            alt: "alt text e".to_string(),
            content_type: MediaContentType::Jpeg,
        };

        let _ = create(&pool, &params_1, &user.id).await?;
        let _ = create(&pool, &params_2, &user.id).await?;
        let media_3 = create(&pool, &params_3, &user.id).await?;
        let media_4 = create(&pool, &params_4, &user.id).await?;
        let media_5 = create(&pool, &params_5, &user.id).await?;

        let page_options = PageOptions {
            offset: Some(2),
            limit: Some(3),
        };
        let order_options = MediaOrderOptions {
            filename: Some(Order::Asc),
            created_at: None,
        };
        let filter_options = MediaFilterOptions {
            ..Default::default()
        };

        let results = list(&pool, page_options, order_options, filter_options).await?;

        assert_eq!(results.records.len(), 3, "incorrect number of results");
        assert_eq!(results.records[0].id, media_4.id, "incorrect first id");
        assert_eq!(results.records[1].id, media_3.id, "incorrect second id");
        assert_eq!(results.records[2].id, media_5.id, "incorrect third id");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_filter_media_by_owner(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        session.signup(&app).await?;

        let user_1 = users::create_guest_user(&pool).await?;
        let user_2 = users::create_guest_user(&pool).await?;

        let params_1 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-b.jpg".to_string(),
            filename: "image-b.jpg".to_string(),
            name: "image-b".to_string(),
            alt: "alt text b".to_string(),
            content_type: MediaContentType::Jpeg,
        };
        let params_2 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-a.jpg".to_string(),
            filename: "image-a.jpg".to_string(),
            name: "image-a".to_string(),
            alt: "alt text a".to_string(),
            content_type: MediaContentType::Jpeg,
        };
        let params_3 = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/image-d.jpg".to_string(),
            filename: "image-d.jpg".to_string(),
            name: "image-d".to_string(),
            alt: "alt text d".to_string(),
            content_type: MediaContentType::Jpeg,
        };

        let _ = create(&pool, &params_1, &user_2.id).await?;
        let media_2 = create(&pool, &params_2, &user_1.id).await?;
        let media_3 = create(&pool, &params_3, &user_1.id).await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let order_options = MediaOrderOptions {
            ..Default::default()
        };
        let filter_options = MediaFilterOptions {
            owner_id: Some(user_1.id),
            ..Default::default()
        };

        let results = list(&pool, page_options, order_options, filter_options).await?;

        assert_eq!(results.total, 2, "incorrect total");
        assert_eq!(results.records[0].id, media_2.id, "incorrect first id");
        assert_eq!(results.records[1].id, media_3.id, "incorrect second id");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_media_record(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        session.signup(&app).await?;
        session
            .login(&app, "admin@crown-shy.com", TEST_PASSWORD)
            .await?;

        let (_, user, _) = session.current_user(&app).await?;

        let params = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/test-image.jpg".to_string(),
            filename: "test-image.jpg".to_string(),
            name: "test-image".to_string(),
            alt: "test alt text".to_string(),
            content_type: MediaContentType::Jpeg,
        };

        let created_media = create(&pool, &params, &user.id).await?;

        let update_media = MediaEditableFields {
            name: Some("new-name".to_string()),
            ..Default::default()
        };

        let _ = update(&pool, &created_media.id, &update_media).await?;

        let media = get_by_id(&pool, &created_media.id).await?;

        assert_eq!(media.name, "new-name".to_string(), "incorrect filename");
        assert_eq!(
            media.alt,
            "test alt text".to_string(),
            "alt text was modified incorrectly"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_media_record(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        session.signup(&app).await?;
        session
            .login(&app, "admin@crown-shy.com", TEST_PASSWORD)
            .await?;

        let (_, user, _) = session.current_user(&app).await?;

        let params = CreateMedia {
            store_name: "test_media".to_string(),
            storage_key: "asd123/test-image.jpg".to_string(),
            filename: "test-image.jpg".to_string(),
            name: "test-image".to_string(),
            alt: "test alt text".to_string(),
            content_type: MediaContentType::Jpeg,
        };

        let created_media = create(&pool, &params, &user.id).await?;

        let _ = delete(&pool, &created_media.id).await?;

        let err = get_by_id(&pool, &created_media.id).await.unwrap_err();

        match err {
            ModelError::Data(DataError::ResourceNotFound(e)) => {
                assert_eq!(e, "Media".to_string(), "incorrect error message");
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
