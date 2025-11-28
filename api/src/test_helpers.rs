use std::{collections::HashMap, error::Error, sync::Arc};
use uuid::Uuid;

use axum::{
    body::Body,
    http::{header::COOKIE, HeaderValue, Request, StatusCode},
    response::Response,
    Router,
};
use bon::builder;
use fake::{
    faker::lorem::en::{Paragraph, Sentence, Words},
    Fake,
};
use http_body_util::BodyExt;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use sqlx::PgPool;
use tower::ServiceExt;

use crate::{
    config::ComhairleConfig,
    mailer::MockComhairleMailer,
    models::users::UpdateUserRequest,
    websockets::{MockWebSocketService, WebSocketService},
    ComhairleState,
};

pub fn mock_mailer() -> Arc<MockComhairleMailer> {
    let mailer = MockComhairleMailer::base();
    Arc::new(mailer)
}

pub fn mock_websockets() -> Arc<dyn WebSocketService> {
    let websockets = MockWebSocketService::base();
    Arc::new(websockets)
}

#[builder]
pub fn test_state(
    db: PgPool,
    mailer: Option<Arc<MockComhairleMailer>>,
    config: Option<ComhairleConfig>,
    websockets: Option<Arc<dyn WebSocketService>>,
) -> Result<ComhairleState, Box<dyn Error>> {
    let state = ComhairleState {
        db,
        mailer: mailer.unwrap_or_else(mock_mailer),
        config: config.unwrap_or_else(|| test_config().unwrap()),
        websockets: websockets.unwrap_or_else(|| mock_websockets()),
    };
    Ok(state)
}

pub fn test_config() -> Result<ComhairleConfig, Box<dyn Error>> {
    let mut config = crate::config::load()?;
    config.admin_users = Some(vec!["admin@crown-shy.com".into()]);
    Ok(config)
}

pub fn extract<T: DeserializeOwned>(target: &str, entity: &serde_json::Value) -> T {
    if let Some(error) = entity.get("err") {
        println!("Got error {error:#?}");
    }
    let value = entity.get(target).to_owned();

    if value.is_none() {
        println!("Issue with value {entity:#?} {target:#?}");
    }
    let value = value.unwrap().to_owned();

    serde_json::from_value(value)
        .inspect_err(|e| println!("Failed to deserialize error {e:#?}"))
        .unwrap()
}

pub fn polis_tool_config() -> serde_json::Value {
    json!({
        "type" : "polis",
        "topic": "topic"
    })
}

pub fn learn_tool_config() -> serde_json::Value {
    json!({
    "type": "learn",
    "pages":[
        [
            {"lang": "en", "content" : "#Test", "type" : "markdown"}
        ]
    ]})
}

pub async fn response_to_json(response: Response) -> Value {
    let body = response.into_body().collect().await.unwrap().to_bytes();

    serde_json::from_slice(&body).unwrap_or_else(|_| {
        let body_str: String = std::str::from_utf8(&body)
            .expect("response to be a string")
            .into();
        json!({"err": body_str})
    })
}

pub struct UserSession {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub cookie: Option<HeaderValue>,
}

impl UserSession {
    pub fn new_anon() -> Self {
        Self {
            id: None,
            username: None,
            password: None,
            email: None,
            email_verified: false,
            cookie: None,
        }
    }

    pub fn new_admin() -> Self {
        Self {
            id: None,
            username: Some("admin".into()),
            password: Some("admin".into()),
            email: Some("admin@crown-shy.com".into()),
            email_verified: true,
            cookie: None,
        }
    }

    pub fn new(username: &str, password: &str, email: &str) -> Self {
        Self {
            id: None,
            username: Some(username.to_owned()),
            password: Some(password.to_owned()),
            email: Some(email.to_owned()),
            email_verified: false,
            cookie: None,
        }
    }

    pub async fn get(
        &mut self,
        app: &Router,
        url: &str,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        let mut request = Request::builder().uri(url).method("GET");

        if let Some(cookie) = &self.cookie {
            request = request.header(COOKIE, cookie)
        }

        let request = request.body(Body::empty()).unwrap();
        let response = app.clone().oneshot(request).await?;
        let status = response.status();

        let cookie = response
            .headers()
            .get(axum::http::header::SET_COOKIE)
            .map(|cookie| cookie.to_owned());

        if let Some(cookie) = &cookie {
            self.cookie = Some(cookie.clone());
        }

        let value = response_to_json(response).await;
        Ok((status, value, cookie))
    }

    pub async fn delete(
        &mut self,
        app: &Router,
        url: &str,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        let mut request = Request::builder().uri(url).method("DELETE");

        if let Some(cookie) = &self.cookie {
            request = request.header(COOKIE, cookie)
        }

        let request = request.body(Body::empty()).unwrap();
        let response = app.clone().oneshot(request).await?;
        let status = response.status();

        let cookie = response
            .headers()
            .get(axum::http::header::SET_COOKIE)
            .map(|cookie| cookie.to_owned());

        if let Some(cookie) = &cookie {
            self.cookie = Some(cookie.clone());
        }

        let value = response_to_json(response).await;
        Ok((status, value, cookie))
    }

