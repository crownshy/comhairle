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
    /// Corresponds to refresh token `jti` claim.
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
pub async fn create<'e, E>(
    db: E,
    user_id: Uuid,
    ip_addr: &ClientIp,
    user_agent: &ClientUserAgent,
    family_id: Option<Uuid>,
) -> Result<RefreshToken, ComhairleError>
where
    E: sqlx::PgExecutor<'e>,
{
    let expires_at = Utc::now() + Duration::days(7);

    let mut columns = vec![
        RefreshTokenIden::UserId,
        RefreshTokenIden::IpAddress,
        RefreshTokenIden::UserAgent,
        RefreshTokenIden::ExpiresAt,
    ];
    let mut values = vec![
        user_id.into(),
        ip_addr.0.clone().into(),
        user_agent.0.as_deref().into(),
        expires_at.into(),
    ];

    if let Some(family_id) = family_id {
        columns.push(RefreshTokenIden::FamilyId);
        values.push(family_id.into());
    }

    let (sql, values) = Query::insert()
        .into_table(RefreshTokenIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let token = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(token)
}

/// Fetches a refresh token record by its primary key.
///
/// The `id` here corresponds to the `jti` claim embedded in the refresh JWT
/// issued to the client. When handling `/auth/refresh`, decode and verify the
/// JWT first, then pass the `jti` claim as `id` to look up the corresponding
/// database record.
///
/// This method only performs the lookup; it does not check whether the
/// token has expired or been revoked. Callers are responsible for checking
/// `expires_at` and `revoked_at` on the returned record before treating the
/// token as valid (see [`rotate`].
///
/// # Returns
///
/// - `Ok(Some(RefreshToken))` — a record exists for this id (may still be
///   expired or revoked; check the relevant fields).
/// - `Ok(None)` — no record exists for this id. This is distinct from
///   "expired" or "revoked" and should generally be treated as a more
///   serious signal (e.g. a forged or tampered `jti`, or referencing a
///   token that was hard-deleted), since a legitimately-issued token should
///   always have a corresponding row, even after rotation/revocation.
/// - `Err(ComhairleError::DatabaseError(_)` — the query itself failed
///   (connection/db error), unrelated to whether the token is valid.
#[instrument(err(Debug))]
pub async fn get_by_id<'e, E>(db: E, id: Uuid) -> Result<Option<RefreshToken>, ComhairleError>
where
    E: sqlx::PgExecutor<'e>,
{
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(RefreshTokenIden::Table)
        .and_where(Expr::col(RefreshTokenIden::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);

    let token = query_as_with(&sql, values)
        .fetch_optional(db)
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

#[derive(Debug)]
pub enum RefreshFailure {
    InvalidClaim,
    Missing,
    NotFound,
    OwnershipMismatch,
    Expired,
    ReuseDetected,
}

impl std::fmt::Display for RefreshFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RefreshFailure::InvalidClaim => write!(f, "Invalid token claim"),
            RefreshFailure::Missing => write!(f, "Token missing"),
            RefreshFailure::NotFound => write!(f, "Token not found"),
            RefreshFailure::OwnershipMismatch => write!(f, "Token ownership mismatch"),
            RefreshFailure::Expired => write!(f, "Token expired"),
            RefreshFailure::ReuseDetected => write!(f, "Token reuse detected"),
        }
    }
}

/// Rotates a refresh token as part of the `/auth/refresh` flow. Validates that
/// `old_token_id` is currently active and belongs to `user_id`, then
/// atomically revokes it and issues a replacement in the same rotation
/// family (`family_id` is carried forward, not regenerated).
///
/// The `id` of the returned `RefreshToken` should be embedded as the `jti`
/// claim of the newly issued refresh JWT (see [`get_by_id`] for the inverse
/// lookup).
///
/// # Reuse detection
///
/// If `old_token_id` refers to a token that has already been rotated or
/// revoked, this is treated as a signal that the token may have been
/// stolen and replayed after the legitimate client already moved past it.
/// In that case, the *entire token family* is revoked via [`revoke_family`].
/// Callers should treat this as a hard failure — force re-authentication
/// rather than retrying.
///
/// A concurrent rotation/revocation of the same row (a race rather than a
/// replay) is handled the same way defensively, since the two cases are
/// indistinguishable from a single request's point of view.
///
/// # Errors
///
/// * `RefreshFailure::NotFound` - Row not found for `old_token_id`.
/// * `RefreshFailure::OwnershipMismatch` - `old_token.user_id` does not match
///   the supplied `user_id` (`jti` claim referred to someone else's token).
/// * `RefreshFailure::ReuseDetected` - The token has already been revoked or
///   the final `UPDATE` affects zero rows (lost a race with a concurrent
///   rotation/revocation of the same row).
/// * `RefreshFailure::Expired` - The token has expired.
#[instrument(err(Debug))]
pub async fn rotate(
    db: &PgPool,
    old_token_id: Uuid,
    user_id: Uuid,
    ip_addr: &ClientIp,
    user_agent: &ClientUserAgent,
) -> Result<RefreshToken, ComhairleError> {
    let mut tx = db.begin().await?;

    let old_token =
        get_by_id(&mut *tx, old_token_id)
            .await?
            .ok_or(ComhairleError::SessionRefreshFailure(
                RefreshFailure::NotFound,
            ))?;

    if old_token.user_id != user_id {
        return Err(ComhairleError::SessionRefreshFailure(
            RefreshFailure::OwnershipMismatch,
        ));
    }

    if old_token.revoked_at.is_some() {
        // Revoke all tokens in family
        revoke_family(&mut *tx, old_token.family_id, "reuse_detected").await?;
        tx.commit().await.resolve_db_err("Refresh Token")?;
        return Err(ComhairleError::SessionRefreshFailure(
            RefreshFailure::ReuseDetected,
        ));
    }

    if old_token.expires_at < Utc::now() {
        return Err(ComhairleError::SessionRefreshFailure(
            RefreshFailure::Expired,
        ));
    }

    let new_token = create(
        &mut *tx,
        user_id,
        ip_addr,
        user_agent,
        Some(old_token.family_id),
    )
    .await?;

    let (sql, values) = Query::update()
        .table(RefreshTokenIden::Table)
        .values([
            (RefreshTokenIden::RevokedAt, Utc::now().into()),
            (RefreshTokenIden::RevokedReason, "rotated".into()),
            (RefreshTokenIden::ReplacedBy, new_token.id.into()),
        ])
        .and_where(Expr::col(RefreshTokenIden::Id).eq(old_token_id))
        .and_where(Expr::col(RefreshTokenIden::RevokedAt).is_null())
        .and_where(Expr::col(RefreshTokenIden::UserId).eq(user_id))
        .and_where(Expr::col(RefreshTokenIden::ExpiresAt).gt(Utc::now()))
        .build_sqlx(PostgresQueryBuilder);

    let result = sqlx::query_with(&sql, values).execute(&mut *tx).await?;

    if result.rows_affected() == 0 {
        // Someone else revoked/rotated between SELECT and UPDATE queries
        revoke_family(&mut *tx, old_token.family_id, "reuse_detected").await?;
        tx.commit().await.resolve_db_err("Refresh Token")?;
        return Err(ComhairleError::SessionRefreshFailure(
            RefreshFailure::ReuseDetected,
        ));
    }

    tx.commit().await?;

    Ok(new_token)
}

