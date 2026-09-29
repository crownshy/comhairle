use crate::models::error::ModelError;
use std::fmt;

use crate::models::error::DataError;
use crate::models::error::UserError;
use crate::models::error::ValidationError;
use crate::models::id::gen_id;
use crate::models::password::{hash_pw, validate_password_strength};
use crate::models::{
    pagination::{Order, PageOptions, PaginatedResults},
    permissions::{
        ResourcePermissionIden, ResourceType as PermissionResourceType, Role as PermissionRole,
    },
};
use chrono::{DateTime, Utc};
#[cfg(any(test, feature = "test-util"))]
use fake::Dummy;
use schemars::JsonSchema;
use sea_query::{
    Expr, PostgresQueryBuilder, Query, SelectStatement, enum_def, extension::postgres::PgExpr,
};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use tracing::{instrument, warn};
use uuid::Uuid;

/// Defines the type of authentication has been used to create
/// The user
/// Expected payload for a signin request  
#[derive(Deserialize, Debug, JsonSchema)]
#[cfg_attr(any(test, feature = "test-util"), derive(Dummy))]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub email: String,
}

#[derive(Deserialize, Debug, JsonSchema)]
#[cfg_attr(any(test, feature = "test-util"), derive(Dummy))]
pub struct OtpSignupRequest {
    pub email: String,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
pub enum UserAuthType {
    #[sqlx(rename = "guest")]
    Guest,
    #[sqlx(rename = "email_password")]
    EmailPassword,
    #[sqlx(rename = "one_time_passcode")]
    Otp,
    #[sqlx(rename = "scot_account")]
    ScotAccount,
}

impl From<UserAuthType> for sea_query::Value {
    fn from(val: UserAuthType) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl fmt::Display for UserAuthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            UserAuthType::Guest => "guest",
            UserAuthType::EmailPassword => "email_password",
            UserAuthType::Otp => "one_time_passcode",
            UserAuthType::ScotAccount => "scot_account",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "snake_case")]
pub enum Resource {
    Organisation,
    Conversation,
}

impl Resource {
    pub fn to_str(&self) -> &'static str {
        match self {
            Resource::Organisation => "Organisation",
            Resource::Conversation => "Conversation",
        }
    }
}

impl From<Resource> for sea_query::Value {
    fn from(val: Resource) -> Self {
        val.to_str().into()
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.to_str();
        write!(f, "{}", value)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone)]
#[sqlx(type_name = "text")]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Owner,
    Contributor,
    Translator,
    Moderator,
}

impl Role {
    pub fn to_str(&self) -> &'static str {
        match self {
            Role::Owner => "Owner",
            Role::Contributor => "Contributor",
            Role::Translator => "Translator",
            Role::Moderator => "Moderator",
        }
    }
}

