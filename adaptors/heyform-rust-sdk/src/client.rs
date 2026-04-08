use reqwest::{
    cookie::{CookieStore, Jar},
    Client,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use url::Url;

use crate::{
    error::{HeyFormError, Result},
    queries::*,
    types::*,
};

/// HeyForm client for interacting with the GraphQL API
pub struct HeyFormClient {
    client: Client,
    base_url: String,
    cookies: Arc<Jar>,
    browser_id: Option<String>,
}

impl HeyFormClient {
    /// Create a new HeyForm client
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        Self::with_browser_id(base_url, None)
    }

    /// Create a new HeyForm client with a browser ID
    pub fn with_browser_id(
        base_url: impl Into<String>,
        browser_id: Option<String>,
    ) -> Result<Self> {
        let base_url = base_url.into();
        let cookies = Arc::new(reqwest::cookie::Jar::default());
        let client = Client::builder().cookie_provider(cookies.clone()).build()?;

        // Generate a random nanoid if no browser ID is provided
        let browser_id = browser_id.unwrap_or_else(|| nanoid::nanoid!(12));

        let heyform_client = Self {
            client,
            base_url,
            cookies,
            browser_id: Some(browser_id.clone()),
        };

        heyform_client.set_browser_id_cookie(&browser_id)?;
        heyform_client.set_consent_cookie()?;

        Ok(heyform_client)
    }

    /// Create a client with default HeyForm URL
    pub fn default() -> Result<Self> {
        Self::new("https://app.heyform.net")
    }

    /// Get the GraphQL endpoint URL
    fn graphql_url(&self) -> Result<String> {
        Ok(format!("{}/graphql", self.base_url))
    }

    /// Set the x-browser-Id cookie
    fn set_browser_id_cookie(&self, browser_id: &str) -> Result<()> {
        let url = Url::parse(&self.base_url)?;
        let cookie = format!("HEYFORM_BROWSER_ID={}", browser_id);
        self.cookies.add_cookie_str(&cookie, &url);
        Ok(())
    }

    /// Set the consent cookie (cc_cookie)
    fn set_consent_cookie(&self) -> Result<()> {
        let url = Url::parse(&self.base_url)?;
        let consent_data = json!({
            "categories": ["necessary"],
            "revision": 0,
            "data": null,
            "consentTimestamp": "2025-04-21T21:04:17.889Z",
            "consentId": nanoid::nanoid!(),
            "services": {"necessary": []},
            "languageCode": "en",
            "lastConsentTimestamp": "2025-04-21T21:04:17.889Z",
            "expirationTime": 1760994257890i64
        });
        let cookie = format!("cc_cookie={}", consent_data);
        self.cookies.add_cookie_str(&cookie, &url);
        Ok(())
    }

    /// Debug method to print current cookies
    pub fn debug_cookies(&self) -> Result<()> {
        let url = Url::parse(&self.base_url)?;
        let cookies = self.cookies.cookies(&url);
        if let Some(cookie_header) = cookies {
            println!("Current cookies: {}", cookie_header.to_str().unwrap_or(""));
        } else {
            println!("No cookies found");
        }
        Ok(())
    }

    /// Update the browser ID and set the cookie
    pub fn set_browser_id(&mut self, browser_id: String) -> Result<()> {
        self.set_browser_id_cookie(&browser_id)?;
        self.browser_id = Some(browser_id);
        Ok(())
    }

    /// Get the current browser ID
    pub fn get_browser_id(&self) -> Option<&String> {
        self.browser_id.as_ref()
    }

    /// Execute a GraphQL query/mutation
    async fn execute_graphql<T>(
        &self,
        query: &str,
        variables: Value,
        operation_name: Option<&str>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut payload = json!({
            "query": query,
            "variables": variables
        });

        if let Some(op_name) = operation_name {
            payload["operationName"] = json!(op_name);
        }

        let mut request_builder = self
            .client
            .post(self.graphql_url()?)
            .header("Content-Type", "application/json");

        // Add x-browser-Id header if browser ID is set
        if let Some(browser_id) = &self.browser_id {
            request_builder = request_builder.header("x-browser-Id", browser_id);
        }

        let response = request_builder.json(&payload).send().await?;

        let status = response.status();
        println!("Response Status: {}", status);

        if !status.is_success() {
            let error_body = response.text().await?;
            println!("Error Response Body: {}", error_body);
            return Err(HeyFormError::GraphQL(format!(
                "HTTP {}: {}",
                status, error_body
            )));
        }

        let response_text = response.text().await?;
        println!("Response Body: {}", response_text);

        let graphql_response: GraphQLResponse<T> = serde_json::from_str(&response_text)?;

        if let Some(errors) = graphql_response.errors {
            let error_messages: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();
            println!("GraphQL Errors: {:?}", errors);
            return Err(HeyFormError::GraphQL(error_messages.join("; ")));
        }

        graphql_response
            .data
            .ok_or_else(|| HeyFormError::GraphQL("No data in response".to_string()))
    }

    /// Sign up a new user account
    pub async fn signup(&self, input: SignUpInput) -> Result<bool> {
        let variables = json!({ "input": input });
        let response: HashMap<String, bool> = self
            .execute_graphql(SIGNUP_MUTATION, variables, None)
            .await?;

        Ok(response.get("signUp").copied().unwrap_or(false))
    }

    /// Login to an existing account
    pub async fn login(&self, input: LoginInput) -> Result<bool> {
        self.debug_cookies()?;

        let variables = json!({ "input": input });
        let response: HashMap<String, bool> = self
            .execute_graphql(LOGIN_MUTATION, variables, None)
            .await?;
        self.debug_cookies()?;

        Ok(response.get("login").copied().unwrap_or(false))
    }

    /// Create a new workspace (team)
    pub async fn create_workspace(&self, input: CreateTeamInput) -> Result<String> {
        self.debug_cookies()?;

        let variables = json!({ "input": input });
        let response: HashMap<String, String> = self
            .execute_graphql(CREATE_TEAM_MUTATION, variables, Some("createTeam"))
            .await?;

        response
            .get("createTeam")
            .cloned()
            .ok_or_else(|| HeyFormError::GraphQL("No team ID returned".to_string()))
    }

    /// Create a new form (poll)
    pub async fn create_poll(&self, input: CreateFormInput) -> Result<String> {
        let variables = json!({ "input": input });
        let response: HashMap<String, String> = self
            .execute_graphql(CREATE_FORM_MUTATION, variables, Some("createForm"))
            .await?;

        response
            .get("createForm")
            .cloned()
            .ok_or_else(|| HeyFormError::GraphQL("No form ID returned".to_string()))
    }

    /// Publish a form and get its embed URL
    pub async fn publish_poll(&self, form_id: &str, base_url: Option<&str>) -> Result<String> {
        // First, update the form to be published and active
        let update_input = UpdateFormInput {
            form_id: form_id.to_string(),
            name: None,
            active: Some(true),
            published: Some(true),
        };

        let variables = json!({ "input": update_input });
        let _: HashMap<String, bool> = self
            .execute_graphql(UPDATE_FORM_MUTATION, variables, Some("updateForm"))
            .await?;

        // Generate the embed URL
        let base = base_url.unwrap_or(&self.base_url);
        let embed_url = format!("{}/form/{}", base, form_id);

        Ok(embed_url)
    }

    pub async fn update_poll(&self, form_id: &str, form: Form) -> Result<bool> {
        let variables = json!({
            "input": {
                "formId": form_id,
                "fields": form.fields
            }
        });

        let response: HashMap<String, bool> = self
            .execute_graphql(
                UPDATE_FORM_SCHEMAS_MUTATION,
                variables,
                Some("updateFormSchemas"),
            )
            .await?;

        Ok(response.get("updateFormSchemas").copied().unwrap_or(false))
    }

    /// Set custom CSS on a form
    pub async fn set_custom_css(&self, form_id: &str, custom_css: &str) -> Result<bool> {
        let theme = FormTheme {
            font_family: None,
            question_text_color: None,
            answer_text_color: None,
            button_background: None,
            button_text_color: None,
            background_color: None,
            background_image: None,
            background_brightness: None,
            logo: None,
            custom_css: Some(custom_css.to_string()),
        };

        let input = UpdateFormThemeInput {
            form_id: form_id.to_string(),
            theme,
        };

        let variables = json!({ "input": input });
        let response: HashMap<String, bool> = self
            .execute_graphql(
                UPDATE_FORM_THEME_MUTATION,
                variables,
                Some("updateFormTheme"),
            )
            .await?;

        Ok(response.get("updateFormTheme").copied().unwrap_or(false))
    }

    /// Get form details
    pub async fn get_form(&self, form_id: &str) -> Result<Form> {
        let variables = json!({
            "input": {
                "formId": form_id
            }
        });
        let response: HashMap<String, Form> = self
            .execute_graphql(FORM_DETAIL_QUERY, variables, None)
            .await?;

        response
            .get("formDetail")
            .cloned()
            .ok_or_else(|| HeyFormError::NotFound("Form not found".to_string()))
    }

    /// Clone/duplicate an existing form
    pub async fn clone_form(&self, form_id: &str) -> Result<String> {
        let variables = json!({
            "input": {
                "formId": form_id
            }
        });
        let response: HashMap<String, String> = self
            .execute_graphql(DUPLICATE_FORM_MUTATION, variables, Some("duplicateForm"))
            .await?;

        response
            .get("duplicateForm")
            .cloned()
            .ok_or_else(|| HeyFormError::GraphQL("No form ID returned".to_string()))
    }

    /// Get current user details
    pub async fn get_user(&self) -> Result<User> {
        let response: HashMap<String, User> = self
            .execute_graphql(USER_DETAIL_QUERY, json!({}), None)
            .await?;

        response
            .get("user")
            .cloned()
            .ok_or_else(|| HeyFormError::NotFound("User not found".to_string()))
    }

    /// Get user's teams/workspaces
    pub async fn get_teams(&self) -> Result<Vec<Team>> {
        let response: HashMap<String, Vec<Team>> =
            self.execute_graphql(TEAMS_QUERY, json!({}), None).await?;

        Ok(response.get("teams").cloned().unwrap_or_default())
    }

    /// Create a hidden field in a form
    pub async fn create_form_hidden_field(&self, input: CreateHiddenFieldInput) -> Result<bool> {
        let variables = json!({ "input": input });
        let response: HashMap<String, bool> = self
            .execute_graphql(
                CREATE_FORM_HIDDEN_FIELD_MUTATION,
                variables,
                Some("createFormHiddenField"),
            )
            .await?;

        Ok(response
            .get("createFormHiddenField")
            .copied()
            .unwrap_or(false))
    }
}
