use std::fmt;

use crate::{
    error::ComhairleError,
    routes::auth::{hash_pw, SignupRequest},
    tools::id::gen_id,
};
use schemars::JsonSchema;
use sea_query::{enum_def, extension::postgres::PgExpr, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};
use uuid::Uuid;

/// Defines the type of authentication has been used to create
/// The user
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
pub enum UserAuthType {
    #[sqlx(rename = "annon")]
    Annon,
    #[sqlx(rename = "email_password")]
    EmailPassword,
    #[sqlx(rename = "scot_account")]
    ScotAccount,
}

impl Into<sea_query::Value> for UserAuthType {
    fn into(self) -> sea_query::Value {
        sea_query::Value::String(Some(Box::new(self.to_string())))
    }
}

impl fmt::Display for UserAuthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            UserAuthType::Annon => "annon",
            UserAuthType::EmailPassword => "email_password",
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

impl Into<sea_query::Value> for Resource {
    fn into(self) -> sea_query::Value {
        self.to_str().into()
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

impl Into<sea_query::Value> for Role {
    fn into(self) -> sea_query::Value {
        self.to_str().into()
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
    pub email_verified: bool,
}

/// Create a user from a signup request
pub async fn create_user(user: &SignupRequest, db: &PgPool) -> Result<User, ComhairleError> {
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
        .returning(Query::returning().columns([
            UserIden::Id,
            UserIden::Username,
            UserIden::Password,
            UserIden::AuthType,
            UserIden::AvatarUrl,
            UserIden::Email,
            UserIden::EmailVerified,
        ]))
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
            if pg_err.code() == "23505" {
                if let Some(constraint) = pg_err.constraint() {
                    if constraint.contains("username") {
                        return Err(ComhairleError::DuplicateUsername(user.username.clone()));
                    } else if constraint.contains("email") {
                        return Err(ComhairleError::DuplicateEmail(user.email.clone()));
                    }
                }
            }
            Err(ComhairleError::DatabaseError(sqlx::Error::Database(db_err)))
        }
        Err(e) => Err(ComhairleError::DatabaseError(e)),
    }
}

/// Create an annon user
pub async fn create_annon_user(db: &PgPool) -> Result<User, ComhairleError> {
    let mut retries = 5; // Retry up to 5 times to generate a unique username
    while retries > 0 {
        let sudo_random_name = gen_id();

        let (sql, values) = Query::insert()
            .into_table(UserIden::Table)
            .columns([UserIden::Username, UserIden::AuthType])
            .values([sudo_random_name.into(), UserAuthType::Annon.into()])
            .unwrap()
            .returning(Query::returning().columns([
                UserIden::AuthType,
                UserIden::Id,
                UserIden::Username,
                UserIden::Password,
                UserIden::AvatarUrl,
                UserIden::Email,
            ]))
            .build_sqlx(PostgresQueryBuilder);

        let user = sqlx::query_as_with::<_, User, _>(&sql, values)
            .fetch_one(db)
            .await;

        match user {
            Ok(user) => return Ok(user),
            Err(sqlx::Error::Database(db_err)) => {
                let pg_err = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>();
                if pg_err.code() == "23505" && pg_err.constraint() == Some("username") {
                    // handle unique constraint violation on random username collision.
                    retries -= 1;
                    continue;
                }
                return Err(ComhairleError::DatabaseError(sqlx::Error::Database(db_err)));
            }
            Err(e) => return Err(ComhairleError::DatabaseError(e)),
        }
    }
    Err(ComhairleError::DuplicateUsername(
        "too many retires".to_string(),
    ))
}

/// Return a user by ID
pub async fn get_user_by_id(id: &Uuid, db: &PgPool) -> Result<User, ComhairleError> {
    let (sql, values) = Query::select()
        .columns([
            UserIden::Id,
            UserIden::Username,
            UserIden::Password,
            UserIden::AvatarUrl,
            UserIden::AuthType,
            UserIden::Email,
            UserIden::EmailVerified,
        ])
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::NoUserFoundForId(id.to_owned()))?;
    Ok(user)
}

/// Return a user by email
pub async fn get_user_by_email(email: &str, db: &PgPool) -> Result<User, ComhairleError> {
    let (sql, values) = Query::select()
        .columns([
            UserIden::Id,
            UserIden::Username,
            UserIden::Password,
            UserIden::AvatarUrl,
            UserIden::AuthType,
            UserIden::Email,
            UserIden::EmailVerified,
        ])
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Email).ilike(email))
        .build_sqlx(PostgresQueryBuilder);

    let user = sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::NoUserFoundForEmail(email.to_owned()))?;
    Ok(user)
}