impl From<Role> for sea_query::Value {
    fn from(val: Role) -> Self {
        val.to_str().into()
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = self.to_str();
        write!(f, "{}", value)
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
#[enum_def(table_name = "resource_role")]
pub struct UserResourceRole {
    pub resource_kind: Resource,
    pub resource_id: Uuid,
    pub resource_role: Role,
    pub user_id: Uuid,
}

/// User table representation
/// user is a protected word in postgresql so
/// we actually use the comahirle_user table
#[derive(Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "comhairle_user")]
pub struct User {
    pub id: Uuid,
    pub username: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub avatar_url: Option<String>,
    pub auth_type: UserAuthType,
    pub email: Option<String>,
    pub guest_code: Option<String>,
    pub email_verified: bool,
    pub organization_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Client IP captured at account creation. Internal/audit only — never
    /// serialized to API responses (see `skip_serializing`).
    #[serde(skip_serializing)]
    pub signup_ip: Option<String>,
    /// Client browser signature (User-Agent) captured at account creation.
    /// Internal/audit only — never serialized to API responses.
    #[serde(skip_serializing)]
    pub signup_user_agent: Option<String>,
}

const DEFAULT_COLUMNS: [UserIden; 13] = [
    UserIden::Id,
    UserIden::Username,
    UserIden::Password,
    UserIden::AuthType,
    UserIden::AvatarUrl,
    UserIden::Email,
    UserIden::GuestCode,
    UserIden::EmailVerified,
    UserIden::OrganizationId,
    UserIden::CreatedAt,
    UserIden::UpdatedAt,
    UserIden::SignupIp,
    UserIden::SignupUserAgent,
];

/// Create a user from a signup request
#[instrument(err(Debug), skip(db))]
pub async fn create_user(user: &SignupRequest, db: &PgPool) -> Result<User, ModelError> {
    let password = hash_pw(&user.password)?;
    let (sql, values) = Query::insert()
        .into_table(UserIden::Table)
        .columns([
            UserIden::AuthType,
            UserIden::Username,
            UserIden::Password,
            UserIden::AvatarUrl,
            UserIden::Email,
        ])
        .values([
            UserAuthType::EmailPassword.into(),
            user.username.clone().into(),
            password.into(),
            user.avatar_url.clone().into(),
            user.email.clone().into(),
        ])
        .unwrap()
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let user_result = sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await;

    // Check to see if the either a unique username or email has been
    // duplicated
    match user_result {
        Ok(user) => Ok(user),
        Err(sqlx::Error::Database(db_err)) => {
            let pg_err = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>();
            if pg_err.code() == "23505"
                && let Some(constraint) = pg_err.constraint()
                && constraint.contains("email")
            {
                return Err(UserError::DuplicateEmail(user.email.clone()).into());
            }
            Err(DataError::DatabaseError(sqlx::Error::Database(db_err)).into())
        }
        Err(e) => Err(DataError::DatabaseError(e).into()),
    }
}

/// Create an guest user
#[instrument(err(Debug), skip(db))]
pub async fn create_guest_user(db: &PgPool) -> Result<User, ModelError> {
    let mut retries = 5; // Retry up to 5 times to generate a unique username
    while retries > 0 {
        let sudo_random_code = gen_id();

        let (sql, values) = Query::insert()
            .into_table(UserIden::Table)
            .columns([UserIden::GuestCode, UserIden::AuthType])
            .values([sudo_random_code.into(), UserAuthType::Guest.into()])?
            .returning(Query::returning().columns(DEFAULT_COLUMNS))
            .build_sqlx(PostgresQueryBuilder);

        let user = sqlx::query_as_with::<_, User, _>(&sql, values)
            .fetch_one(db)
            .await;

        match user {
            Ok(user) => return Ok(user),
            Err(sqlx::Error::Database(db_err)) => {
                let pg_err = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>();
                if pg_err.code() == "23505" && pg_err.constraint() == Some("guest_code") {
                    // handle unique constraint violation on random guest_code collision.
                    retries -= 1;
                    continue;
                }
                return Err(DataError::DatabaseError(sqlx::Error::Database(db_err)).into());
            }
            Err(e) => return Err(DataError::DatabaseError(e).into()),
        }
    }
    Err(UserError::DuplicateGuestCode("too many retires".to_string()).into())
}

#[instrument(err(Debug), skip(db))]
pub async fn create_otp_user(user: &OtpSignupRequest, db: &PgPool) -> Result<User, ModelError> {
    let username = user
        .username
        .as_deref()
        .or_else(|| user.email.split_once("@").map(|(local, _)| local))
        .ok_or_else(|| ValidationError::BadRequest("Invalid email address".to_string()))?;

    let columns = vec![UserIden::AuthType, UserIden::Email, UserIden::Username];
    let values = vec![
        UserAuthType::Otp.into(),
        user.email.clone().into(),
        username.into(),
    ];

    let (sql, values) = Query::insert()
        .into_table(UserIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let user_result = sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await;

    match user_result {
        Ok(user) => return Ok(user),
        Err(sqlx::Error::Database(db_err)) => {
            let pg_err = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>();
            if pg_err.code() == "23505"
                && let Some(constraint) = pg_err.constraint()
                && constraint.contains("email")
            {
                return Err(UserError::DuplicateEmail(user.email.to_string()).into());
            }
            Err(DataError::DatabaseError(sqlx::Error::Database(db_err)).into())
        }
        Err(e) => Err(DataError::DatabaseError(e).into()),
    }
}

/// Gets a user by email if they exist or creates a new otp user from email if
/// none found
#[instrument(err(Debug), skip(db))]
pub async fn get_or_create_user_by_email(
    db: &PgPool,
    email: &str,
    client_ip: &str,
    user_agent: Option<&str>,
) -> Result<User, ModelError> {
    if let Ok(existing) = get_user_by_email(email, db).await {
        return Ok(existing);
    }

    let new_user = create_otp_user(
        &OtpSignupRequest {
            email: email.to_string(),
            username: None,
        },
        db,
    )
    .await?;

    // Best-effort: record the signup IP and browser signature for the
    // freshly created account.
    if let Err(error) = set_signup_metadata(&new_user.id, client_ip, user_agent, db).await {
        warn!(
            "Failed to record signup metadata for user {}: {error}",
            new_user.id
        );
    }

    Ok(new_user)
}

pub fn organization_admin_username(email: &str) -> String {
    let local_part = email
        .split('@')
        .next()
        .unwrap_or("org_admin")
        .chars()
        .filter(|character| {
            character.is_ascii_alphanumeric() || *character == '_' || *character == '-'
        })
        .collect::<String>();

    let base = if local_part.is_empty() {
        "org_admin".to_string()
    } else {
        local_part.to_lowercase()
    };

    format!("{}_{}", base, gen_id())
}

pub fn organization_admin_temporary_password() -> String {
    format!("TempAdmin#{}Aa1", gen_id())
}

/// Record the client IP and browser signature (User-Agent) for a freshly
/// created user in a single update.
///
/// Stored purely for internal/audit purposes; neither value is serialized back
/// out over the API (both fields are `skip_serializing`). A `None` user agent
/// leaves the column NULL.
#[instrument(err(Debug), skip(db))]
pub async fn set_signup_metadata(
    user_id: &Uuid,
    ip: &str,
    user_agent: Option<&str>,
    db: &PgPool,
) -> Result<(), ModelError> {
    let (sql, values) = Query::update()
        .table(UserIden::Table)
        .value(UserIden::SignupIp, ip)
        .value(UserIden::SignupUserAgent, user_agent)
        .and_where(Expr::col(UserIden::Id).eq(user_id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_with(&sql, values).execute(db).await?;
    Ok(())
}

/// Return a user by ID
#[instrument(err(Debug), skip(db))]
pub async fn get_user_by_id(id: &Uuid, db: &PgPool) -> Result<User, UserError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| UserError::NoUserFoundForId(id.to_owned()))?;
    Ok(user)
}

/// Return a user by email
#[instrument(err(Debug), skip(db))]
pub async fn get_user_by_email(email: &str, db: &PgPool) -> Result<User, UserError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Email).ilike(email))
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| UserError::NoUserFoundForEmail(email.to_owned()))?;
    Ok(user)
}

