use std::{collections::HashMap, error::Error};

use axum::{
    body::Body,
    http::{header::COOKIE, HeaderValue, Request, StatusCode},
    response::Response,
    Router,
};
use http_body_util::BodyExt;
use serde_json::{json, Value};

use tower::ServiceExt;

pub async fn response_to_str(response: Response) -> String {
    let body = response.into_body().collect().await.unwrap().to_bytes();
    std::str::from_utf8(&body)
        .expect("The response body to be a string")
        .into()
}

pub async fn response_to_json(response: Response) -> Value {
    let body = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body).unwrap()
}

pub struct UserSession {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub cookie: Option<HeaderValue>,
}

impl UserSession {
    pub fn new_anon() -> Self {
        Self {
            username: None,
            password: None,
            email: None,
            cookie: None,
        }
    }
    pub fn new(username: &str, password: &str, email: &str) -> Self {
        Self {
            username: Some(username.to_owned()),
            password: Some(password.to_owned()),
            email: Some(email.to_owned()),
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
            HashMap<String, Option<String>>,
            Option<HeaderValue>,
        ),
        Box<dyn Error>,
    > {
        let (status, value, cookie) = self.get(app, "/auth/current_user").await?;

        let user: HashMap<String, Option<String>> = serde_json::from_value(value).unwrap();
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

    pub async fn signup_annon(
        &mut self,
        app: &Router,
    ) -> Result<
        (
            StatusCode,
            HashMap<String, Option<String>>,
            Option<HeaderValue>,
        ),
        Box<dyn Error>,
    > {
        let (status, value, cookie) = self.post(&app, "/auth/signup_annon", Body::empty()).await?;
        let user: HashMap<String, Option<String>> = serde_json::from_value(value)?;
        Ok((status, user, cookie))
    }

    pub async fn signup(
        &mut self,
        app: &Router,
    ) -> Result<
        (
            StatusCode,
            HashMap<String, Option<String>>,
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

        let user: HashMap<String, Option<String>> = serde_json::from_value(value)?;

        self.cookie = cookie.clone();

        Ok((status, user, cookie))
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
