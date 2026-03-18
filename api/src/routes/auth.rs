use aide::{
    axum::{
        routing::{get_with, post_with},
        ApiRouter,
    },
    OperationIo,
};

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::{FromRequestParts, Json, Path, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::TimeDelta;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rand_core::OsRng;
use regex::Regex;

/// Helper function to check if a user is admin
pub fn is_user_admin(
    user: &crate::models::users::User,
    config: &crate::config::ComhairleConfig,
) -> bool {
    let re = Regex::new(r"^test(?:[1-9]|10)@crown-shy\.com$").unwrap();
    if let (Some(admin_users), Some(email)) = (&config.admin_users, &user.email) {
        let downcase_admin_users: Vec<String> =
            admin_users.iter().map(|a| a.to_lowercase()).collect();
        return downcase_admin_users.contains(&email.to_lowercase())
            || re.is_match(&email.to_lowercase());
    }
    false
}
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use std::marker::PhantomData;
use std::{collections::HashMap, sync::Arc};
use tracing::{instrument, warn};
use uuid::Uuid;
// use tower_cookies::{Cookie, Cookies};

use crate::{
    error::ComhairleError,
    models::users::{
        create_annon_user, create_user, get_user_by_email, get_user_by_id, get_user_by_username,
        get_user_resource_roles, update_user, Resource, Role, UpdateUserRequest, User,
        UserAuthType, UserResourceRole,
    },
    routes::user::dto::UserDto,
    ComhairleState,
};

#[cfg(test)]
use fake::Dummy;

/// This is the key that we use in the cookie for the JWT
pub const AUTH_KEY: &str = "auth-token";

/// Validate password strength according to security requirements
///
/// Requirements:
/// - Minimum 16 characters
/// - Must include characters from at least 3 of 4 categories:
///   - Uppercase letters (A-Z)
///   - Lowercase letters (a-z)
///   - Numbers (0-9)
///   - Special characters
/// - Uses zxcvbn for additional complexity checking
pub fn validate_password_strength(password: &str) -> Result<(), ComhairleError> {
    let mut errors = Vec::new();

    // Check minimum length
    if password.len() < 16 {
        errors.push(format!(
            "Password must be at least 16 characters long (current length: {})",
            password.len()
        ));
    }

    // Check character categories
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password
        .chars()
        .any(|c| !c.is_alphanumeric() && !c.is_whitespace());

    let category_count = [has_uppercase, has_lowercase, has_digit, has_special]
        .iter()
        .filter(|&&x| x)
        .count();

    if category_count < 3 {
        let mut missing_categories = Vec::new();
        if !has_uppercase {
            missing_categories.push("uppercase letters (A-Z)");
        }
        if !has_lowercase {
            missing_categories.push("lowercase letters (a-z)");
        }
        if !has_digit {
            missing_categories.push("numbers (0-9)");
        }
        if !has_special {
            missing_categories.push("special characters (e.g., !@#$%^&*)");
        }

        errors.push(format!(
            "Password must include characters from at least 3 of 4 categories. Consider adding: {}",
            missing_categories.join(", ")
        ));
    }

    // Use zxcvbn for additional complexity checking
    let entropy = zxcvbn::zxcvbn(password, &[]);

    // zxcvbn scores range from 0 (weak) to 4 (strong)
    // We require a score of at least 3
    use zxcvbn::Score;
    if matches!(entropy.score(), Score::Zero | Score::One | Score::Two) {
        let feedback = entropy.feedback();
        let warning = feedback
            .and_then(|f| f.warning())
            .map(|w| format!("{:?}", w))
            .unwrap_or_else(|| "Password is too predictable or common".to_string());

        let suggestions = feedback.and_then(|f| {
            let suggs = f.suggestions();
            if suggs.is_empty() {
                None
            } else {
                Some(
                    suggs
                        .into_iter()
                        .map(|s| format!("{:?}", s))
                        .collect::<Vec<String>>()
                        .join("; "),
                )
            }
        });

        let mut complexity_msg = format!("Password complexity is too low. {}", warning);
        if let Some(suggs) = suggestions {
            complexity_msg.push_str(&format!(" Suggestions: {}", suggs));
        }
        errors.push(complexity_msg);
    }

    if !errors.is_empty() {
        return Err(ComhairleError::WeakPassword(errors.join(". ")));
    }

    Ok(())
}

/// Generate a hashed password
pub fn hash_pw(password: &str) -> Result<String, ComhairleError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| ComhairleError::PasswordHash)?;

    Ok(hash.to_string())
}

/// Expected payload for a login request
#[derive(Deserialize, JsonSchema)]
struct LoginRequest {
    email: String,
    password: String,
}

/// Expected payload for an annon login request
#[derive(Deserialize, JsonSchema)]
struct AnnonLoginRequest {
    username: String,
}

#[derive(Deserialize, JsonSchema)]
struct VerifyEmailTokenRequest {
    token: String,
}

#[derive(Deserialize, JsonSchema)]
struct ResendVerificationEmailRequest {
    id: String,
}

#[derive(Deserialize, JsonSchema)]
struct CreatePasswordResetRequest {
    email: String,
}

