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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const COLLECTION_COLUMNS: [ThemeCollectionIden; 5] = [
    ThemeCollectionIden::Id,
    ThemeCollectionIden::WorkflowStepId,
    ThemeCollectionIden::Context,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const THEME_COLUMNS: [ThemeIden; 7] = [
    ThemeIden::Id,
    ThemeIden::ThemeCollectionId,
    ThemeIden::Title,
    ThemeIden::Description,
    ThemeIden::CreatedByAi,
    ThemeIden::CreatedAt,
    ThemeIden::UpdatedAt,
];

#[derive(Deserialize, Debug, JsonSchema)]
pub struct CreateThemeCollection {
    pub context: String,
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

fn clean_text(field: &str, value: &str) -> Result<String, ComhairleError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ComhairleError::BadRequest(format!(
            "{field} can't be blank"
        )));
    }
    Ok(value.to_string())
}

#[instrument(err(Debug), skip(db))]
pub async fn create_collection(
    db: &PgPool,
    workflow_step_id: Uuid,
    payload: &CreateThemeCollection,
) -> Result<ThemeCollection, ComhairleError> {
    let context = clean_text("Context", &payload.context)?;

    let (sql, values) = Query::insert()
        .into_table(ThemeCollectionIden::Table)
        .columns([
            ThemeCollectionIden::WorkflowStepId,
            ThemeCollectionIden::Context,
        ])
        .values([workflow_step_id.into(), context.into()])?
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
}
