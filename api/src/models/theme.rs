//! Themes: the topics a theme collection groups together for a workflow step. A
//! collection belongs to a workflow step, and each theme belongs to a collection.

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use sea_query::{Expr, Order, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::models::SqlxResultExt;

#[derive(Serialize, Deserialize, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "theme_collection")]
pub struct ThemeCollection {
    pub id: Uuid,
    pub workflow_step_id: Uuid,
    pub context: String,
    pub origin_tool: String,
    pub origin_sub_id: Option<String>,
    pub requested_by_user_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const COLLECTION_COLUMNS: [ThemeCollectionIden; 8] = [
    ThemeCollectionIden::Id,
    ThemeCollectionIden::WorkflowStepId,
    ThemeCollectionIden::Context,
    ThemeCollectionIden::OriginTool,
    ThemeCollectionIden::OriginSubId,
    ThemeCollectionIden::RequestedByUserId,
    ThemeCollectionIden::CreatedAt,
    ThemeCollectionIden::UpdatedAt,
];

#[derive(Serialize, Deserialize, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "theme")]
pub struct Theme {
    pub id: Uuid,
    pub theme_collection_id: Uuid,
    pub title: String,
    pub description: String,
    pub created_by_ai: bool,
    /// Carried forward as an "existing theme" into the collection's next generation
    /// run. Manually created themes (`created_by_ai = false`) are always carried
    /// forward regardless of this flag.
    pub pinned: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const THEME_COLUMNS: [ThemeIden; 8] = [
    ThemeIden::Id,
    ThemeIden::ThemeCollectionId,
    ThemeIden::Title,
    ThemeIden::Description,
    ThemeIden::CreatedByAi,
    ThemeIden::Pinned,
    ThemeIden::CreatedAt,
    ThemeIden::UpdatedAt,
];

fn default_origin_tool() -> String {
    "manual".to_string()
}

#[derive(Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateThemeCollection {
    pub context: String,
    #[serde(default = "default_origin_tool")]
    pub origin_tool: String,
    #[serde(default)]
    pub origin_sub_id: Option<String>,
    #[serde(default)]
    pub requested_by_user_id: Option<Uuid>,
}

#[derive(Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NewTheme {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub created_by_ai: bool,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdateTheme {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdateThemePinned {
    pub pinned: bool,
}

fn clean_text(field: &str, value: &str) -> Result<String, ComhairleError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ComhairleError::BadRequest(format!(
            "{field} can't be blank"
        )));
    }
    Ok(value.to_string())
}

