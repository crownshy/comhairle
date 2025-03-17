use axum::{
    extract::{FromRequestParts, Json, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    RequestPartsExt, Router,
};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use tower_cookies::{Cookie, Cookies};

use crate::{
    config::ComhairleConfig,
    error::ComhairleError,
    models::users::{create_annon_user, create_user, get_user_by_email, get_user_by_id, User},
    ComhairleState,
};

/// This is the key that we use in the cookie for the JWT
const AUTH_KEY: &str = "auth-token";

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
#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

/// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    id: String,
    username: Option<String>,
    sudo_user: Option<String>,
}

/// Generate JWT
fn generate_jwt(user: &User, secret: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration,
        username: user.username.clone(),
        id: user.id.to_string(),
        sudo_user: None,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

/// Expected payload for a signin request  
#[derive(Deserialize, Debug)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub email: String,
}

/// Signup handler
async fn signup(
    State(state): State<Arc<ComhairleState>>,
    cookies: Cookies,
    Json(payload): Json<SignupRequest>,
) -> Result<Response, ComhairleError> {
    let user = create_user(&payload, &state.db).await?;
    let token = generate_jwt(&user, &state.config.jwt_secret);

    let cookie = Cookie::build((AUTH_KEY, token))
        .path("/")
        .secure(true)
        .http_only(true);

    cookies.add(cookie.into());
    Ok((StatusCode::CREATED, Json(user)).into_response())
}

/// Signup handler for annon
async fn signup_annon(State(state): State<Arc<ComhairleState>>, cookies: Cookies) -> Response {
    match create_annon_user(&state.db).await {
        Ok(user) => {
            let token = generate_jwt(&user, &state.config.jwt_secret);
            let cookie = Cookie::build((AUTH_KEY, token))
                .path("/")
                .secure(true)
                .http_only(true);

            cookies.add(cookie.into());
            (StatusCode::CREATED, Json(user)).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"err" : String::from("Failed to create annon user")})),
        )
            .into_response(),
    }
}

/// Email/Password Login Handler
// TODO Tidy this up
async fn login(
    State(state): State<Arc<ComhairleState>>,
    cookies: Cookies,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    if let Ok(user) = get_user_by_email(&payload.email, &state.db).await {
        if let Some(password) = &user.password {
            let hash = PasswordHash::new(password)
                .map_err(|_| ComhairleError::PasswordHash)
                .expect("Password to be hashable");

            if Argon2::default()
                .verify_password(&payload.password.into_bytes(), &hash)
                .is_ok()
            {
                let token = generate_jwt(&user.clone(), &state.config.jwt_secret);
                let cookie = Cookie::build((AUTH_KEY, token))
                    .path("/")
                    .secure(true)
                    .http_only(true);
                cookies.add(cookie.into());

                return (StatusCode::OK, Json(user)).into_response();
            } else {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"err" : String::from("Incorrect Username / Password")})),
                )
                    .into_response();
            }
        } else {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"err":"This user cannot login with a password"})),
            )
                .into_response();
        }
    } else {
        return (StatusCode::NOT_FOUND, Json(json!({"err":"No such user"}))).into_response();
    };
}

/// Decode a JWT
pub fn decode_jwt(jwt: &str, secret: &str) -> Result<TokenData<Claims>, StatusCode> {
    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}

/// An extractor to get a required current user.
/// If no user is logged in then this will fail and
/// Return a Not Found response
pub struct RequiredUser(pub User);

/// An extractor to get the current user if they exist
/// If a user is not logged in, this will still run
/// but produce a None value in the extractor
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
        let cookies = parts.extract::<Cookies>().await;
        if let Ok(cookies) = cookies {
            let token = cookies.get(AUTH_KEY);
            if let Some(token) = token {
                let token_str = token.to_string();
                let token_str = token_str.split("=").skip(1).next().unwrap();
                let poss_user = validate_jwt(state, &token_str).await;
                Ok(OptionalUser(poss_user.ok()))
            } else {
                Ok(OptionalUser(None))
            }
        } else {
            Ok(OptionalUser(None))
        }
    }
}

/// Ensure the JTW is valid and if it is return the associated user
/// May break this out in future for routes that require a valid
/// user but dont care who they are
pub async fn validate_jwt(
    state: &Arc<ComhairleState>,
    token: &str,
) -> Result<User, ComhairleError> {
    let token_data = match decode_jwt(token, &state.config.jwt_secret) {
        Ok(data) => data,
        Err(_) => {
            return Err(ComhairleError::AuthJWTError(
                "Unable to decode token".to_string(),
            ))
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
pub async fn logout(cookies: Cookies) -> impl IntoResponse {
    cookies.remove(Cookie::build(AUTH_KEY).path("/").into());
    Json(json!({"msg":"Logged out"}))
}

/// Handler for the current user if there is one
pub async fn current_user(OptionalUser(user): OptionalUser) -> impl IntoResponse {
    if let Some(user) = user {
        (StatusCode::OK, Json(user)).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"err" : "No current logged in user"})),
        )
            .into_response()
    }
}