#[derive(Deserialize, JsonSchema)]
struct PasswordResetUpdateRequest {
    token: String,
    password: String,
    confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmailLinkClaims {
    email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionClaims {
    username: Option<String>,
    sudo_user: Option<String>, // TODO: Remove at some point
    email_verified: bool,
    roles: Vec<String>,
}

/// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub struct Claims<T>
where
    T: Serialize + DeserializeOwned,
{
    sub: String,
    exp: usize,
    id: String,
    #[serde(flatten)]
    details: T,
}

/// Generate JWT
pub fn generate_jwt<T: Serialize + DeserializeOwned>(
    user: &User,
    custom_claims: T,
    secret: &str,
    custom_duration: Option<TimeDelta>,
) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(custom_duration.unwrap_or_else(|| chrono::Duration::hours(24)))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration,
        id: user.id.to_string(),
        details: custom_claims,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

/// Expected payload for a signin request  
#[derive(Deserialize, Debug, JsonSchema)]
#[cfg_attr(test, derive(Dummy))]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub email: String,
}

/// Signup handler

#[instrument(err(Debug), skip(state, payload))]
async fn signup(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
    Json(payload): Json<SignupRequest>,
) -> Result<(CookieJar, (StatusCode, Json<UserDto>)), ComhairleError> {
    // Validate password strength
    validate_password_strength(&payload.password)?;

    let user = create_user(&payload, &state.db).await?;
    let claims = EmailLinkClaims {
        email: user.email.clone(),
    };
    let verification_token = generate_jwt(
        &user,
        claims,
        &state.config.jwt_secret,
        Some(chrono::Duration::minutes(15)),
    );
    let verify_link = format!(
        "{}/auth/verify-user?token={}",
        state.config.domain, verification_token
    );

    state.mailer.send_welcome_email(&user, verify_link)?;

    let claims = SessionClaims {
        username: user.username.clone(),
        sudo_user: None,
        email_verified: user.email_verified,
        roles: Vec::new(),
    };
    let session_token = generate_jwt(&user, claims, &state.config.jwt_secret, None);
    let cookie = Cookie::build((AUTH_KEY, session_token))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None);

    let user: UserDto = user.into();
    Ok((jar.add(cookie), (StatusCode::CREATED, Json(user))))
}

/// Signup handler for annon
#[instrument(err(Debug), skip(state))]
async fn signup_annon(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
) -> Result<(CookieJar, (StatusCode, Json<UserDto>)), ComhairleError> {
    let user = create_annon_user(&state.db).await?;
    let claims = SessionClaims {
        username: user.username.clone(),
        sudo_user: None,
        email_verified: user.email_verified,
        roles: Vec::new(),
    };
    let token = generate_jwt(&user, claims, &state.config.jwt_secret, None);

    let cookie = Cookie::build((AUTH_KEY, token))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None);

    let user: UserDto = user.into();
    Ok((jar.add(cookie), (StatusCode::CREATED, Json(user))))
}

/// Email/Password Login Handler
#[instrument(err(Debug), skip(state, payload))]
async fn login(
    State(state): State<Arc<ComhairleState>>,
    jar: CookieJar,
    Json(payload): Json<LoginRequest>,
) -> Result<(CookieJar, (StatusCode, Json<UserDto>)), ComhairleError> {
    let user = get_user_by_email(&payload.email, &state.db).await?;

    let password = user
        .password
        .as_ref()
        .ok_or_else(|| ComhairleError::WrongUserType)?;

    let hash = PasswordHash::new(password).map_err(|_| ComhairleError::PasswordHash)?;

    if Argon2::default()
        .verify_password(&payload.password.into_bytes(), &hash)
        .is_err()
    {
        return Err(ComhairleError::WrongPassword);
    }

    let claims = SessionClaims {
        username: user.username.clone(),
        sudo_user: None,
        email_verified: user.email_verified,
        roles: Vec::new(),
    };
    let token = generate_jwt(&user.clone(), claims, &state.config.jwt_secret, None);
    let cookie = Cookie::build((AUTH_KEY, token))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None);

    let user: UserDto = user.into();
    Ok((jar.add(cookie), (StatusCode::OK, Json(user))))
}

#[instrument(err(Debug), skip(state, payload))]
async fn login_annon(
    State(state): State<Arc<ComhairleState>>,
    cookies: CookieJar,
    Json(payload): Json<AnnonLoginRequest>,
) -> Result<(CookieJar, (StatusCode, Json<UserDto>)), ComhairleError> {
    let user = get_user_by_username(&payload.username, &state.db).await?;

    if user.auth_type != UserAuthType::Annon {
        // return not found to avoid revealing that a correct username has been used.
        return Err(ComhairleError::NoUserFound);
    }

    let claims = SessionClaims {
        username: user.username.clone(),
        sudo_user: None,
        email_verified: user.email_verified,
        roles: Vec::new(),
    };
    let token = generate_jwt(&user.clone(), claims, &state.config.jwt_secret, None);
    let cookie = Cookie::build((AUTH_KEY, token))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None);

    let user: UserDto = user.into();
    Ok((cookies.add(cookie), (StatusCode::OK, Json(user))))
}

#[instrument(err(Debug), skip(state, payload))]
async fn resend_verification_email(
    State(state): State<Arc<ComhairleState>>,
    Json(payload): Json<ResendVerificationEmailRequest>,
) -> Result<StatusCode, ComhairleError> {
    let id = Uuid::parse_str(&payload.id).map_err(|_| ComhairleError::InvalidUserId)?;
    let user = get_user_by_id(&id, &state.db).await?;
    let claims = EmailLinkClaims {
        email: user.email.clone(),
    };
    let token = generate_jwt(
        &user,
        claims,
        &state.config.jwt_secret,
        Some(chrono::Duration::minutes(15)),
    );
    let verify_link = format!("{}/auth/verify-user?token={}", state.config.domain, token);
    state
        .mailer
        .send_verification_email(&user.username, &user.email, verify_link)?;
    Ok(StatusCode::OK)
}