/// Gets the workflow step's theme collection for this origin tool/sub id, creating
/// it if none exists yet. A collection is a durable, per-(workflow_step, origin_tool,
/// origin_sub_id) container: organizers can add themes to it before any generation
/// has run, and repeated generation runs accumulate into the same collection rather
/// than starting a fresh one each time. An existing collection's `context` and
/// `requested_by_user_id` are refreshed to the latest call's values.
#[instrument(err(Debug), skip(db))]
pub async fn create_collection(
    db: &PgPool,
    workflow_step_id: Uuid,
    payload: &CreateThemeCollection,
) -> Result<ThemeCollection, ComhairleError> {
    let context = clean_text("Context", &payload.context)?;

    let mut select = Query::select();
    select
        .columns(COLLECTION_COLUMNS)
        .from(ThemeCollectionIden::Table)
        .and_where(Expr::col(ThemeCollectionIden::WorkflowStepId).eq(workflow_step_id))
        .and_where(Expr::col(ThemeCollectionIden::OriginTool).eq(payload.origin_tool.clone()));
    match &payload.origin_sub_id {
        Some(sub_id) => {
            select.and_where(Expr::col(ThemeCollectionIden::OriginSubId).eq(sub_id.clone()));
        }
        None => {
            select.and_where(Expr::col(ThemeCollectionIden::OriginSubId).is_null());
        }
    }
    let (sql, values) = select.build_sqlx(PostgresQueryBuilder);
    let existing: Option<ThemeCollection> = query_as_with(&sql, values).fetch_optional(db).await?;

    if let Some(existing) = existing {
        let (sql, values) = Query::update()
            .table(ThemeCollectionIden::Table)
            .values([
                (ThemeCollectionIden::Context, context.into()),
                (
                    ThemeCollectionIden::RequestedByUserId,
                    payload.requested_by_user_id.into(),
                ),
                (
                    ThemeCollectionIden::UpdatedAt,
                    Expr::current_timestamp().into(),
                ),
            ])
            .and_where(Expr::col(ThemeCollectionIden::Id).eq(existing.id))
            .returning(Query::returning().columns(COLLECTION_COLUMNS))
            .build_sqlx(PostgresQueryBuilder);

        return query_as_with(&sql, values)
            .fetch_one(db)
            .await
            .resolve_db_err("Theme Collection");
    }

    let (sql, values) = Query::insert()
        .into_table(ThemeCollectionIden::Table)
        .columns([
            ThemeCollectionIden::WorkflowStepId,
            ThemeCollectionIden::Context,
            ThemeCollectionIden::OriginTool,
            ThemeCollectionIden::OriginSubId,
            ThemeCollectionIden::RequestedByUserId,
        ])
        .values([
            workflow_step_id.into(),
            context.into(),
            payload.origin_tool.clone().into(),
            payload.origin_sub_id.clone().into(),
            payload.requested_by_user_id.into(),
        ])?
        .returning(Query::returning().columns(COLLECTION_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Theme Collection")
}

#[instrument(err(Debug), skip(db))]
async fn get_collection(
    db: &PgPool,
    workflow_step_id: Uuid,
    id: Uuid,
) -> Result<ThemeCollection, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(COLLECTION_COLUMNS)
        .from(ThemeCollectionIden::Table)
        .and_where(Expr::col(ThemeCollectionIden::Id).eq(id))
        .and_where(Expr::col(ThemeCollectionIden::WorkflowStepId).eq(workflow_step_id))
        .build_sqlx(PostgresQueryBuilder);

    query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Theme Collection")
}

/// Fetches the most recently active theme collection for a workflow step, optionally
/// narrowed to a single origin tool, so the UI can restore the last generated themes
/// after a page refresh without needing to re-run generation. Ordered by `updated_at`,
/// not `created_at`: `create_collection` reuses (and bumps `updated_at` on) an
/// existing collection rather than creating a new row each generation run, so
/// `created_at` alone would keep surfacing whichever collection happened to be
/// created last, not the one most recently generated into.
#[instrument(err(Debug), skip(db))]
pub async fn get_latest_for_workflow_step(
    db: &PgPool,
    workflow_step_id: Uuid,
    origin_tool: Option<&str>,
) -> Result<Option<ThemeCollection>, ComhairleError> {
    let mut query = Query::select();
    query
        .columns(COLLECTION_COLUMNS)
        .from(ThemeCollectionIden::Table)
        .and_where(Expr::col(ThemeCollectionIden::WorkflowStepId).eq(workflow_step_id))
        .order_by(ThemeCollectionIden::UpdatedAt, Order::Desc)
        .limit(1);

    if let Some(origin_tool) = origin_tool {
        query.and_where(Expr::col(ThemeCollectionIden::OriginTool).eq(origin_tool));
    }

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let collection = query_as_with(&sql, values).fetch_optional(db).await?;

    Ok(collection)
}

#[instrument(err(Debug), skip(db))]
pub async fn list_themes(
    db: &PgPool,
    workflow_step_id: Uuid,
    theme_collection_id: Uuid,
) -> Result<(ThemeCollection, Vec<Theme>), ComhairleError> {
    let collection = get_collection(db, workflow_step_id, theme_collection_id).await?;

    let (sql, values) = Query::select()
        .columns(THEME_COLUMNS)
        .from(ThemeIden::Table)
        .and_where(Expr::col(ThemeIden::ThemeCollectionId).eq(theme_collection_id))
        .order_by(ThemeIden::CreatedAt, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let themes = query_as_with(&sql, values).fetch_all(db).await?;

    Ok((collection, themes))
}

#[instrument(err(Debug), skip(db))]
pub async fn add_theme(
    db: &PgPool,
    workflow_step_id: Uuid,
    theme_collection_id: Uuid,
    payload: &NewTheme,
) -> Result<Theme, ComhairleError> {
    get_collection(db, workflow_step_id, theme_collection_id).await?;

    let title = clean_text("Title", &payload.title)?;
    let description = clean_text("Description", &payload.description)?;

    let (sql, values) = Query::insert()
        .into_table(ThemeIden::Table)
        .columns([
            ThemeIden::ThemeCollectionId,
            ThemeIden::Title,
            ThemeIden::Description,
            ThemeIden::CreatedByAi,
        ])
        .values([
            theme_collection_id.into(),
            title.into(),
            description.into(),
            payload.created_by_ai.into(),
        ])?
        .returning(Query::returning().columns(THEME_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Theme")
}

/// Themes carried forward as "existing themes" for the collection's next generation
/// run: pinned ones, plus every manually created theme (organizer intent is always
/// carried forward, pin or not).
#[instrument(err(Debug), skip(db))]
pub async fn list_carry_forward_themes(
    db: &PgPool,
    theme_collection_id: Uuid,
) -> Result<Vec<Theme>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(THEME_COLUMNS)
        .from(ThemeIden::Table)
        .and_where(Expr::col(ThemeIden::ThemeCollectionId).eq(theme_collection_id))
        .and_where(
            Expr::col(ThemeIden::Pinned)
                .eq(true)
                .or(Expr::col(ThemeIden::CreatedByAi).eq(false)),
        )
        .order_by(ThemeIden::CreatedAt, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);

    let themes = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(themes)
}

/// Clears out last run's unpinned, AI-generated themes before a new generation run,
/// so regenerating replaces the previous AI themes instead of piling on top of them.
/// Pinned themes and manually created ones are never touched here - they're what
/// `list_carry_forward_themes` feeds back into the next run.
#[instrument(err(Debug), skip(db))]
pub async fn delete_stale_generated_themes(
    db: &PgPool,
    theme_collection_id: Uuid,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ThemeIden::Table)
        .and_where(Expr::col(ThemeIden::ThemeCollectionId).eq(theme_collection_id))
        .and_where(Expr::col(ThemeIden::Pinned).eq(false))
        .and_where(Expr::col(ThemeIden::CreatedByAi).eq(true))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values).execute(db).await?;

    Ok(())
}

#[instrument(err(Debug), skip(db))]
pub async fn set_pinned(
    db: &PgPool,
    theme_collection_id: Uuid,
    theme_id: Uuid,
    pinned: bool,
) -> Result<Theme, ComhairleError> {
    let (sql, values) = Query::update()
        .table(ThemeIden::Table)
        .values([
            (ThemeIden::Pinned, pinned.into()),
            (ThemeIden::UpdatedAt, Expr::current_timestamp().into()),
        ])
        .and_where(Expr::col(ThemeIden::Id).eq(theme_id))
        .and_where(Expr::col(ThemeIden::ThemeCollectionId).eq(theme_collection_id))
        .returning(Query::returning().columns(THEME_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Theme")
}

#[instrument(err(Debug), skip(db))]
pub async fn update_theme(
    db: &PgPool,
    theme_collection_id: Uuid,
    theme_id: Uuid,
    payload: &UpdateTheme,
) -> Result<Theme, ComhairleError> {
    let title = clean_text("Title", &payload.title)?;
    let description = clean_text("Description", &payload.description)?;

    let (sql, values) = Query::update()
        .table(ThemeIden::Table)
        .values([
            (ThemeIden::Title, title.into()),
            (ThemeIden::Description, description.into()),
            (ThemeIden::UpdatedAt, Expr::current_timestamp().into()),
        ])
        .and_where(Expr::col(ThemeIden::Id).eq(theme_id))
        .and_where(Expr::col(ThemeIden::ThemeCollectionId).eq(theme_collection_id))
        .returning(Query::returning().columns(THEME_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Theme")
}

#[instrument(err(Debug), skip(db))]
pub async fn delete_theme(
    db: &PgPool,
    theme_collection_id: Uuid,
    theme_id: Uuid,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ThemeIden::Table)
        .and_where(Expr::col(ThemeIden::Id).eq(theme_id))
        .and_where(Expr::col(ThemeIden::ThemeCollectionId).eq(theme_collection_id))
        .build_sqlx(PostgresQueryBuilder);

    let deleted = sqlx::query_with(&sql, values).execute(db).await?;
    if deleted.rows_affected() == 0 {
        return Err(ComhairleError::ResourceNotFound("Theme".into()));
    }

    Ok(())
}

#[instrument(err(Debug), skip(db))]
pub async fn delete_collection(
    db: &PgPool,
    workflow_step_id: Uuid,
    id: Uuid,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ThemeCollectionIden::Table)
        .and_where(Expr::col(ThemeCollectionIden::Id).eq(id))
        .and_where(Expr::col(ThemeCollectionIden::WorkflowStepId).eq(workflow_step_id))
        .build_sqlx(PostgresQueryBuilder);

    let deleted = sqlx::query_with(&sql, values).execute(db).await?;
    if deleted.rows_affected() == 0 {
        return Err(ComhairleError::ResourceNotFound("Theme Collection".into()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    use serde_json::json;

    use crate::models::model_test_helpers::{
        get_random_conversation_id, get_random_workflow_id, setup_default_app_and_session,
    };
    use crate::routes::workflow_steps::dto::WorkflowStepDto;
    use crate::test_helpers::learn_tool_config;

    async fn get_random_workflow_step_id(
        app: &axum::Router,
        session: &mut crate::test_helpers::UserSession,
    ) -> Result<Uuid, Box<dyn Error>> {
        let conversation_id = get_random_conversation_id(app, session).await?;
        let workflow_id = get_random_workflow_id(app, session, &conversation_id).await?;

        let (_, value, _) = session
            .create_workflow_step(
                app,
                &conversation_id.to_string(),
                &workflow_id.to_string(),
                json!({
                    "name": "test_workflow_step",
                    "step_order": 1,
                    "activation_rule": "manual",
                    "description": "A test workflow step",
                    "is_offline": false,
                    "required": false,
                    "tool_setup": learn_tool_config(),
                }),
            )
            .await?;
        let workflow_step: WorkflowStepDto = serde_json::from_value(value)?;

        Ok(workflow_step.id)
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_collection_and_add_themes(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step_id = get_random_workflow_step_id(&app, &mut session).await?;

        let collection = create_collection(
            &pool,
            workflow_step_id,
            &CreateThemeCollection {
                context: "Statements about transport".into(),
                origin_tool: "manual".into(),
                origin_sub_id: None,
                requested_by_user_id: None,
            },
        )
        .await?;

        let added = add_theme(
            &pool,
            workflow_step_id,
            collection.id,
            &NewTheme {
                title: "Buses".into(),
                description: "More frequent buses".into(),
                created_by_ai: true,
            },
        )
        .await?;
        assert!(added.created_by_ai, "created_by_ai not saved");

        let (_, themes) = list_themes(&pool, workflow_step_id, collection.id).await?;
        assert_eq!(themes.len(), 1, "incorrect theme count");
        assert_eq!(themes[0].id, added.id, "wrong theme returned");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_and_delete_theme(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step_id = get_random_workflow_step_id(&app, &mut session).await?;
        let collection = create_collection(
            &pool,
            workflow_step_id,
            &CreateThemeCollection {
                context: "Context".into(),
                origin_tool: "manual".into(),
                origin_sub_id: None,
                requested_by_user_id: None,
            },
        )
        .await?;
        let theme = add_theme(
            &pool,
            workflow_step_id,
            collection.id,
            &NewTheme {
                title: "Original".into(),
                description: "Original description".into(),
                created_by_ai: false,
            },
        )
        .await?;

        let updated = update_theme(
            &pool,
            collection.id,
            theme.id,
            &UpdateTheme {
                title: "Renamed".into(),
                description: "New description".into(),
            },
        )
        .await?;
        assert_eq!(updated.title, "Renamed", "title not updated");

        delete_theme(&pool, collection.id, theme.id).await?;
        let (_, themes) = list_themes(&pool, workflow_step_id, collection.id).await?;
        assert!(themes.is_empty(), "theme not deleted");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_collection_and_cascade_themes(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step_id = get_random_workflow_step_id(&app, &mut session).await?;
        let collection = create_collection(
            &pool,
            workflow_step_id,
            &CreateThemeCollection {
                context: "Context".into(),
                origin_tool: "manual".into(),
                origin_sub_id: None,
                requested_by_user_id: None,
            },
        )
        .await?;
        add_theme(
            &pool,
            workflow_step_id,
            collection.id,
            &NewTheme {
                title: "Title".into(),
                description: "Description".into(),
                created_by_ai: false,
            },
        )
        .await?;

        delete_collection(&pool, workflow_step_id, collection.id).await?;

        let result = list_themes(&pool, workflow_step_id, collection.id).await;
        assert!(
            matches!(result, Err(ComhairleError::ResourceNotFound(_))),
            "deleted collection still readable"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_latest_collection_for_workflow_step(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step_id = get_random_workflow_step_id(&app, &mut session).await?;

        assert!(
            get_latest_for_workflow_step(&pool, workflow_step_id, None)
                .await?
                .is_none(),
            "no collection should exist yet"
        );

        create_collection(
            &pool,
            workflow_step_id,
            &CreateThemeCollection {
                context: "Polis statements".into(),
                origin_tool: "polis".into(),
                origin_sub_id: None,
                requested_by_user_id: None,
            },
        )
        .await?;
        let heyform = create_collection(
            &pool,
            workflow_step_id,
            &CreateThemeCollection {
                context: "HeyForm statements".into(),
                origin_tool: "heyform".into(),
                origin_sub_id: None,
                requested_by_user_id: None,
            },
        )
        .await?;

        let latest = get_latest_for_workflow_step(&pool, workflow_step_id, Some("heyform"))
            .await?
            .expect("a collection should exist");
        assert_eq!(
            latest.id, heyform.id,
            "should return the matching tool's collection"
        );

        assert!(
            get_latest_for_workflow_step(&pool, workflow_step_id, Some("prioritization"))
                .await?
                .is_none(),
            "should not match a different origin tool"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn create_collection_is_idempotent_per_origin(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step_id = get_random_workflow_step_id(&app, &mut session).await?;

        let first = create_collection(
            &pool,
            workflow_step_id,
            &CreateThemeCollection {
                context: "Original context".into(),
                origin_tool: "polis".into(),
                origin_sub_id: None,
                requested_by_user_id: None,
            },
        )
        .await?;

        let second = create_collection(
            &pool,
            workflow_step_id,
            &CreateThemeCollection {
                context: "Updated context".into(),
                origin_tool: "polis".into(),
                origin_sub_id: None,
                requested_by_user_id: None,
            },
        )
        .await?;

        assert_eq!(first.id, second.id, "should reuse the same collection");
        assert_eq!(second.context, "Updated context", "context should refresh");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_pin_and_list_carry_forward_themes(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step_id = get_random_workflow_step_id(&app, &mut session).await?;
        let collection = create_collection(
            &pool,
            workflow_step_id,
            &CreateThemeCollection {
                context: "Context".into(),
                origin_tool: "polis".into(),
                origin_sub_id: None,
                requested_by_user_id: None,
            },
        )
        .await?;

        let manual = add_theme(
            &pool,
            workflow_step_id,
            collection.id,
            &NewTheme {
                title: "Manual".into(),
                description: "Added by an organizer".into(),
                created_by_ai: false,
            },
        )
        .await?;
        let pinned_ai = add_theme(
            &pool,
            workflow_step_id,
            collection.id,
            &NewTheme {
                title: "Pinned AI theme".into(),
                description: "Generated, then pinned".into(),
                created_by_ai: true,
            },
        )
        .await?;
        let unpinned_ai = add_theme(
            &pool,
            workflow_step_id,
            collection.id,
            &NewTheme {
                title: "Unpinned AI theme".into(),
                description: "Generated, never pinned".into(),
                created_by_ai: true,
            },
        )
        .await?;
        assert!(!manual.pinned, "themes should default unpinned");

        let pinned = set_pinned(&pool, collection.id, pinned_ai.id, true).await?;
        assert!(pinned.pinned, "pinned flag not saved");

        let carry_forward = list_carry_forward_themes(&pool, collection.id).await?;
        let carry_forward_ids: Vec<Uuid> = carry_forward.iter().map(|t| t.id).collect();
        assert!(
            carry_forward_ids.contains(&manual.id),
            "manual themes should always carry forward"
        );
        assert!(
            carry_forward_ids.contains(&pinned_ai.id),
            "pinned AI themes should carry forward"
        );
        assert!(
            !carry_forward_ids.contains(&unpinned_ai.id),
            "unpinned AI themes should not carry forward"
        );

        let unpinned = set_pinned(&pool, collection.id, pinned_ai.id, false).await?;
        assert!(!unpinned.pinned, "unpinning not saved");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_only_unpinned_ai_themes(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let workflow_step_id = get_random_workflow_step_id(&app, &mut session).await?;
        let collection = create_collection(
            &pool,
            workflow_step_id,
            &CreateThemeCollection {
                context: "Context".into(),
                origin_tool: "polis".into(),
                origin_sub_id: None,
                requested_by_user_id: None,
            },
        )
        .await?;

        let manual = add_theme(
            &pool,
            workflow_step_id,
            collection.id,
            &NewTheme {
                title: "Manual".into(),
                description: "Added by an organizer".into(),
                created_by_ai: false,
            },
        )
        .await?;
        let pinned_ai = add_theme(
            &pool,
            workflow_step_id,
            collection.id,
            &NewTheme {
                title: "Pinned AI theme".into(),
                description: "Generated, then pinned".into(),
                created_by_ai: true,
            },
        )
        .await?;
        set_pinned(&pool, collection.id, pinned_ai.id, true).await?;
        let unpinned_ai = add_theme(
            &pool,
            workflow_step_id,
            collection.id,
            &NewTheme {
                title: "Stale AI theme".into(),
                description: "Generated last run, never pinned".into(),
                created_by_ai: true,
            },
        )
        .await?;

        delete_stale_generated_themes(&pool, collection.id).await?;

        let (_, remaining) = list_themes(&pool, workflow_step_id, collection.id).await?;
        let remaining_ids: Vec<Uuid> = remaining.iter().map(|t| t.id).collect();
        assert!(
            remaining_ids.contains(&manual.id),
            "manual theme should survive"
        );
        assert!(
            remaining_ids.contains(&pinned_ai.id),
            "pinned theme should survive"
        );
        assert!(
            !remaining_ids.contains(&unpinned_ai.id),
            "stale unpinned AI theme should be deleted"
        );

        Ok(())
    }
}