/// Function to set up the auth routes
pub async fn router(_config: &ComhairleConfig) -> Router<Arc<ComhairleState>> {
    Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
        .route("/signup_annon", post(signup_annon))
        .route("/logout", post(logout))
        .route("/current_user", get(current_user))
}

#[cfg(test)]
mod tests {
    use crate::{config, setup_server, test_helpers::UserSession};
    use axum::http::StatusCode;
    use sqlx::PgPool;
    use std::error::Error;

    #[sqlx::test]
    fn user_should_be_able_to_sign_up(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let config = config::load()?;
        let app = setup_server(config, pool).await?;

        let username = "test_user";
        let password = "test_password";
        let email = "test_email";

        let mut session = UserSession::new(username, password, email);
        let (status, _, _) = session.signup(&app).await?;
        assert_eq!(status, StatusCode::CREATED, "should be created");

        let (status, user, _) = session.current_user(&app).await?;

        assert_eq!(status, StatusCode::OK, "should get current user");

        assert_eq!(
            *user.get("username").unwrap(),
            Some(username.to_owned()),
            "current user should contain the right username"
        );

        assert_eq!(
            *user.get("auth_type").unwrap(),
            Some("email_password".to_owned()),
            "current user should have auth_type email password"
        );
        assert_eq!(
            *user.get("email").unwrap(),
            Some(email.to_owned()),
            "current user should contain the right email"
        );

        assert!(
            user.get("id").is_some(),
            "current user should contain an id"
        );
        Ok(())
    }

    #[sqlx::test]
    fn user_should_not_be_able_to_login_with_wrong_password(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let config = config::load()?;
        let app = setup_server(config, pool).await?;

        let username = "test_user";
        let password = "test_password";
        let email = "test_email";

        let mut session = UserSession::new(username, password, email);
        session.signup(&app).await?;

        session.logout(&app).await?;

        let mut session = UserSession::new(username, "wrong password", email);
        let (status, _, _) = session.login(&app).await?;

        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "API should return unauthorized"
        );
        Ok(())
    }
    #[sqlx::test]
    fn username_and_email_should_be_unique(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let config = config::load()?;
        let app = setup_server(config, pool).await?;

        let username = "test_user";
        let password = "test_password";
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

        let mut session = UserSession::new("test_user2", password, email);
        let (status, _, _) = session.signup(&app).await?;

        assert_eq!(
            status,
            StatusCode::CONFLICT,
            "Should not be able to have same email"
        );
        Ok(())
    }

    #[sqlx::test]
    fn user_should_be_able_to_logout(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let config = config::load()?;
        let app = setup_server(config, pool).await?;

        let username = "test_user";
        let password = "test_password";
        let email = "test_email";

        let mut session = UserSession::new(username, password, email);
        let signup_response = session.signup(&app).await?;
        assert_eq!(signup_response.0, StatusCode::CREATED, "should be created");

        let (status, user, _) = session.current_user(&app).await?;

        assert_eq!(status, StatusCode::OK, "should get current user");

        assert_eq!(
            *user.get("username").unwrap(),
            Some(username.to_owned()),
            "current user should contain the right username"
        );

        assert_eq!(
            *user.get("email").unwrap(),
            Some(email.to_owned()),
            "current user should contain the right email"
        );

        assert!(
            user.get("id").is_some(),
            "current user should contain an id"
        );
        Ok(())
    }

    #[sqlx::test]
    fn annon_user_should_by_able_to_signup(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let config = config::load()?;
        let app = setup_server(config, pool).await?;
        let mut annon_user = UserSession::new_anon();
        let (status, _, _) = annon_user.signup_annon(&app).await?;

        assert_eq!(status, StatusCode::CREATED, "Should be created");

        let (status, user_response, _) = annon_user.current_user(&app).await?;

        assert_eq!(status, StatusCode::OK, "Should be ok ");

        assert!(
            user_response.get("username").unwrap().is_some(),
            "current annon user should have a username"
        );

        assert_eq!(
            *user_response.get("auth_type").unwrap(),
            Some("annon".to_owned()),
            "current annon user should have a username"
        );

        assert!(
            user_response.get("id").unwrap().is_some(),
            "current annon user should have an id"
        );

        Ok(())
    }
}