pub async fn get_user_resource_roles(
    resource_kind: Resource,
    resource_id: &Uuid,
    resource_roles: &[Role],
    user_id: &Uuid,
    db: &PgPool,
) -> Result<Vec<UserResourceRole>, ComhairleError> {
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
        .map_err(|e| ComhairleError::DatabaseError(e))
}

pub async fn user_has_resource_role(
    resource_kind: Resource,
    resource_id: &Uuid,
    resource_roles: &[Role],
    user_id: &Uuid,
    db: &PgPool,
) -> Result<bool, ComhairleError> {
    let result =
        get_user_resource_roles(resource_kind, resource_id, resource_roles, user_id, db).await?;

    if result.is_empty() {
        return Ok(false);
    }
    Ok(true)
}

pub async fn add_user_resource_role(
    resource_kind: Resource,
    resource_id: &Uuid,
    resource_role: Role,
    user_id: &Uuid,
    db: &PgPool,
) -> Result<(), ComhairleError> {
    let (sql, values) = Query::insert()
        .columns([
            UserResourceRoleIden::ResourceKind,
            UserResourceRoleIden::ResourceId,
            UserResourceRoleIden::ResourceRole,
            UserResourceRoleIden::UserId,
        ])
        .values_panic([
            resource_kind.into(),
            resource_id.clone().into(),
            resource_role.into(),
            user_id.clone().into(),
        ])
        .into_table(UserResourceRoleIden::Table)
        .build_sqlx(PostgresQueryBuilder);
    // TODO IF NOT EXISTS

    sqlx::query_with(&sql, values).execute(db).await?;
    Ok(())
}

/// Return a user by username
pub async fn get_user_by_username(username: &str, db: &PgPool) -> Result<User, ComhairleError> {
    let (sql, values) = Query::select()
        .columns([
            UserIden::Id,
            UserIden::Username,
            UserIden::Password,
            UserIden::AvatarUrl,
            UserIden::AuthType,
            UserIden::Email,
            UserIden::EmailVerified,
        ])
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Username).eq(username))
        .build_sqlx(PostgresQueryBuilder);

    sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|_| ComhairleError::NoUserFound)
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub password: Option<String>,
    pub verified: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct UpgradeAccountRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Update user details (username and/or password)
pub async fn update_user(
    user_id: &Uuid,
    update_request: &UpdateUserRequest,
    db: &PgPool,
) -> Result<User, ComhairleError> {
    let mut query = Query::update();
    query.table(UserIden::Table);

    let mut has_updates = false;

    if let Some(username) = &update_request.username {
        query.value(UserIden::Username, username.clone());
        has_updates = true;
    }

    if let Some(password) = &update_request.password {
        let hashed_password = hash_pw(password)?;
        query.value(UserIden::Password, hashed_password);
        has_updates = true;
    }
    if let Some(verified) = &update_request.verified {
        query.value(UserIden::EmailVerified, *verified);
        has_updates = true;
    }

    if !has_updates {
        return get_user_by_id(user_id, db).await;
    }

    let (sql, values) = query
        .and_where(Expr::col(UserIden::Id).eq(user_id.to_owned()))
        .returning(Query::returning().columns([
            UserIden::Id,
            UserIden::Username,
            UserIden::Password,
            UserIden::AvatarUrl,
            UserIden::AuthType,
            UserIden::Email,
            UserIden::EmailVerified,
        ]))
        .build_sqlx(PostgresQueryBuilder);

    let user_result = sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await;

    match user_result {
        Ok(user) => Ok(user),
        Err(sqlx::Error::Database(db_err)) => {
            let pg_err = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>();
            if pg_err.code() == "23505" {
                if let Some(constraint) = pg_err.constraint() {
                    if constraint.contains("username") {
                        return Err(ComhairleError::DuplicateUsername(
                            update_request.username.clone().unwrap_or_default(),
                        ));
                    }
                }
            }
            Err(ComhairleError::DatabaseError(sqlx::Error::Database(db_err)))
        }
        Err(e) => Err(ComhairleError::DatabaseError(e)),
    }
}

