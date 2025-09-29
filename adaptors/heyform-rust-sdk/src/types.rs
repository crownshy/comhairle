use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

// Authentication types
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SignUpInput {
    pub name: String,
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

// Team/Workspace types
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamInput {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub invite_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    pub created_at: String,
    pub projects: Vec<Project>,
}

// Form types
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateFormInput {
    pub project_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub interactive_mode: InteractiveMode,
    pub kind: FormKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_schema: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub id: String,
    pub team_id: String,
    pub project_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub interactive_mode: Option<i32>,
    pub kind: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<FormSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FormField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_settings: Option<ThemeSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FormSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_archive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_question_list: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FormField {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validations: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frozen: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ThemeSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<FormTheme>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FormTheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_text_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_brightness: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "customCSS")]
    pub custom_css: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFormInput {
    pub form_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFormThemeInput {
    pub form_id: String,
    pub theme: FormTheme,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateHiddenFieldInput {
    pub form_id: String,
    pub field_id: String,
    pub field_name: String,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum InteractiveMode {
    Classic = 1,
    Conversational = 2,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum FormKind {
    Form = 1,
    Survey = 2,
    Poll = 3,
    Quiz = 4,
}

// Project types
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub team_id: String,
    pub name: String,
}

// User types
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    pub is_email_verified: bool,
    pub is_social_account: bool,
}

// GraphQL response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<GraphQLErrorLocation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLErrorLocation {
    pub line: i32,
    pub column: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_kind_serialization() {
        let form_kind = FormKind::Poll;
        let serialized = serde_json::to_string(&form_kind).unwrap();
        assert_eq!(serialized, "3");
    }

    #[test]
    fn test_interactive_mode_serialization() {
        let mode = InteractiveMode::Conversational;
        let serialized = serde_json::to_string(&mode).unwrap();
        assert_eq!(serialized, "2");
    }

    #[test]
    fn test_form_kind_deserialization() {
        let json = "3";
        let deserialized: FormKind = serde_json::from_str(json).unwrap();
        assert!(matches!(deserialized, FormKind::Poll));
    }
}