/// Revokes every currently-active token in a rotation family.
///
/// Used to contain a suspected token compromise, since every token issued via
/// rotation from a common ancestor shares one `family_id`, revoking the family
/// invalidates the entire lineage in one statement.
#[instrument(err(Debug), skip(db))]
pub async fn revoke_family<'e, E>(
    db: E,
    family_id: Uuid,
    revoke_reason: &str,
) -> Result<Vec<RefreshToken>, ComhairleError>
where
    E: sqlx::PgExecutor<'e>,
{
    let (sql, values) = Query::update()
        .table(RefreshTokenIden::Table)
        .values([
            (RefreshTokenIden::RevokedAt, Utc::now().into()),
            (
                RefreshTokenIden::RevokedReason,
                revoke_reason.to_owned().into(),
            ),
        ])
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .and_where(Expr::col(RefreshTokenIden::FamilyId).eq(family_id))
        .and_where(Expr::col(RefreshTokenIden::RevokedAt).is_null())
        .build_sqlx(PostgresQueryBuilder);

    let tokens = query_as_with(&sql, values).fetch_all(db).await?;

    Ok(tokens)
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

        let token = create(&pool, user_id, &ip_addr, &user_agent, None).await?;

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
    async fn should_return_none_if_no_token_present(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let result = get_by_id(&pool, Uuid::new_v4()).await?;

        assert!(result.is_none());

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_update_refresh_token(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let ip_addr = ClientIp("127.0.0.1".to_string());
        let user_agent = ClientUserAgent(Some("Mozilla/5.0 (X11; Linux x86_64)".to_string()));
        let user_id = get_random_user_id(&app, &mut session).await?;

        let original_token = create(&pool, user_id, &ip_addr, &user_agent, None).await?;
        let rotation_token = create(&pool, user_id, &ip_addr, &user_agent, None).await?;
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

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_revoke_all_tokens_in_family(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let ip_addr = ClientIp("127.0.0.1".to_string());
        let user_agent = ClientUserAgent(Some("Mozilla/5.0 (X11; Linux x86_64)".to_string()));
        let user_id = get_random_user_id(&app, &mut session).await?;

        let unrelated_token = create(&pool, user_id, &ip_addr, &user_agent, None).await?;
        let family_token_a = create(&pool, user_id, &ip_addr, &user_agent, None).await?;
        let family_token_b = create(
            &pool,
            user_id,
            &ip_addr,
            &user_agent,
            Some(family_token_a.family_id),
        )
        .await?;
        let family_token_c = create(
            &pool,
            user_id,
            &ip_addr,
            &user_agent,
            Some(family_token_b.family_id),
        )
        .await?;

        let revoked_tokens = revoke_family(&pool, family_token_a.family_id, "test").await?;

        assert_eq!(revoked_tokens.len(), 3, "incorrect no affected tokens");
        assert!(
            revoked_tokens
                .iter()
                .all(|t| t.revoked_at.is_some() && t.revoked_reason.as_ref().unwrap() == "test"),
            "revoked columns incorrect"
        );
        assert!(
            !revoked_tokens.iter().any(|t| t.id == unrelated_token.id),
            "unrelated token updated"
        );
        assert!(
            unrelated_token.revoked_at.is_none(),
            "unrelated_token revoked"
        );
        assert!(
            revoked_tokens.iter().any(|t| t.id == family_token_a.id),
            "family_token_a missing"
        );
        assert!(
            revoked_tokens.iter().any(|t| t.id == family_token_b.id),
            "family_token_b missing"
        );
        assert!(
            revoked_tokens.iter().any(|t| t.id == family_token_c.id),
            "family_token_c missing"
        );

        Ok(())
    }
}
