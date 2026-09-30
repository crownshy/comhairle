use chrono::{DateTime, Duration, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow, query_as_with};
use tracing::instrument;
use uuid::Uuid;

use crate::models::SqlxResultExt;
use crate::models::error::{AuthError, DataError};
use crate::models::request_context::{ClientIp, ClientUserAgent};

const REFRESH_TOKEN_TTL: Duration = Duration::days(7);

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

#[derive(Debug)]
pub struct CreateRefreshToken<'a> {
    pub user_id: Uuid,
    pub ip_addr: &'a ClientIp,
    pub user_agent: &'a ClientUserAgent,
    pub family_id: Option<Uuid>,
    pub custom_expiry: Option<DateTime<Utc>>,
}

#[instrument(err(Debug))]
pub async fn create<'e, E>(
    db: E,
    payload: CreateRefreshToken<'e>,
) -> Result<RefreshToken, DataError>
where
    E: sqlx::PgExecutor<'e>,
{
    let mut columns = vec![
        RefreshTokenIden::UserId,
        RefreshTokenIden::IpAddress,
        RefreshTokenIden::UserAgent,
        RefreshTokenIden::ExpiresAt,
    ];
    let mut values = vec![
        payload.user_id.into(),
        payload.ip_addr.0.clone().into(),
        payload.user_agent.0.as_deref().into(),
        payload
            .custom_expiry
            .unwrap_or_else(|| Utc::now() + REFRESH_TOKEN_TTL)
            .into(),
    ];

    if let Some(family_id) = payload.family_id {
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
/// - `Err(DataError::DatabaseError(_)` — the query itself failed
///   (connection/db error), unrelated to whether the token is valid.
#[instrument(err(Debug))]
pub async fn get_by_id<'e, E>(db: E, id: Uuid) -> Result<Option<RefreshToken>, DataError>
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

#[instrument(err(Debug))]
pub async fn revoke_for_user(
    db: &PgPool,
    user_id: Uuid,
    revoked_reason: &str,
) -> Result<Vec<RefreshToken>, DataError> {
    let (sql, values) = Query::update()
        .table(RefreshTokenIden::Table)
        .values([
            (RefreshTokenIden::RevokedAt, Utc::now().into()),
            (RefreshTokenIden::RevokedReason, revoked_reason.into()),
        ])
        .and_where(Expr::col(RefreshTokenIden::UserId).eq(user_id))
        .and_where(Expr::col(RefreshTokenIden::RevokedAt).is_null())
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let tokens = query_as_with(&sql, values)
        .fetch_all(db)
        .await
        .resolve_db_err("Refresh Token")?;

    Ok(tokens)
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
) -> Result<RefreshToken, AuthError> {
    let mut tx = db.begin().await?;

    let old_token = get_by_id(&mut *tx, old_token_id)
        .await?
        .ok_or(AuthError::SessionRefreshFailure(RefreshFailure::NotFound))?;

    if old_token.user_id != user_id {
        return Err(AuthError::SessionRefreshFailure(RefreshFailure::OwnershipMismatch).into());
    }

    if old_token.revoked_at.is_some() {
        // Revoke all tokens in family
        revoke_family(&mut *tx, old_token.family_id, "reuse_detected").await?;
        tx.commit().await.resolve_db_err("Refresh Token")?;
        return Err(AuthError::SessionRefreshFailure(RefreshFailure::ReuseDetected).into());
    }

    if old_token.expires_at < Utc::now() {
        return Err(AuthError::SessionRefreshFailure(RefreshFailure::Expired).into());
    }

    let new_token = create(
        &mut *tx,
        CreateRefreshToken {
            user_id,
            ip_addr,
            user_agent,
            family_id: Some(old_token.family_id),
            custom_expiry: None,
        },
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
        return Err(AuthError::SessionRefreshFailure(RefreshFailure::ReuseDetected).into());
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
) -> Result<Vec<RefreshToken>, DataError>
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
