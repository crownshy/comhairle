#[cfg(test)]
mod tests {
    use heyform_sdk::{
        HeyFormClient, SignUpInput, LoginInput, CreateTeamInput, CreateFormInput,
        InteractiveMode, FormKind, FormTheme, UpdateFormThemeInput,
    };

    // Note: These are integration tests that would require a running HeyForm instance
    // In practice, you might want to use mocking for unit tests

    #[tokio::test]
    #[ignore] // Ignore by default since it requires real API
    async fn test_signup_flow() {
        let client = HeyFormClient::default().unwrap();
        
        let signup_input = SignUpInput {
            name: "Test User".to_string(),
            email: format!("test+{}@example.com", chrono::Utc::now().timestamp()),
            password: "TestPassword123!".to_string(),
            team_id: None,
            invite_code: None,
        };

        let result = client.signup(signup_input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Ignore by default since it requires real API
    async fn test_login_flow() {
        let client = HeyFormClient::default().unwrap();
        
        let login_input = LoginInput {
            email: "test@example.com".to_string(),
            password: "TestPassword123!".to_string(),
        };

        let result = client.login(login_input).await;
        // This will fail without valid credentials, but tests the flow
        assert!(result.is_err());
    }

    #[test]
    fn test_client_creation() {
        let client = HeyFormClient::default();
        assert!(client.is_ok());

        let client = HeyFormClient::new("https://custom.heyform.net");
        assert!(client.is_ok());
    }

    #[test]
    fn test_form_theme_serialization() {
        let theme = FormTheme {
            font_family: Some("Arial".to_string()),
            question_text_color: Some("#333333".to_string()),
            answer_text_color: Some("#666666".to_string()),
            button_background: Some("#007bff".to_string()),
            button_text_color: Some("#ffffff".to_string()),
            background_color: Some("#f8f9fa".to_string()),
            background_image: None,
            background_brightness: Some(0),
            logo: None,
            custom_css: Some(".custom { color: red; }".to_string()),
        };

        let json = serde_json::to_string(&theme);
        assert!(json.is_ok());

        let deserialized: Result<FormTheme, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }

    #[test]
    fn test_form_input_types() {
        let create_input = CreateFormInput {
            project_id: "test-project".to_string(),
            name: Some("Test Form".to_string()),
            interactive_mode: InteractiveMode::Conversational,
            kind: FormKind::Poll,
        };

        let json = serde_json::to_string(&create_input);
        assert!(json.is_ok());
    }

    #[tokio::test]
    async fn test_error_handling() {
        let client = HeyFormClient::new("https://invalid-url-that-does-not-exist.com");
        assert!(client.is_ok());

        let login_input = LoginInput {
            email: "test@example.com".to_string(),
            password: "password".to_string(),
        };

        let result = client.unwrap().login(login_input).await;
        assert!(result.is_err());
    }
}