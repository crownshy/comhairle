//! Tests for [`crate::models::api_key`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use crate::models::users;

    use crate::models::api_key::*;
    #[allow(unused_imports)]
    use crate::models::error::{
        AuthError, ConversationError, DataError, EventError, InviteError, ModelError,
        PermissionError, ReportError, UserError, ValidationError, WorkflowError,
    };
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

    #[test]
    fn same_keys_produce_same_hashes() {
        let hash_1 = hash_api_key("sk_test_somekey");
        let hash_2 = hash_api_key("sk_test_somekey");
        assert_eq!(hash_1, hash_2);
    }

    #[test]
    fn different_keys_produce_different_hashes() {
        let hash_1 = hash_api_key("sk_test_somekey");
        let hash_2 = hash_api_key("sk_test_someotherkey");
        assert_ne!(hash_1, hash_2);
    }

    #[test]
    fn hash_is_correct_length() {
        let hash = hash_api_key("sk_test_somekey");
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn key_has_correct_prefix() {
        let (raw, _) = generate_api_key("sk_test");
        assert!(raw.starts_with("sk_test"));
    }

    #[test]
    fn generated_keys_are_unique() {
        let (raw_1, _) = generate_api_key("sk_test");
        let (raw_2, _) = generate_api_key("sk_test");
        assert_ne!(raw_1, raw_2);
    }

    #[test]
    fn hash_matches_generated_key() {
        let (raw, hash) = generate_api_key("sk_test");
        assert_eq!(hash, hash_api_key(&raw));
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_matching_user_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let user = users::create_guest_user(&pool).await?;

        let key = create(
            &pool,
            user.id,
            CreateApiKeyRequest {
                name: "test_api_key".to_string(),
                prefix: "sk_test".to_string(),
            },
        )
        .await?;

        let user_id = get_matching_user_id(&pool, &key).await?;

        assert_eq!(user_id, user.id, "ids don't match");

        Ok(())
    }
}