#[instrument(err(Debug), skip(db))]
pub async fn get_user_resource_roles(
    resource_kind: Resource,
    resource_id: &Uuid,
    resource_roles: &[Role],
    user_id: &Uuid,
    db: &PgPool,
) -> Result<Vec<UserResourceRole>, ModelError> {
    let (sql, values) = Query::select()
        .columns([
            UserResourceRoleIden::ResourceKind,
            UserResourceRoleIden::ResourceId,
            UserResourceRoleIden::ResourceRole,
            UserResourceRoleIden::UserId,
        ])
        .from(UserResourceRoleIden::Table)
        .and_where(Expr::col(UserResourceRoleIden::ResourceKind).eq(resource_kind.to_str()))
        .and_where(Expr::col(UserResourceRoleIden::ResourceId).eq(resource_id.to_owned()))
        .and_where(
            Expr::col(UserResourceRoleIden::ResourceRole).in_tuples(
                resource_roles
                    .iter()
                    .map(|role| role.to_str())
                    .collect::<Vec<_>>(),
            ),
        )
        .and_where(Expr::col(UserResourceRoleIden::UserId).eq(user_id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with::<_, UserResourceRole, _>(&sql, values)
        .fetch_all(db)
        .await
        .map_err(|e| DataError::DatabaseError(e).into())
}

#[instrument(err(Debug), skip(db))]
pub async fn user_has_resource_role(
    resource_kind: Resource,
    resource_id: &Uuid,
    resource_roles: &[Role],
    user_id: &Uuid,
    db: &PgPool,
) -> Result<bool, ModelError> {
    let result =
        get_user_resource_roles(resource_kind, resource_id, resource_roles, user_id, db).await?;

    if result.is_empty() {
        return Ok(false);
    }
    Ok(true)
}

#[instrument(err(Debug), skip(db))]
pub async fn add_user_resource_role(
    resource_kind: Resource,
    resource_id: &Uuid,
    resource_role: Role,
    user_id: &Uuid,
    db: &PgPool,
) -> Result<(), ModelError> {
    let (sql, values) = Query::insert()
        .columns([
            UserResourceRoleIden::ResourceKind,
            UserResourceRoleIden::ResourceId,
            UserResourceRoleIden::ResourceRole,
            UserResourceRoleIden::UserId,
        ])
        .values_panic([
            resource_kind.into(),
            (*resource_id).into(),
            resource_role.into(),
            (*user_id).into(),
        ])
        .into_table(UserResourceRoleIden::Table)
        .build_sqlx(PostgresQueryBuilder);
    // TODO IF NOT EXISTS

    sqlx::query_with(&sql, values).execute(db).await?;
    Ok(())
}

/// Return a guest user by guest_code
#[instrument(err(Debug), skip(db))]
pub async fn get_guest_user_by_code(guest_code: &str, db: &PgPool) -> Result<User, UserError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::GuestCode).eq(guest_code))
        .and_where(Expr::col(UserIden::AuthType).eq(UserAuthType::Guest))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| UserError::NoUserFound)
}