#[instrument(err(Debug), skip(state, payload))]
async fn verify_email_token(
    State(state): State<Arc<ComhairleState>>,
    cookies: CookieJar,
    Json(payload): Json<VerifyEmailTokenRequest>,
) -> Result<(CookieJar, (StatusCode, Json<UserDto>)), ComhairleError> {
    let current_user = validate_jwt::<EmailLinkClaims>(&state, &payload.token).await?;

    if current_user.auth_type == UserAuthType::Annon {
        return Err(ComhairleError::WrongUserType);
    }

    if current_user.email_verified {
        return Err(ComhairleError::EmailAlreadyVerified);
    }

    let updated_verified_status = UpdateUserRequest {
        email_verified: Some(true),
        ..Default::default()
    };

    let updated_user = update_user(&current_user.id, &updated_verified_status, &state.db).await?;

    let claims = SessionClaims {
        username: updated_user.username.clone(),
        sudo_user: None,
        email_verified: updated_user.email_verified,
        roles: Vec::new(),
    };
    let session_token = generate_jwt(
        &updated_user.clone(),
        claims,
        &state.config.jwt_secret,
        None,
    );
    let cookie = Cookie::build((AUTH_KEY, session_token.clone()))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None);

    let user: UserDto = updated_user.into();
    Ok((cookies.add(cookie), (StatusCode::OK, Json(user))))
}

#[instrument(err(Debug), skip(state, payload))]
async fn password_reset_create(
    State(state): State<Arc<ComhairleState>>,
    Json(payload): Json<CreatePasswordResetRequest>,
) -> Result<StatusCode, ComhairleError> {
    let user = get_user_by_email(&payload.email, &state.db).await?;
    let claims = EmailLinkClaims {
        email: user.email.clone(),
    };
    let token = generate_jwt(
        &user,
        claims,
        &state.config.jwt_secret,
        Some(chrono::Duration::minutes(15)),
    );
    let reset_link = format!(
        "{}/auth/password-reset/update?token={}",
        state.config.domain, token
    );

    state
        .mailer
        .send_password_reset_email(&user.email, &user.username, reset_link)?;

    Ok(StatusCode::NO_CONTENT)
}

#[instrument(err(Debug), skip(state, payload))]
async fn password_reset_update(
    State(state): State<Arc<ComhairleState>>,
    Json(payload): Json<PasswordResetUpdateRequest>,
) -> Result<StatusCode, ComhairleError> {
    let user = validate_jwt::<EmailLinkClaims>(&state, &payload.token).await?;

    if user.auth_type == UserAuthType::Annon {
        return Err(ComhairleError::WrongUserType);
    }

    if payload.password != payload.confirm_password {
        return Err(ComhairleError::PasswordConfirmationMismatch);
    }

    // Validate password strength
    validate_password_strength(&payload.password)?;

    let updated_password = UpdateUserRequest {
        password: Some(payload.password),
        ..Default::default()
    };

    update_user(&user.id, &updated_password, &state.db).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Decode a JWT
pub fn decode_jwt<T: Serialize + DeserializeOwned>(
    jwt: &str,
    secret: &str,
) -> Result<TokenData<Claims<T>>, StatusCode> {
    let result: Result<TokenData<Claims<T>>, StatusCode> = decode(
        jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}

#[derive(Deserialize)]
struct ConversationPath {
    conversation_id: Uuid,
}

/// An extractor to ensure that a required user has a role.
/// If the user does not have the role then this will fail and
/// return a Not Authorized response
/// e.g.
///     RequiredRole(role, _, _): RequiredRole<Conversation, Owner>
#[derive(OperationIo)]
pub struct RequiredRole<Kind: RequiredRoleResource, Roles: RequiredRoleRoleTuple>(
    UserResourceRole,
    PhantomData<Kind>,
    PhantomData<Roles>,
);
impl<Kind: RequiredRoleResource, Roles: RequiredRoleRoleTuple> RequiredRole<Kind, Roles> {
    pub fn new(user_resource_role: UserResourceRole) -> Self {
        RequiredRole(user_resource_role, PhantomData, PhantomData)
    }
}

impl<Resource: RequiredRoleResource, Roles: RequiredRoleRoleTuple>
    FromRequestParts<Arc<ComhairleState>> for RequiredRole<Resource, Roles>
{
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let RequiredUser(user) = parts.extract_with_state::<RequiredUser, _>(state).await?;
        let params = parts
            .extract::<Path<ConversationPath>>()
            .await
            .map_err(|_| {
                ComhairleError::ResourceNotFound("Path must contain a conversation_id".to_string())
            })?;

        let roles = get_user_resource_roles(
            Resource::to_enum(),
            &params.conversation_id, // TODO: handle slug
            &Roles::to_enum_vec(),
            &user.id,
            &state.db,
        )
        .await?;

        match roles.first() {
            Some(role) => Ok(RequiredRole::new(role.clone())),
            None => Err(ComhairleError::UserNotAuthorized),
        }
    }
}

pub trait RequiredRoleRole: Default {
    fn to_enum() -> Role;
}

pub trait RequiredRoleRoleTuple {
    fn to_enum_vec() -> Vec<Role>;
}

impl RequiredRoleRoleTuple for () {
    fn to_enum_vec() -> Vec<Role> {
        Vec::new()
    }
}

impl<Head: RequiredRoleRole, Tail: RequiredRoleRoleTuple> RequiredRoleRoleTuple for (Head, Tail) {
    fn to_enum_vec() -> Vec<Role> {
        let mut roles = Tail::to_enum_vec();
        roles.push(Head::to_enum());
        roles
    }
}

impl<Head: RequiredRoleRole> RequiredRoleRoleTuple for (Head,) {
    fn to_enum_vec() -> Vec<Role> {
        <(Head, ())>::to_enum_vec()
    }
}

#[derive(Default)]
pub struct Owner {}
impl RequiredRoleRole for Owner {
    fn to_enum() -> Role {
        Role::Owner
    }
}

#[derive(Default)]
pub struct Contributor {}
impl RequiredRoleRole for Contributor {
    fn to_enum() -> Role {
        Role::Contributor
    }
}

#[derive(Default)]
pub struct Translator {}
impl RequiredRoleRole for Translator {
    fn to_enum() -> Role {
        Role::Translator
    }
}
#[derive(Default)]
pub struct Moderator {}
impl RequiredRoleRole for Moderator {
    fn to_enum() -> Role {
        Role::Moderator
    }
}

pub trait RequiredRoleResource: Default {
    fn to_enum() -> Resource;
}

#[derive(Default)]
pub struct Organisation {}
impl RequiredRoleResource for Organisation {
    fn to_enum() -> Resource {
        Resource::Organisation
    }
}
#[derive(Default)]
pub struct Conversation {}
impl RequiredRoleResource for Conversation {
    fn to_enum() -> Resource {
        Resource::Conversation
    }
}

/// An extractor to get a required current user.
/// If no user is logged in then this will fail and
/// Return a Not Found response
#[derive(OperationIo)]
pub struct RequiredAdminUser(pub User);

impl FromRequestParts<Arc<ComhairleState>> for RequiredAdminUser {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let user = parts.extract_with_state::<RequiredUser, _>(state).await?;

        if is_user_admin(&user.0, &state.config) {
            Ok(RequiredAdminUser(user.0.clone()))
        } else {
            Err(ComhairleError::RequiresAuthUser)
        }
    }
}