/// Upgrade an anonymous account to email/password account
pub async fn upgrade_account(
    user_id: &Uuid,
    upgrade_request: &UpgradeAccountRequest,
    db: &PgPool,
) -> Result<User, ComhairleError> {
    // First verify the user exists and is an anonymous account
    let current_user = get_user_by_id(user_id, db).await?;

    if current_user.auth_type != UserAuthType::Annon {
        return Err(ComhairleError::WrongUserType);
    }

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
        .returning(Query::returning().columns([
            UserIden::Id,
            UserIden::Username,
            UserIden::Password,
            UserIden::AvatarUrl,
            UserIden::AuthType,
            UserIden::Email,
        ]))
        .build_sqlx(PostgresQueryBuilder);

    let user_result = sqlx::query_as_with::<_, User, _>(&sql, values)
        .fetch_one(db)
        .await;

    match user_result {
        Ok(user) => Ok(user),
        Err(sqlx::Error::Database(db_err)) => {
            let pg_err = db_err.downcast_ref::<sqlx::postgres::PgDatabaseError>();
            if pg_err.code() == "23505" {
                if let Some(constraint) = pg_err.constraint() {
                    if constraint.contains("username") {
                        return Err(ComhairleError::DuplicateUsername(
                            upgrade_request.username.clone(),
                        ));
                    } else if constraint.contains("email") {
                        return Err(ComhairleError::DuplicateEmail(
                            upgrade_request.email.clone(),
                        ));
                    }
                }
            }
            Err(ComhairleError::DatabaseError(sqlx::Error::Database(db_err)))
        }
        Err(e) => Err(ComhairleError::DatabaseError(e)),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::users::{add_user_resource_role, user_has_resource_role, Resource, Role},
        setup_server,
        test_helpers::{test_config, test_state, UserSession},
    };
    use sqlx::PgPool;
    use std::error::Error;
    use std::sync::Arc;
    use uuid::Uuid;

    #[sqlx::test]
    fn user_has_resource_role_tests(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool.clone()).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let (status, conversation, _) = session
            .create_conversation(
                &app,
                serde_json::json! ({
                    "title" : "Test conversation",
                    "short_description" : "A test conversation",
                    "description" : "A longer description",
                    "image_url" : "http://someimage.png",
                    "tags" : ["one", "two", "three"],
                    "is_public" : false,
                    "is_invite_only" : false,
                    "slug" : "new_conversation"
                }),
            )
            .await?;
        assert_eq!(status, 201, "should be able to create a conversation");
        let conversation_id = Uuid::parse_str(conversation["id"].as_str().unwrap())?;

        let mut session = UserSession::new(
            "test_user".into(),
            "test_password".into(),
            "test.user@gmail.com".into(),
        );
        session.signup(&app).await?;

        add_user_resource_role(
            Resource::Conversation,
            &conversation_id,
            Role::Contributor,
            &session.id.unwrap(),
            &pool,
        )
        .await?;

        assert_eq!(
            user_has_resource_role(
                Resource::Conversation,
                &conversation_id,
                &[Role::Contributor],
                &session.id.unwrap(),
                &pool.clone(),
            )
            .await?,
            true,
            "true when user has role",
        );
        assert_eq!(
            user_has_resource_role(
                Resource::Conversation,
                &conversation_id,
                &[Role::Contributor],
                &Uuid::parse_str("5FDFC2CE-C7F5-43DB-AA1F-0A8698E76D2E").unwrap(),
                &pool.clone(),
            )
            .await?,
            false,
            "false when no user with that ID",
        );
        assert_eq!(
            user_has_resource_role(
                Resource::Conversation,
                &Uuid::parse_str("5FDFC2CE-C7F5-43DB-AA1F-0A8698E76D2E").unwrap(),
                &[Role::Contributor],
                &session.id.unwrap(),
                &pool.clone(),
            )
            .await?,
            false,
            "false when no conversation with that ID",
        );
        assert_eq!(
            user_has_resource_role(
                Resource::Conversation,
                &conversation_id,
                &[Role::Owner],
                &session.id.unwrap(),
                &pool.clone(),
            )
            .await?,
            false,
            "false when wrong role kind",
        );
        assert_eq!(
            user_has_resource_role(
                Resource::Conversation,
                &conversation_id,
                &[Role::Owner, Role::Contributor],
                &session.id.unwrap(),
                &pool.clone(),
            )
            .await?,
            true,
            "true when user could be multiple roles and has one",
        );

        add_user_resource_role(
            Resource::Conversation,
            &conversation_id,
            Role::Translator,
            &session.id.unwrap(),
            &pool,
        )
        .await?;

        assert_eq!(
            user_has_resource_role(
                Resource::Conversation,
                &conversation_id,
                &[Role::Translator],
                &session.id.unwrap(),
                &pool.clone(),
            )
            .await?,
            true,
            "true when user has multiple roles and one is required",
        );

        Ok(())
    }
}