/// Return a non-guest user by username. Guest users do not have unique constraint
/// on usernames so should be selected by `guest_code`.
#[instrument(err(Debug), skip(db))]
pub async fn get_user_by_username(username: &str, db: &PgPool) -> Result<User, UserError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Username).eq(username))
        .and_where(Expr::col(UserIden::AuthType).not().eq(UserAuthType::Guest))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| UserError::NoUserFound)
}

/// Return all users associated with an organization.
#[instrument(err(Debug), skip(db))]
pub async fn list_by_organization_id(
    organization_id: &Uuid,
    db: &PgPool,
) -> Result<Vec<User>, ModelError> {
    let (sql, values) = Query::select()
        .columns(DEFAULT_COLUMNS)
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::OrganizationId).eq(*organization_id))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_all(db)
        .await
        .map_err(|e| DataError::DatabaseError(e).into())
}

/// Set or clear the organization membership for a user.
#[instrument(err(Debug), skip(db))]
pub async fn set_user_organization_id(
    user_id: &Uuid,
    organization_id: Option<Uuid>,
    db: &PgPool,
) -> Result<User, ModelError> {
    let (sql, values) = Query::update()
        .table(UserIden::Table)
        .value(UserIden::OrganizationId, organization_id)
        .and_where(Expr::col(UserIden::Id).eq(*user_id))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| DataError::DatabaseError(e).into())
}

#[derive(Debug, Deserialize, Default, Serialize, JsonSchema)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email_verified: Option<bool>,
    pub organization_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct UpgradeAccountRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Update user details (username and/or password)