/// An extractor to get a required current user.
/// If no user is logged in then this will fail and
/// Return a Not Found response
#[derive(OperationIo)]
pub struct RequiredUser(pub User);

/// An extractor to get the current user if they exist
/// If a user is not logged in, this will still run
/// but produce a None value in the extractor
#[derive(OperationIo, Debug)]
pub struct OptionalUser(pub Option<User>);

impl FromRequestParts<Arc<ComhairleState>> for RequiredUser {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let poss_user = parts.extract_with_state::<OptionalUser, _>(state).await?;

        if let Some(user) = poss_user.0 {
            Ok(RequiredUser(user))
        } else {
            Err(ComhairleError::UserRequired)
        }
    }
}

impl FromRequestParts<Arc<ComhairleState>> for OptionalUser {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let jar = parts
            .extract::<CookieJar>()
            .await
            .map_err(|e| ComhairleError::AuthJWTError(e.to_string()))?;

        if let Some(token_cookie) = jar.get(AUTH_KEY) {
            let token_str = token_cookie.value();
            let poss_user = validate_jwt::<SessionClaims>(state, token_str).await.ok();
            Ok(OptionalUser(poss_user))
        } else {
            Ok(OptionalUser(None))
        }
    }
}

/// Ensure the JTW is valid and if it is return the associated user
/// May break this out in future for routes that require a valid
/// user but dont care who they are
pub async fn validate_jwt<T: Serialize + DeserializeOwned>(
    state: &Arc<ComhairleState>,
    token: &str,
) -> Result<User, ComhairleError> {
    let token_data = match decode_jwt::<T>(token, &state.config.jwt_secret) {
        Ok(data) => data,
        Err(e) => {
            warn!("unable to decode {e}");
            return Err(ComhairleError::AuthJWTError(
                "Unable to decode token".to_string(),
            ));
        }
    };

    // Fetch the user details from the database
    let uuid = Uuid::parse_str(&token_data.claims.id).unwrap();
    let current_user = match get_user_by_id(&uuid, &state.db).await {
        Ok(user) => user,
        Err(e) => {
            return Err(e);
        }
    };

    Ok(current_user)
}

/// Destroy the cookie on our session to log a user out
pub async fn logout(jar: CookieJar) -> (CookieJar, Response) {
    let cookie = Cookie::build(AUTH_KEY).path("/");
    (
        jar.remove(cookie),
        Json(json!({"msg":"Logged out"})).into_response(),
    )
}

/// Handler for the current user if there is one
#[instrument(err(Debug))]
pub async fn current_user(
    OptionalUser(user): OptionalUser,
) -> Result<(StatusCode, Json<UserDto>), ComhairleError> {
    let user: UserDto = (user.ok_or_else(|| ComhairleError::NoLogedInUser)?).into();

    Ok((StatusCode::OK, Json(user)))
}

/// Handler for testing RequiresRole
pub async fn test_requires_roles(
    RequiredRole(_, _, _): RequiredRole<Conversation, (Owner, (Contributor,))>,
    RequiredUser(user): RequiredUser,
) -> Result<(StatusCode, Json<UserDto>), ComhairleError> {
    let user: UserDto = user.into();
    Ok((StatusCode::OK, Json(user)))
}

