use chrono::{DateTime, Duration, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, SimpleExpr, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::error::ComhairleError;
use crate::middleware::request_logging::{ClientIp, ClientUserAgent};
use crate::models::SqlxResultExt;

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "refresh_token")]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub family_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_reason: Option<String>,
    pub replaced_by: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const DEFAULT_COLUMNS: [RefreshTokenIden; 11] = [
    RefreshTokenIden::Id,
    RefreshTokenIden::UserId,
    RefreshTokenIden::FamilyId,
    RefreshTokenIden::ExpiresAt,
    RefreshTokenIden::RevokedAt,
    RefreshTokenIden::RevokedReason,
    RefreshTokenIden::ReplacedBy,
    RefreshTokenIden::IpAddress,
    RefreshTokenIden::UserAgent,
    RefreshTokenIden::CreatedAt,
    RefreshTokenIden::UpdatedAt,
];

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    user_id: Uuid,
    ip_addr: &ClientIp,
    user_agent: &ClientUserAgent,
) -> Result<RefreshToken, ComhairleError> {
    let expires_at = Utc::now() + Duration::days(7);

    let (sql, values) = Query::insert()
        .into_table(RefreshTokenIden::Table)
        .columns([
            RefreshTokenIden::UserId,
            RefreshTokenIden::IpAddress,
            RefreshTokenIden::UserAgent,
            RefreshTokenIden::ExpiresAt,
        ])
        .values([
            user_id.into(),
            ip_addr.0.clone().into(),
            user_agent.0.as_deref().into(),
            expires_at.into(),
        ])?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let token = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(token)
}

#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: Uuid) -> Result<RefreshToken, ComhairleError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RefreshTokenIden::Table)
        .and_where(Expr::col(RefreshTokenIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let token = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Refresh Token")?;

    Ok(token)
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UpdateRefreshToken {
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_reason: Option<String>,
    pub replaced_by: Option<Uuid>,
}

impl UpdateRefreshToken {
    fn values(&self) -> Vec<(RefreshTokenIden, SimpleExpr)> {
        let mut values = vec![];

        if let Some(value) = self.revoked_at {
            values.push((RefreshTokenIden::RevokedAt, value.into()));
        }
        if let Some(value) = &self.revoked_reason {
            values.push((RefreshTokenIden::RevokedReason, value.to_owned().into()));
        }
        if let Some(value) = self.replaced_by {
            values.push((RefreshTokenIden::ReplacedBy, value.into()));
        }

        values
    }
}

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    id: Uuid,
    payload: UpdateRefreshToken,
) -> Result<RefreshToken, ComhairleError> {
    let values = payload.values();

    let (sql, values) = Query::update()
        .table(RefreshTokenIden::Table)
        .values(values)
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .and_where(Expr::col(RefreshTokenIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let token = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .resolve_db_err("Refresh Token")?;

    Ok(token)
}

#[cfg(test)]
mod tests {
    use crate::models::model_test_helpers::{get_random_user_id, setup_default_app_and_session};

    use super::*;

    use chrono::SubsecRound;
    use sqlx::PgPool;
    use std::error::Error;

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_create_refresh_token(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let ip_addr = ClientIp("127.0.0.1".to_string());
        let user_agent = ClientUserAgent(Some("Mozilla/5.0 (X11; Linux x86_64)".to_string()));
        let user_id = get_random_user_id(&app, &mut session).await?;

        let now = Utc::now();

        let token = create(&pool, user_id, &ip_addr, &user_agent).await?;

        assert_eq!(
            token.ip_address.unwrap(),
            ip_addr.0,
            "ip addresses don't match"
        );
        assert!(
            token.expires_at > now + Duration::days(6)
                && token.expires_at < now + Duration::days(8),
            "expires_at incorrect"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_refresh_token(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let ip_addr = ClientIp("127.0.0.1".to_string());
        let user_agent = ClientUserAgent(Some("Mozilla/5.0 (X11; Linux x86_64)".to_string()));
        let user_id = get_random_user_id(&app, &mut session).await?;

        let original_token = create(&pool, user_id, &ip_addr, &user_agent).await?;
        let rotation_token = create(&pool, user_id, &ip_addr, &user_agent).await?;
        let revoked_at = Utc::now();

        assert!(
            original_token.revoked_reason.is_none(),
            "incorrect revoked_reason before update"
        );
        assert!(
            original_token.revoked_at.is_none(),
            "incorrect revoked_at before update"
        );
        assert!(
            original_token.replaced_by.is_none(),
            "incorrect replaced_by before update"
        );

        let updated_original = update(
            &pool,
            original_token.id,
            UpdateRefreshToken {
                revoked_at: Some(revoked_at),
                revoked_reason: Some("Token rotation".to_string()),
                replaced_by: Some(rotation_token.id),
            },
        )
        .await?;

        assert_eq!(
            updated_original.revoked_reason.unwrap(),
            "Token rotation".to_string(),
            "incorrect revoked_reason after update"
        );
        assert_eq!(
            updated_original.revoked_at.unwrap(),
            revoked_at.trunc_subsecs(6), // Postgres only stores up to 6 decimals
            "incorrect revoked_at after update"
        );
        assert_eq!(
            updated_original.replaced_by.unwrap(),
            rotation_token.id,
            "incorrect replaced_by after update"
        );

        Ok(())
    }
}