#[instrument(err(Debug), skip(db))]
pub async fn update_user(
    user_id: &Uuid,
    update_request: &UpdateUserRequest,
    db: &PgPool,
) -> Result<User, ModelError> {
    let mut query = Query::update();
    query.table(UserIden::Table);

    let mut has_updates = false;

    if let Some(username) = &update_request.username {
        query.value(UserIden::Username, username.clone());
        has_updates = true;
    }

    if let Some(password) = &update_request.password {
        validate_password_strength(password)?;
        let hashed_password = hash_pw(password)?;
        query.value(UserIden::Password, hashed_password);
        has_updates = true;
    }

    if let Some(email_verified) = &update_request.email_verified {
        query.value(UserIden::EmailVerified, *email_verified);
        has_updates = true;
    }

    if let Some(organization_id) = &update_request.organization_id {
        query.value(UserIden::OrganizationId, *organization_id);
        has_updates = true;
    }

    if !has_updates {
        return get_user_by_id(user_id, db).await.map_err(Into::into);
    }

    let (sql, values) = query
        .and_where(Expr::col(UserIden::Id).eq(user_id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await?;

    Ok(user)
}

/// Upgrade an anonymous account to email/password account
#[instrument(err(Debug), skip(db))]
pub async fn upgrade_account(
    user_id: &Uuid,
    upgrade_request: &UpgradeAccountRequest,
    db: &PgPool,
) -> Result<User, ModelError> {
    // First verify the user exists and is an anonymous account
    let current_user = get_user_by_id(user_id, db).await?;

    if current_user.auth_type != UserAuthType::Guest {
        return Err(UserError::WrongUserType.into());
    }

    validate_password_strength(&upgrade_request.password)?;
    let hashed_password = hash_pw(&upgrade_request.password)?;

    let (sql, values) = Query::update()
        .table(UserIden::Table)
        .values([
            (UserIden::Username, upgrade_request.username.clone().into()),
            (UserIden::Email, upgrade_request.email.clone().into()),
            (UserIden::Password, hashed_password.into()),
            (UserIden::AuthType, UserAuthType::EmailPassword.into()),
        ])
        .and_where(Expr::col(UserIden::Id).eq(user_id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let user_result = sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await;

    match user_result {
        Ok(user) => Ok(user),
        Err(sqlx::Error::Database(db_err)) => {
            let pg_err = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>();
            if pg_err.code() == "23505"
                && let Some(constraint) = pg_err.constraint()
                && constraint.contains("email")
            {
                return Err(UserError::DuplicateEmail(upgrade_request.email.clone()).into());
            }
            Err(DataError::DatabaseError(sqlx::Error::Database(db_err)).into())
        }
        Err(e) => Err(DataError::DatabaseError(e).into()),
    }
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UserFilterOptions {
    is_admin: Option<bool>,
}

impl UserFilterOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(is_admin) = self.is_admin {
            let admin_subquery = Query::select()
                .expr(Expr::val(1))
                .from(ResourcePermissionIden::Table)
                .and_where(
                    Expr::col((
                        ResourcePermissionIden::Table,
                        ResourcePermissionIden::UserId,
                    ))
                    .equals((UserIden::Table, UserIden::Id)),
                )
                .and_where(
                    Expr::col((
                        ResourcePermissionIden::Table,
                        ResourcePermissionIden::ResourceType,
                    ))
                    .eq(PermissionResourceType::System.as_ref()),
                )
                .and_where(
                    Expr::col((
                        ResourcePermissionIden::Table,
                        ResourcePermissionIden::RoleName,
                    ))
                    .eq(PermissionRole::Admin.as_ref()),
                )
                .to_owned();

            query = if is_admin {
                query.and_where(Expr::exists(admin_subquery)).to_owned()
            } else {
                query
                    .and_where(Expr::exists(admin_subquery).not())
                    .to_owned()
            }
        }

        query
    }
}

#[derive(Deserialize, Debug, JsonSchema)]
pub struct UserOrderOptions {
    username: Option<Order>,
    created_at: Option<Order>,
}

impl UserOrderOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(order) = &self.username {
            query = query
                .order_by((UserIden::Table, UserIden::Username), order.into())
                .to_owned();
        }
        if let Some(order) = &self.created_at {
            query = query
                .order_by((UserIden::Table, UserIden::CreatedAt), order.into())
                .to_owned();
        }

        query
    }
}

#[instrument(err(Debug), skip(db))]
pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    filter_options: UserFilterOptions,
    order_options: UserOrderOptions,
) -> Result<PaginatedResults<User>, ModelError> {
    let query = Query::select().from(UserIden::Table).to_owned();

    let query = filter_options.apply(query);
    let query = order_options.apply(query);

    let users = page_options.fetch_paginated_results(db, query).await?;

    Ok(users)
}