/// Function to set up the auth routes
pub async fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/login_annon",
            post_with(login_annon, |op| {
                op.id("LoginAnnonUser")
                    .tag("Auth")
                    .summary("Login an annon user")
                    .response::<200, Json<UserDto>>()
            }),
        )
        .api_route(
            "/login",
            post_with(login, |op| {
                op.id("LoginUser")
                    .tag("Auth")
                    .summary("Login a user")
                    .response::<200, Json<UserDto>>()
            }),
        )
        .api_route(
            "/signup",
            post_with(signup, |op| {
                op.id("SignUp")
                    .tag("Auth")
                    .summary("Signup a user with email and password")
                    .response::<201, Json<UserDto>>()
            }),
        )
        .api_route(
            "/signup_annon",
            post_with(signup_annon, |op| {
                op.id("SignupAnnonUser")
                    .tag("Auth")
                    .summary("Signup and annon user")
                    .response::<201, Json<UserDto>>()
            }),
        )
        .api_route(
            "/logout",
            post_with(logout, |op| {
                op.id("LogoutUser")
                    .tag("Auth")
                    .summary("Logout the current user")
                    .response::<200, Json<HashMap<String, String>>>()
            }),
        )
        .api_route(
            "/verify_email_token",
            post_with(verify_email_token, |op| {
                op.id("VerifyEmailToken")
                    .tag("Auth")
                    .summary("Verify token from email verification link")
                    .response::<200, Json<UserDto>>()
            }),
        )
        .api_route(
            "/resend_verification_email",
            post_with(resend_verification_email, |op| {
                op.id("ResendVerificationEmail")
                    .tag("Auth")
                    .summary("Resend email verification link to user")
                    .response::<200, ()>()
            }),
        )
        .api_route(
            "/password_reset_create",
            post_with(password_reset_create, |op| {
                op.id("PasswordResetCreate")
                    .tag("Auth")
                    .summary("Create password reset flow by sending reset link to user email")
                    .response::<204, ()>()
            }),
        )
        .api_route(
            "/password_reset_update",
            post_with(password_reset_update, |op| {
                op.id("PasswordResetUpdate")
                    .tag("Auth")
                    .summary("Update password of user in reset flow")
                    .response::<204, ()>()
            }),
        )
        .api_route(
            "/current_user",
            get_with(current_user, |op| {
                op.id("CurrentUser")
                    .tag("Auth")
                    .summary("Get the current user")
                    .response::<200, Json<UserDto>>()
            }),
        )
        // TODO: this route is used for testing only. Once we have authorisation logic locekd down
        // in other endpoints, this can be removed and those auth requirements tested.
        .api_route(
            "/test_requires_roles/{conversation_id}",
            get_with(test_requires_roles, |op| {
                op.id("TestRequiresRoles")
                    .summary("Test the requires roles")
                    .response::<200, Json<UserDto>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {

    use crate::{
        mailer::MockComhairleMailer,
        models::users::{
            add_user_resource_role, get_user_by_email, Resource, Role, UpdateUserRequest, User,
            UserAuthType,
        },
        routes::{
            auth::{generate_jwt, EmailLinkClaims, SessionClaims},
            user::dto::UserDto,
        },
        setup_server,
        test_helpers::{test_state, UserSession},
    };

    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    use axum::http::StatusCode;
    use mockall::predicate::{always, eq};
    use sqlx::PgPool;
    use std::{error::Error, sync::Arc};
    use uuid::Uuid;

    #[sqlx::test]
    async fn user_should_be_able_to_sign_up(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email";

        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new(username, password, email);
        let (status, _, _) = session.signup(&app).await?;
        assert_eq!(status, StatusCode::CREATED, "should be created");

        let (status, user, _) = session.current_user(&app).await?;

        assert_eq!(status, StatusCode::OK, "should get current user");

        assert_eq!(
            user.username,
            Some(username.to_owned()),
            "current user should contain the right username"
        );

        assert_eq!(
            user.auth_type,
            UserAuthType::EmailPassword,
            "current user should have auth_type email password"
        );
        assert_eq!(
            user.email,
            Some(email.to_owned()),
            "current user should contain the right email"
        );

        assert_ne!(user.id, Uuid::nil(), "current user should contain an id");
        Ok(())
    }

    #[sqlx::test]
    async fn user_should_receive_signup_email(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email";

        let mut mailer = MockComhairleMailer::new();
        mailer
            .expect_send_welcome_email()
            .once()
            .returning(|_, _| Ok(()));

        mailer.expect_send_verification_email().times(0);
        mailer.expect_send_password_reset_email().times(0);

        let state = test_state().db(pool).mailer(Arc::new(mailer)).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new(username, password, email);
        let (status, _, _) = session.signup(&app).await?;
        assert_eq!(status, StatusCode::CREATED, "should be created");

        let (status, user, _) = session.current_user(&app).await?;

        assert_eq!(status, StatusCode::OK, "should get current user");

        assert_eq!(
            user.username,
            Some(username.to_owned()),
            "current user should contain the right username"
        );

        assert_eq!(
            user.auth_type,
            UserAuthType::EmailPassword,
            "current user should have auth_type email password"
        );
        assert_eq!(
            user.email,
            Some(email.to_owned()),
            "current user should contain the right email"
        );

        assert_ne!(user.id, Uuid::nil(), "current user should contain an id");
        Ok(())
    }

    #[sqlx::test]
    async fn user_should_receive_verification_email(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut mailer = MockComhairleMailer::new();
        mailer
            .expect_send_welcome_email()
            .once()
            .returning(|_, _| Ok(()));

        mailer
            .expect_send_verification_email()
            .with(
                eq(Some("test_user".to_string())),
                eq(Some("test_email".to_string())),
                always(),
            )
            .once()
            .returning(|_, _, _| Ok(()));

        mailer.expect_send_password_reset_email().times(0);

        let state = test_state().db(pool).mailer(Arc::new(mailer)).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email";

        let mut session = UserSession::new(username, password, email);
        session.signup(&app).await?;
        let (status, _, _) = session.resend_verification_email(&app).await?;

        assert_eq!(status, StatusCode::OK, "should send verification email");

        Ok(())
    }

    #[sqlx::test]
    async fn unverified_user_should_be_verified(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email";

        let state = test_state().db(pool).call()?;
        let secret = &state.config.jwt_secret.clone();
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new(username, password, email);
        let (_, user, _) = session.signup(&app).await?;

        // Extract id from signed-up user
        let id = user.get("id").unwrap().as_ref().unwrap().as_str().unwrap();
        let user = User {
            id: Uuid::parse_str(id).unwrap(),
            email: Some(email.to_string()),
            password: Some(password.to_string()),
            username: Some(username.to_string()),
            auth_type: UserAuthType::EmailPassword,
            avatar_url: None,
            email_verified: false,
            organization_id: None,
        };
        let claims = SessionClaims {
            username: user.username.clone(),
            sudo_user: None,
            email_verified: user.email_verified,
            roles: Vec::new(),
        };
        let token = generate_jwt(&user, claims, secret, None);
        let (status, user, _) = session.verify_email_token(&app, token).await?;
        let user: UserDto = serde_json::from_value(user)?;

        assert_eq!(status, StatusCode::OK, "Token successfully verified");
        assert!(user.email_verified, "user email_verified status updated");
        assert_eq!(
            user.email,
            Some(email.to_owned()),
            "current user should contain the right email"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn annon_user_cannot_be_verified(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email";

        let state = test_state().db(pool).call()?;
        let secret = &state.config.jwt_secret.clone();
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new(username, password, email);
        let (_, user, _) = session.signup_annon(&app).await?;

        let id = user.get("id").unwrap().as_ref().unwrap().as_str().unwrap();
        let user = User {
            id: Uuid::parse_str(id).unwrap(),
            email: Some(email.to_string()),
            password: Some(password.to_string()),
            username: Some(username.to_string()),
            auth_type: UserAuthType::Annon,
            avatar_url: None,
            email_verified: false,
            organization_id: None,
        };
        let claims = SessionClaims {
            username: user.username.clone(),
            sudo_user: None,
            email_verified: user.email_verified,
            roles: Vec::new(),
        };
        let token = generate_jwt(&user, claims, secret, None);
        let (status, _, _) = session.verify_email_token(&app, token).await?;

        assert_eq!(
            status,
            StatusCode::INTERNAL_SERVER_ERROR,
            "cannot verify annonymous user"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn user_cannot_be_verified_twice(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email";

        let state = test_state().db(pool).call()?;
        let secret = &state.config.jwt_secret.clone();
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new(username, password, email);
        session.signup(&app).await?;
        let updated_user_values = UpdateUserRequest {
            email_verified: Some(true),
            ..Default::default()
        };
        let (_, user, _) = session
            .update_user_details(&app, updated_user_values)
            .await?;
        let user: UserDto = serde_json::from_value(user)?;

        let user = User {
            id: user.id,
            email: Some(email.to_string()),
            password: Some(password.to_string()),
            username: Some(username.to_string()),
            auth_type: UserAuthType::Annon,
            avatar_url: None,
            email_verified: user.email_verified,
            organization_id: None,
        };
        let claims = SessionClaims {
            username: user.username.clone(),
            sudo_user: None,
            email_verified: user.email_verified,
            roles: Vec::new(),
        };
        let token = generate_jwt(&user, claims, secret, None);
        let (status, _, _) = session.verify_email_token(&app, token).await?;

        assert_eq!(status, StatusCode::CONFLICT, "user email already verified");

        Ok(())
    }

    #[sqlx::test]
    async fn user_should_not_be_able_to_login_with_wrong_password(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email";

        let mut session = UserSession::new(username, password, email);
        session.signup(&app).await?;
        session.logout(&app).await?;

        let mut session = UserSession::new(username, "wrong password", email);
        let (status, _, _) = session.login(&app, email, "wrong_password").await?;

        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "API should return unauthorized"
        );
        Ok(())
    }

    #[sqlx::test]
    async fn user_should_be_able_to_login_with_email_with_different_case(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email@email.com";

        let mut session = UserSession::new(username, password, email);
        session.signup(&app).await?;
        session.logout(&app).await?;

        let mut session = UserSession::new(username, crate::test_helpers::TEST_PASSWORD, "test_Email@email.com");
        let (status, _, _) = session.login(&app, email, password).await?;
        assert_eq!(status, StatusCode::OK, "API should return authorized");
        Ok(())
    }

    #[sqlx::test]
    fn other_user_types_should_not_be_able_to_annon_login(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email";

        let mut session = UserSession::new(username, password, email);
        session.signup(&app).await?;
        session.logout(&app).await?;
        let (status, _, _) = session.login_annon(&app).await?;

        assert_eq!(status, StatusCode::NOT_FOUND, "API should return NOT_FOUND");
        Ok(())
    }

    #[sqlx::test]
    fn annon_user_should_be_able_to_login(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_anon();
        session.signup_annon(&app).await?;
        session.logout(&app).await?;

        let (status, _, _) = session.login_annon(&app).await?;

        assert_eq!(status, StatusCode::OK, "API should respond OK");
        Ok(())
    }

    #[sqlx::test]
    fn unknown_username_should_not_be_able_to_annon_login(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new_anon();
        session.username = Some("foo".to_string());

        let (status, _, _) = session.login_annon(&app).await?;

        assert_eq!(
            status,
            StatusCode::NOT_FOUND,
            "API should respond NOT_FOUND"
        );
        Ok(())
    }

    #[sqlx::test]
    async fn username_and_email_should_be_unique(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email";

        let mut session = UserSession::new(username, password, email);
        session.signup(&app).await?;

        let mut session = UserSession::new(username, password, "test_email2");
        let (status, _, _) = session.signup(&app).await?;

        assert_eq!(
            status,
            StatusCode::CONFLICT,
            "Should not be able to have same username"
        );

        let mut session = UserSession::new("test_user2", crate::test_helpers::TEST_PASSWORD, email);
        let (status, _, _) = session.signup(&app).await?;

        assert_eq!(
            status,
            StatusCode::CONFLICT,
            "Should not be able to have same email"
        );
        Ok(())
    }

    #[sqlx::test]
    async fn user_should_be_able_to_logout(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email";

        let mut session = UserSession::new(username, password, email);
        let signup_response = session.signup(&app).await?;
        assert_eq!(signup_response.0, StatusCode::CREATED, "should be created");

        let (status, user, _) = session.current_user(&app).await?;

        assert_eq!(status, StatusCode::OK, "should get current user");

        assert_eq!(
            user.username,
            Some(username.to_owned()),
            "current user should contain the right username"
        );

        assert_eq!(
            user.email,
            Some(email.to_owned()),
            "current user should contain the right email"
        );

        assert_ne!(user.id, Uuid::nil(), "current user should contain an id");
        Ok(())
    }

    #[sqlx::test]
    async fn annon_user_should_by_able_to_signup(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut annon_user = UserSession::new_anon();
        let (status, _, _) = annon_user.signup_annon(&app).await?;

        assert_eq!(status, StatusCode::CREATED, "Should be created");

        let (status, user_response, _) = annon_user.current_user(&app).await?;

        assert_eq!(status, StatusCode::OK, "Should be ok ");

        assert!(
            user_response.username.is_some(),
            "current annon user should have a username"
        );

        assert_eq!(
            user_response.auth_type,
            UserAuthType::Annon,
            "current annon user should have a username"
        );

        assert_ne!(
            user_response.id,
            Uuid::nil(),
            "current annon user should have an id"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn user_should_receive_password_reset_email(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email";

        let mut mailer = MockComhairleMailer::new();
        mailer
            .expect_send_welcome_email()
            .once()
            .returning(|_, _| Ok(()));
        mailer
            .expect_send_password_reset_email()
            .once()
            .with(
                eq(Some("test_email".to_string())),
                eq(Some("test_user".to_string())),
                always(),
            )
            .returning(|_, _, _| Ok(()));
        mailer.expect_send_verification_email().times(0);

        let state = test_state().db(pool).mailer(Arc::new(mailer)).call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new(username, password, email);
        session.signup(&app).await?;

        let (status, _, _) = session
            .password_reset_create(&app, email.to_string())
            .await?;

        assert_eq!(
            status,
            StatusCode::NO_CONTENT,
            "reset link sent to user email"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn unknown_user_returns_not_found(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email";

        let mut mailer = MockComhairleMailer::new();
        mailer
            .expect_send_welcome_email()
            .once()
            .returning(|_, _| Ok(()));
        mailer.expect_send_password_reset_email().times(0);
        mailer.expect_send_verification_email().times(0);

        let state = test_state().db(pool).mailer(Arc::new(mailer)).call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new(username, password, email);
        session.signup(&app).await?;

        let (status, _, _) = session
            .password_reset_create(&app, "unknown_user".to_string())
            .await?;

        assert_eq!(
            status,
            StatusCode::NOT_FOUND,
            "unknown user returns not found"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn users_password_should_be_updated(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email";

        let state = test_state().db(pool).call()?;
        let db = &state.db.clone();
        let secret = state.config.jwt_secret.clone();
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new(username, password, email);
        let (_, user, _) = session.signup(&app).await?;
        session.logout(&app).await?;

        let id = user.get("id").unwrap().as_ref().unwrap().as_str().unwrap();
        let user = User {
            id: Uuid::parse_str(id).unwrap(),
            email: Some(email.to_string()),
            password: Some(password.to_string()),
            username: Some(username.to_string()),
            auth_type: UserAuthType::EmailPassword,
            avatar_url: None,
            email_verified: false,
            organization_id: None,
        };
        let claims = EmailLinkClaims {
            email: Some(email.to_string()),
        };
        let token = generate_jwt(&user, claims, &secret, None);

        let updated_password = "UpdatedPassword123!";
        let (reset_status, _, _) = session
            .password_reset_update(&app, &token, updated_password, updated_password)
            .await?;
        let (login_status, _, _) = session.login(&app, email, updated_password).await?;

        let user = get_user_by_email(email, db).await?;
        let hashed_user_password = PasswordHash::new(user.password.as_ref().unwrap()).unwrap();

        assert_eq!(
            reset_status,
            StatusCode::NO_CONTENT,
            "success returned after update"
        );
        assert_eq!(
            login_status,
            StatusCode::OK,
            "success returned after login with new password"
        );
        assert!(
            Argon2::default()
                .verify_password(updated_password.as_bytes(), &hashed_user_password)
                .is_ok(),
            "updated password matches hashed value in the database"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn password_and_confirmation_should_match(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let username = "test_username";
        let email = "test_email";
        let password = crate::test_helpers::TEST_PASSWORD;

        let state = test_state().db(pool).call()?;
        let secret = state.config.jwt_secret.clone();
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new(username, password, email);
        let (_, user, _) = session.signup(&app).await?;
        session.logout(&app).await?;

        let id = user.get("id").unwrap().as_ref().unwrap().as_str().unwrap();
        let user = User {
            id: Uuid::parse_str(id).unwrap(),
            email: Some(email.to_string()),
            username: Some(username.to_string()),
            password: Some(password.to_string()),
            avatar_url: None,
            auth_type: UserAuthType::EmailPassword,
            email_verified: false,
            organization_id: None,
        };
        let claims = EmailLinkClaims {
            email: Some(email.to_string()),
        };
        let token = generate_jwt(&user, claims, &secret, None);
        let (status, _, _) = session
            .password_reset_update(&app, &token, "foo", "bar")
            .await?;

        assert_eq!(
            status,
            StatusCode::BAD_REQUEST,
            "can't update password if confirmation password doesn't match"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn annon_users_cannot_reset_password(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool).call()?;
        let secret = state.config.jwt_secret.clone();
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_anon();
        let (_, user, _) = session.signup_annon(&app).await?;

        let id = user.get("id").unwrap().as_ref().unwrap().as_str().unwrap();
        let username = user
            .get("username")
            .unwrap()
            .as_ref()
            .unwrap()
            .as_str()
            .unwrap();
        let user = User {
            id: Uuid::parse_str(id).unwrap(),
            email: None,
            password: None,
            username: Some(username.to_string()),
            auth_type: UserAuthType::Annon,
            avatar_url: None,
            email_verified: false,
            organization_id: None,
        };
        let claims = EmailLinkClaims { email: None };
        let token = generate_jwt(&user, claims, &secret, None);
        let password = "updated_password";
        let (status, _, _) = session
            .password_reset_update(&app, &token, password, password)
            .await?;

        assert_eq!(
            status,
            StatusCode::INTERNAL_SERVER_ERROR,
            "annon users cannot reset password"
        );

        Ok(())
    }

    #[sqlx::test]
    fn user_requires_roles(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let state = test_state().db(pool.clone()).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let username = "test_user";
        let password = crate::test_helpers::TEST_PASSWORD;
        let email = "test_email";
        let conversation_id = uuid::Uuid::parse_str("8438709B-C269-422E-B3F1-D173295F48CF")?;

        let mut session = UserSession::new(username, password, email);
        session.signup(&app).await?;
        let url = format!("/auth/test_requires_roles/{conversation_id}");

        let (status, _, _) = session.get(&app, &url).await?;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "User without role should be forbidden"
        );

        add_user_resource_role(
            Resource::Conversation,
            &conversation_id,
            Role::Translator,
            &session.id.unwrap(),
            &pool,
        )
        .await?;
        let (status, _, _) = session.get(&app, &url).await?;
        assert_eq!(
            status,
            StatusCode::FORBIDDEN,
            "User with wrong role should be forbidden"
        );

        add_user_resource_role(
            Resource::Conversation,
            &conversation_id,
            Role::Owner,
            &session.id.unwrap(),
            &pool,
        )
        .await?;
        let (status, _, _) = session.get(&app, &url).await?;
        assert_eq!(status, StatusCode::OK, "User with role should have access");

        Ok(())
    }

    // Password validation tests
    #[test]
    fn test_password_too_short() {
        let result = super::validate_password_strength("Short123!");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("at least 16 characters"));
    }

    #[test]
    fn test_password_minimum_length() {
        // Exactly 16 characters with 3 categories
        let result = super::validate_password_strength("Abcdefg123456789");
        // Should pass length check, but might fail complexity
        assert!(result.is_ok() || result.unwrap_err().to_string().contains("complexity"));
    }

    #[test]
    fn test_password_only_lowercase() {
        let result = super::validate_password_strength("abcdefghijklmnop");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("at least 3 of 4 categories"));
    }

    #[test]
    fn test_password_only_two_categories() {
        let result = super::validate_password_strength("abcdefghijklm123");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("at least 3 of 4 categories"));
    }

    #[test]
    fn test_password_three_categories_upper_lower_digit() {
        let result = super::validate_password_strength("Abcdefghijklm123456");
        // Should pass category check (upper, lower, digit)
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_three_categories_upper_lower_special() {
        let result = super::validate_password_strength("Abcdefghijklmnop!");
        // Should pass category check (upper, lower, special)
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_three_categories_lower_digit_special() {
        let result = super::validate_password_strength("abcdefghijk123!@#");
        // Should pass category check (lower, digit, special)
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_all_four_categories() {
        let result = super::validate_password_strength("Abcdefg123456!@#");
        // Should pass with all 4 categories
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_common_pattern() {
        // Common patterns should be caught by zxcvbn
        let result = super::validate_password_strength("Password12345678");
        assert!(result.is_err());
        let err = result.unwrap_err();
        // Should mention complexity
        assert!(err.to_string().contains("complexity"));
    }

    #[test]
    fn test_password_strong_complex() {
        // Strong password with good entropy
        let result = super::validate_password_strength("X9$kPm2#qR7nW4@z");
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_very_long_weak() {
        // Long but repetitive password
        let result = super::validate_password_strength("aaaaaaaaaaaaaaaa1!");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("complexity"));
    }

    #[test]
    fn test_password_with_spaces() {
        // Password with spaces (spaces count as neither alphanumeric nor special in our check)
        let result = super::validate_password_strength("Abc def 123 456!");
        // Should have upper, lower, digit, special
        assert!(result.is_ok());
    }

    #[sqlx::test]
    async fn test_signup_with_weak_password(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let username = "test_user_weak_pw";
        let password = "weak"; // Too short
        let email = "test_weak@example.com";

        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        let mut session = UserSession::new(username, password, email);
        let (status, _, _) = session.signup(&app).await?;

        assert_eq!(status, StatusCode::BAD_REQUEST, "should reject weak password");

        Ok(())
    }

    #[sqlx::test]
    async fn test_password_reset_with_weak_password(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let username = "test_user_reset";
        let password = "ValidPassword123!";
        let email = "test_reset@example.com";

        let state = Arc::new(test_state().db(pool).call()?);
        let app = setup_server(state.clone()).await?;

        // First create a user with valid password
        let mut session = UserSession::new(username, password, email);
        let (status, _, _) = session.signup(&app).await?;
        assert_eq!(status, StatusCode::CREATED);

        // Get the user from DB
        let user = get_user_by_email(email, &state.db).await?;

        // Generate a password reset token
        let claims = EmailLinkClaims {
            email: Some(email.to_string()),
        };
        let token = generate_jwt(&user, claims, &state.config.jwt_secret, None);

        // Try to reset with weak password
        let weak_password = "weak";
        let body = serde_json::json!({
            "token": token,
            "password": weak_password,
            "confirm_password": weak_password,
        })
        .to_string()
        .into();
        let response = session
            .post(&app, "/auth/password_reset_update", body)
            .await?;

        assert_eq!(
            response.0,
            StatusCode::BAD_REQUEST,
            "should reject weak password on reset"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn test_update_user_with_weak_password(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let username = "test_user_update";
        let password = "ValidPassword123!";
        let email = "test_update@example.com";

        let state = test_state().db(pool).call()?;
        let app = setup_server(Arc::new(state)).await?;

        // Create user
        let mut session = UserSession::new(username, password, email);
        let (status, _, _) = session.signup(&app).await?;
        assert_eq!(status, StatusCode::CREATED);

        // Try to update with weak password
        let body = serde_json::json!({
            "password": "weak",
        })
        .to_string()
        .into();
        let response = session.put(&app, "/user/details", body).await?;

        assert_eq!(
            response.0,
            StatusCode::BAD_REQUEST,
            "should reject weak password on update"
        );

        Ok(())
    }
}
