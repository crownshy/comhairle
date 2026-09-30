//! Tests for [`crate::models::translations`], kept in the api crate because they
//! boot the HTTP app and build fixtures through the API.

mod tests {
    use comhairle_macros::TranslatableJson;

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
    use crate::models::translations::*;
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

    struct TestWithTranslations {
        title: TextContentId,
        description: TextContentId,
    }

    struct LocalizedTestWithTranslations {
        title: String,
        description: String,
    }

    impl CollectTextContentIds for TestWithTranslations {
        fn collect_text_content_ids(&self, out: &mut HashSet<TextContentId>) {
            out.insert(self.title);
            out.insert(self.description);
        }
    }

    impl LocalizeTranslations for TestWithTranslations {
        type Localized = LocalizedTestWithTranslations;

        fn localize(self, map: &HashMap<TextContentId, String>) -> Self::Localized {
            LocalizedTestWithTranslations {
                title: map.get(&self.title).cloned().unwrap_or_default(),
                description: map.get(&self.description).cloned().unwrap_or_default(),
            }
        }
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_resolve_translations_for_arbitrary_list(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let trans_a = new_translation(&pool, "en", "Test text a", TextFormat::Plain).await?;
        let trans_b = new_translation(&pool, "en", "Test text b", TextFormat::Plain).await?;
        let trans_c = new_translation(&pool, "en", "Test text c", TextFormat::Plain).await?;

        let ids = [trans_a.id, trans_b.id, trans_c.id];

        let result = localize_translations(&pool, &ids, "en").await?;

        assert_eq!(
            result.get(&trans_a.id).unwrap(),
            "Test text a",
            "missing a translation"
        );
        assert_eq!(
            result.get(&trans_b.id).unwrap(),
            "Test text b",
            "missing b translation"
        );
        assert_eq!(
            result.get(&trans_c.id).unwrap(),
            "Test text c",
            "missing c translation"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_resolve_translations_to_primary_locale_if_locale_translation_missing(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let trans_a = new_translation(&pool, "en", "Test text a", TextFormat::Plain).await?;
        let trans_b = new_translation(&pool, "en", "Test text b", TextFormat::Plain).await?;
        let trans_c = new_translation(&pool, "en", "Test text c", TextFormat::Plain).await?;

        let ids = [trans_a.id, trans_b.id, trans_c.id];

        let result = localize_translations(&pool, &ids, "fr").await?;

        assert_eq!(result.get(&trans_a.id).unwrap(), "Test text a");
        assert_eq!(result.get(&trans_b.id).unwrap(), "Test text b");
        assert_eq!(result.get(&trans_c.id).unwrap(), "Test text c");

        Ok(())
    }

    /// `get_text_translation_optional` turns "no translation for this locale" into
    /// `Ok(None)` by matching on the not-found error. That match is load bearing and
    /// invisible to the type system: if the error it matches on ever changes shape,
    /// the arm silently stops matching and a tolerated miss becomes a hard failure.
    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_return_none_when_no_translation_exists_for_locale(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let content = new_translation(&pool, "en", "Test text", TextFormat::Plain).await?;

        let found = get_text_translation_optional(&pool, &content.id, "en").await?;
        assert!(found.is_some(), "expected the English translation");

        let missing = get_text_translation_optional(&pool, &content.id, "fr").await?;
        assert!(
            missing.is_none(),
            "expected None for a locale with no translation, got {missing:?}"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_collect_text_content_ids_for_struct(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let title = new_translation(&pool, "en", "Test title", TextFormat::Plain).await?;
        let description =
            new_translation(&pool, "en", "Test description", TextFormat::Plain).await?;

        let mut out: HashSet<TextContentId> = HashSet::new();
        let test_translation = TestWithTranslations {
            title: title.id,
            description: description.id,
        };

        test_translation.collect_text_content_ids(&mut out);

        assert!(out.contains(&test_translation.title), "missing title id");
        assert!(
            out.contains(&test_translation.description),
            "missing description id"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_resolve_translations_for_struct_fields(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let title = new_translation(&pool, "en", "Test title", TextFormat::Plain).await?;
        let description =
            new_translation(&pool, "en", "Test description", TextFormat::Plain).await?;

        let mut out: HashSet<TextContentId> = HashSet::new();
        let test_translation = TestWithTranslations {
            title: title.id,
            description: description.id,
        };

        test_translation.collect_text_content_ids(&mut out);

        let map = localize_translations(&pool, &out.into_iter().collect::<Vec<_>>(), "en").await?;

        let resolved = test_translation.localize(&map);

        assert_eq!(resolved.title, "Test title", "incorrect title");
        assert_eq!(
            resolved.description, "Test description",
            "incorrect descrition"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_auto_implement_traits_to_resolve_translations(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        #[derive(TranslatableJson)]
        struct Root {
            #[translatable]
            title: TextContentId,
            #[translatable]
            nested_arr: Vec<Nested>,
        }

        #[derive(TranslatableJson)]
        struct Nested {
            #[translatable]
            text_field: TextContentId,
            other_field: String,
        }

        let root_title = new_translation(&pool, "en", "Root title", TextFormat::Plain).await?;
        let nested_a = new_translation(&pool, "en", "Nested a", TextFormat::Plain).await?;
        let nested_b = new_translation(&pool, "en", "Nested b", TextFormat::Plain).await?;

        let root = Root {
            title: root_title.id,
            nested_arr: vec![
                Nested {
                    text_field: nested_a.id,
                    other_field: "test a".to_string(),
                },
                Nested {
                    text_field: nested_b.id,
                    other_field: "test b".to_string(),
                },
            ],
        };

        let mut out: HashSet<TextContentId> = HashSet::new();
        root.collect_text_content_ids(&mut out);
        let map = localize_translations(&pool, &out.into_iter().collect::<Vec<_>>(), "en").await?;
        let resolved = root.localize(&map);

        assert_eq!(
            resolved.title,
            "Root title".to_string(),
            "incorrect root title"
        );
        assert_eq!(
            resolved.nested_arr[0].text_field, "Nested a",
            "incorrect nested a"
        );
        assert_eq!(
            resolved.nested_arr[1].text_field, "Nested b",
            "incorrect nested a"
        );

        Ok(())
    }
}
