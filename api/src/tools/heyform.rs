use heyform_sdk::{
    client::HeyFormClient, CreateFormInput, CreateHiddenFieldInput, CreateTeamInput, FormKind,
    InteractiveMode, LoginInput, SignUpInput,
};
use rand::{distributions::Alphanumeric, seq::SliceRandom, thread_rng, Rng};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::error::ComhairleError;

use super::ToolConfigSanitize;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct HeyFormToolConfig {
    pub survey_id: String,
    pub survey_url: String,
    pub admin_user: String,
    pub admin_password: String,
    pub workspace_id: String,
    pub project_id: String,
}

impl ToolConfigSanitize for HeyFormToolConfig {
    fn sanatize(&self) -> Self {
        Self {
            survey_id: self.survey_id.clone(),
            survey_url: self.survey_url.clone(),
            admin_user: "".into(),
            admin_password: "".into(),
            workspace_id: self.workspace_id.clone(),
            project_id: self.project_id.clone(),
        }
    }
}
#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct HeyFormToolSetup;

#[derive(Clone, Deserialize, Serialize, Debug, JsonSchema)]
pub struct HeyFormReport;

fn generate_password() -> String {
    let mut rng = thread_rng();

    // Ensure at least one of each required character type
    let lowercase = (b'a'..=b'z').map(char::from).collect::<Vec<_>>();
    let uppercase = (b'A'..=b'Z').map(char::from).collect::<Vec<_>>();
    let numbers = (b'0'..=b'9').map(char::from).collect::<Vec<_>>();

    let mut password = vec![
        *lowercase.choose(&mut rng).unwrap(),
        *uppercase.choose(&mut rng).unwrap(),
        *numbers.choose(&mut rng).unwrap(),
    ];

    // Fill remaining 9 characters from all alphanumeric
    let all_chars: Vec<char> = lowercase
        .into_iter()
        .chain(uppercase.into_iter())
        .chain(numbers.into_iter())
        .collect();

    for _ in 0..9 {
        password.push(*all_chars.choose(&mut rng).unwrap());
    }

    // Shuffle to avoid predictable pattern
    password.shuffle(&mut rng);
    password.into_iter().collect()
}

pub async fn setup(_setup_config: &HeyFormToolSetup) -> Result<HeyFormToolConfig, ComhairleError> {
    let client = HeyFormClient::new("https://forms.comhairle.scot")?;

    let username: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    let email = format!("{username}@comhairle.com");

    let password: String = generate_password();

    let signup_input = SignUpInput {
        name: username,
        email: email.clone(),
        password: password.clone(),
        team_id: None,
        invite_code: None,
    };

    client.signup(signup_input).await?;

    let login_input = LoginInput {
        email: email.clone(),
        password: password.clone(),
    };

    client.login(login_input).await?;

    let workspace_input = CreateTeamInput {
        name: "comhairle workspace".to_string(),
        avatar: None,
        members: None,
    };

    let workspace_id = client.create_workspace(workspace_input).await?;
    let project_id = client.get_teams().await?[0].projects[0].id.clone();

    let poll_input = CreateFormInput {
        project_id: project_id.clone(),
        name: Some("ComhairleForm".to_string()),
        interactive_mode: InteractiveMode::Conversational,
        kind: FormKind::Poll,
        name_schema: Some(vec![serde_json::json!({
            "id": "title",
            "title": "ComhairleForm",
            "kind": "title"
        })]),
    };

    let poll_id = client.create_poll(poll_input).await?;

    let hidden_field_input = CreateHiddenFieldInput {
        form_id: poll_id.clone(),
        field_id: "comhairle_user_id".to_string(),
        field_name: "comhairle_user_id".to_string(),
    };

    client.create_form_hidden_field(hidden_field_input).await?;

    let custom_css = r#"
        .heyform-container {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        }
        .heyform-question {
            color: #ffffff;
            font-family: 'Arial', sans-serif;
        }
        .heyform-button {
            background-color: #4CAF50;
            border: none;
            color: white;
            padding: 15px 32px;
            text-align: center;
            border-radius: 8px;
        }
    "#;
    client.set_custom_css(&poll_id, custom_css).await?;

    let poll_url = client.publish_poll(&poll_id, None).await?;

    Ok(HeyFormToolConfig {
        admin_user: email,
        admin_password: password,
        survey_url: poll_url,
        survey_id: poll_id,
        workspace_id,
        project_id,
    })
}
