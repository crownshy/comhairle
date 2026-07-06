use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Utc};
use rand::RngCore;
use schemars::JsonSchema;
use sea_query::{Expr, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, prelude::FromRow, query_scalar_with, query_with};
use uuid::Uuid;

use crate::error::ComhairleError;

#[derive(Serialize, Deserialize, Debug, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "api_key")]
pub struct ApiKey {
    pub id: Uuid,
    pub hash: String,
    pub user_id: Uuid,
    pub name: String,
    pub prefix: String,
    pub revoked_at: Option<DateTime<Utc>>,
    pub expired_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

fn hash_api_key(raw_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw_key.as_bytes());
    hex::encode(hasher.finalize())
}

fn generate_api_key(prefix: &str) -> (String, String) {
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);

    let raw = format!("{}_{}", prefix, URL_SAFE_NO_PAD.encode(bytes));
    let hash = hash_api_key(&raw);

    (raw, hash)
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub prefix: String,
}

// No tracing to avoid keys being exposed in logs
pub async fn create(
    db: &PgPool,
    user_id: Uuid,
    create_key: CreateApiKeyRequest,
) -> Result<String, ComhairleError> {
    let (raw, hash) = generate_api_key(&create_key.prefix);

    let columns = vec![
        ApiKeyIden::Hash,
        ApiKeyIden::UserId,
        ApiKeyIden::Prefix,
        ApiKeyIden::Name,
    ];
    let key_prefix = raw[..16].to_string();
    let values = vec![
        hash.into(),
        user_id.into(),
        key_prefix.into(),
        create_key.name.into(),
    ];

    let (sql, values) = Query::insert()
        .into_table(ApiKeyIden::Table)
        .columns(columns)
        .values(values)?
        .returning_col(ApiKeyIden::Id)
        .build_sqlx(PostgresQueryBuilder);

    let _id = query_with(&sql, values).fetch_one(db).await?;

    Ok(raw)
}

pub async fn get_matching_user_id(db: &PgPool, key: &str) -> Result<Uuid, ComhairleError> {
    let hash = hash_api_key(key);

    let (sql, values) = Query::select()
        .column(ApiKeyIden::UserId)
        .from(ApiKeyIden::Table)
        .and_where(Expr::col(ApiKeyIden::Hash).eq(hash))
        .and_where(Expr::col(ApiKeyIden::RevokedAt).is_null())
        .and_where(
            Expr::col(ApiKeyIden::ExpiredAt)
                .is_null()
                .or(Expr::col(ApiKeyIden::ExpiredAt).gt(Expr::current_timestamp())),
        )
        .build_sqlx(PostgresQueryBuilder);

    let user_id = query_scalar_with::<_, Uuid, _>(&sql, values)
        .fetch_optional(db)
        .await?
        .ok_or(ComhairleError::InvalidApiKey)?;

    Ok(user_id)
}

#[cfg(test)]
mod tests {
    use crate::models::users;

    use super::*;

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
        let user = users::create_annon_user(&pool).await?;

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