    pub async fn post(
        &mut self,
        app: &Router,
        url: &str,
        body: Body,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        let mut request = Request::builder()
            .uri(url)
            .method("POST")
            .header("content-type", "application/json");

        if let Some(cookie) = &self.cookie {
            request = request.header(COOKIE, cookie)
        }

        let request = request.body(body).unwrap();
        let response = app.clone().oneshot(request).await?;
        let status = response.status();

        let cookie = response
            .headers()
            .get(axum::http::header::SET_COOKIE)
            .map(|cookie| cookie.to_owned());

        if let Some(cookie) = &cookie {
            self.cookie = Some(cookie.clone());
        }

        let value = response_to_json(response).await;
        Ok((status, value, cookie))
    }

    pub async fn put(
        &mut self,
        app: &Router,
        url: &str,
        body: Body,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        let mut request = Request::builder()
            .uri(url)
            .method("PUT")
            .header("content-type", "application/json");

        if let Some(cookie) = &self.cookie {
            request = request.header(COOKIE, cookie)
        }

        let request = request.body(body).unwrap();
        let response = app.clone().oneshot(request).await?;
        let status = response.status();

        let cookie = response
            .headers()
            .get(axum::http::header::SET_COOKIE)
            .map(|cookie| cookie.to_owned());

        if let Some(cookie) = &cookie {
            self.cookie = Some(cookie.clone());
        }

        let value = response_to_json(response).await;
        Ok((status, value, cookie))
    }
    pub async fn logout(
        &mut self,
        app: &Router,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        self.post(app, "/auth/logout", Body::empty()).await
    }

    pub async fn current_user(
        &mut self,
        app: &Router,
    ) -> Result<
        (
            StatusCode,
            HashMap<String, Option<Value>>,
            Option<HeaderValue>,
        ),
        Box<dyn Error>,
    > {
        let (status, value, cookie) = self.get(app, "/auth/current_user").await?;

        let user: HashMap<String, Option<Value>> = serde_json::from_value(value).unwrap();
        Ok((status, user, cookie))
    }

    pub async fn login(
        &mut self,
        app: &Router,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        self.post(
            app,
            "/auth/login",
            json!({"email":self.email, "password": self.password})
                .to_string()
                .into(),
        )
        .await
    }

    pub async fn login_annon(
        &mut self,
        app: &Router,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        self.post(
            app,
            "/auth/login_annon",
            json!({"username":self.username}).to_string().into(),
        )
        .await
    }

    pub async fn signup_annon(
        &mut self,
        app: &Router,
    ) -> Result<
        (
            StatusCode,
            HashMap<String, Option<Value>>,
            Option<HeaderValue>,
        ),
        Box<dyn Error>,
    > {
        let (status, value, cookie) = self.post(app, "/auth/signup_annon", Body::empty()).await?;
        let user: HashMap<String, Option<Value>> = serde_json::from_value(value)?;
        let username: String =
            serde_json::from_value(user.get("username").unwrap().clone().unwrap()).unwrap();
        self.username = Some(username);
        let id: String = serde_json::from_value(user.get("id").unwrap().clone().unwrap()).unwrap();
        self.id = Some(Uuid::parse_str(&id).unwrap());
        Ok((status, user, cookie))
    }

    pub async fn signup(
        &mut self,
        app: &Router,
    ) -> Result<
        (
            StatusCode,
            HashMap<String, Option<Value>>,
            Option<HeaderValue>,
        ),
        Box<dyn Error>,
    > {
        let body: Body = if self.username.is_some() {
            json!({"username":self.username, "password":self.password, "email":self.email})
                .to_string()
                .into()
        } else {
            Body::empty()
        };

        let (status, value, cookie) = self.post(app, "/auth/signup", body).await?;

        let user: HashMap<String, Option<Value>> = serde_json::from_value(value)?;

        self.cookie = cookie.clone();
        if let Some(id) = user.get("id") {
            if let Some(id) = id {
                let id: String = serde_json::from_value(id.clone()).unwrap();
                self.id = Some(Uuid::parse_str(&id).unwrap());
            }
        }

        Ok((status, user, cookie))
    }

    pub async fn resend_verification_email(
        &mut self,
        app: &Router,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        self.post(
            app,
            "/auth/resend_verification_email",
            json!({ "id": self.id }).to_string().into(),
        )
        .await
    }

    pub async fn verify_email_token(
        &mut self,
        app: &Router,
        token: String,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        self.post(
            app,
            "/auth/verify_email_token",
            json!({ "token": token }).to_string().into(),
        )
        .await
    }

    pub async fn update_user_details(
        &mut self,
        app: &Router,
        update_user: UpdateUserRequest,
    ) -> Result<
        (
            StatusCode,
            HashMap<String, Option<Value>>,
            Option<HeaderValue>,
        ),
        Box<dyn Error>,
    > {
        let (status, value, cookie) = self.put(
            app,
            "/user/details",
            json!({ "username": update_user.username, "password": update_user.password, "email_verified": update_user.email_verified }).to_string().into()
        ).await?;

        let user: HashMap<String, Option<Value>> = serde_json::from_value(value)?;

        Ok((status, user, cookie))
    }

    pub async fn password_reset_create(
        &mut self,
        app: &Router,
        email: String,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        self.post(
            app,
            "/auth/password_reset_create",
            json!({ "email": email }).to_string().into(),
        )
        .await
    }

    pub async fn password_reset_update(
        &mut self,
        app: &Router,
        token: &str,
        password: &str,
        confirm_password: &str,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        self.post(
            app,
            "/auth/password_reset_update",
            json!({ "token": token, "password": password, "confirm_password": confirm_password})
                .to_string()
                .into(),
        )
        .await
    }

    pub async fn create_conversation(
        &mut self,
        app: &Router,
        new_coversation: serde_json::Value,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        let (status, value, cookie) = self
            .post(app, "/conversation", new_coversation.to_string().into())
            .await?;
        Ok((status, value, cookie))
    }

    pub async fn update_conversation(
        &mut self,
        app: &Router,
        id: &str,
        conversation_update: serde_json::Value,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        let (status, value, cookie) = self
            .put(
                app,
                &format!("/conversation/{id}"),
                conversation_update.to_string().into(),
            )
            .await?;
        Ok((status, value, cookie))
    }

    pub async fn list_conversations(
        &mut self,
        app: &Router,
        offset: i32,
        limit: i32,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        let url = format!("/conversation?limit={}&offset={}", limit, offset);
        self.get(app, &url).await
    }

    pub async fn delete_conversation(
        &mut self,
        app: &Router,
        id: &str,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        self.delete(app, &format!("/conversation/{id}")).await
    }

    pub async fn create_random_conversation(
        &mut self,
        app: &Router,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        let title: String = Sentence(1..10).fake();
        let description: String = Paragraph(3..4).fake();
        let short_description: String = Paragraph(5..8).fake();
        let image_url: String = "https://fakeimg.pl/1000x600".into();
        let tags: Vec<String> = Words(2..4).fake();
        let is_public: bool = true;
        let is_invite_only: bool = false;

        self.create_conversation(
            app,
            json!({
                "title" : title,
                "short_description": short_description,
                "description": description,
                "image_url": image_url,
                "tags" : tags,
                "is_public": is_public,
                "is_invite_only" : is_invite_only
            }),
        )
        .await
    }

    pub async fn create_random_workflow_steps(
        &mut self,
        app: &Router,
        conversation_id: &str,
        workflow_id: &str,
        _no: i32,
    ) -> Result<Vec<Value>, Box<dyn Error>> {
        let url = format!("/conversation/{conversation_id}/workflow/{workflow_id}/workflow_step");
        let mut workflow_steps: Vec<serde_json::Value> = vec![];
        // Create a bunch of steps
        for no in 0..10 {
            let (_, step, _) = self
                .post(
                    &app,
                    &url,
                    json!({
                    "name": format!("{}", no+1),
                    "step_order": no+1,
                    "activation_rule" : "manual",
                    "description": "A manually retired polis workflow step",
                    "required":false,
                    "is_offline": false,
                    "tool_setup": learn_tool_config()})
                    .to_string()
                    .into(),
                )
                .await
                .expect("Workflow step to be created");
            workflow_steps.push(step);
        }
        Ok(workflow_steps)
    }

    pub async fn create_random_workflow(
        &mut self,
        app: &Router,
        convo_id: &str,
    ) -> Result<(StatusCode, Value, Option<HeaderValue>), Box<dyn Error>> {
        let name: String = Sentence(1..10).fake();
        let description: String = Paragraph(6..10).fake();
        let is_active = true;
        let is_public = true;

        self.post(
            app,
            &format!("/conversation/{convo_id}/workflow"),
            json!({
                "name":name,
                "description": description,
                "is_active": is_active,
                "is_public": is_public,
                "auto_login": false,
            })
            .to_string()
            .into(),
        )
        .await
    }

    pub async fn get_conversation(
        &mut self,
        app: &Router,
        id: &str,
    ) -> Result<(StatusCode, HashMap<String, Value>, Option<HeaderValue>), Box<dyn Error>> {
        let (status, value, cookie) = self.get(app, &format!("/conversation/{id}")).await?;
        let value: HashMap<String, serde_json::Value> = serde_json::from_value(value)?;
        Ok((status, value, cookie))
    }
}
