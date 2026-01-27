use heyform_sdk::{
    CreateFormInput, CreateHiddenFieldInput, CreateTeamInput, FormKind, HeyFormClient,
    InteractiveMode, LoginInput, Result, SignUpInput,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Example: Create a new HeyForm client with automatic browser ID generation
    let client = HeyFormClient::new("https://forms.comhairle.scot")?;

    // Example 1: Sign up a new account

    let signup_input = SignUpInput {
        name: "John Doe3".to_string(),
        email: "john.doe3@example.com".to_string(),
        password: "SecurePassword123!".to_string(),
        team_id: None,
        invite_code: None,
    };

    match client.signup(signup_input).await {
        Ok(success) => println!("Signup successful: {}", success),
        Err(e) => println!("Signup failed: {}", e),
    }

    // Example 2: Login to an existing account
    let login_input = LoginInput {
        email: "john.doe3@example.com".to_string(),
        password: "SecurePassword123!".to_string(),
    };

    match client.login(login_input).await {
        Ok(success) => println!("Login successful: {}", success),
        Err(e) => println!("Login failed: {}", e),
    }

    // Example 3: Create a workspace
    let workspace_input = CreateTeamInput {
        name: "first workspace".to_string(),
        avatar: None,
        members: None,
    };

    let _workspace_id = match client.create_workspace(workspace_input).await {
        Ok(id) => {
            println!("Workspace created with ID: {}", id);
            id
        }
        Err(e) => {
            println!("Workspace creation failed: {}", e);
            return Ok(());
        }
    };

    let project_id = match client.get_teams().await {
        Ok(teams) => {
            println!("got teams: {:#?}", teams);
            teams[0].projects[0].id.clone()
        }
        Err(e) => {
            println!("Workspace creation failed: {}", e);
            return Ok(());
        }
    };

    // Example 4: Create a poll (form)
    // Note: You would need a valid project_id from your workspace
    // This is just an example - in practice you'd get this from the API

    let poll_input = CreateFormInput {
        project_id,
        name: Some("Customer Satisfaction Poll".to_string()),
        interactive_mode: InteractiveMode::Conversational,
        kind: FormKind::Poll,
        name_schema: Some(vec![serde_json::json!({
            "id": "title",
            "title": "Customer Satisfaction Poll",
            "kind": "title"
        })]),
    };

    let poll_id = match client.create_poll(poll_input).await {
        Ok(id) => {
            println!("Poll created with ID: {}", id);
            id
        }
        Err(e) => {
            println!("Poll creation failed: {}", e);
            return Ok(());
        }
    };

    // Example 4.5: Add a hidden field to the poll
    let hidden_field_input = CreateHiddenFieldInput {
        form_id: poll_id.clone(),
        field_id: nanoid::nanoid!(),
        field_name: "comhairle_user_id".to_string(),
    };

    match client.create_form_hidden_field(hidden_field_input).await {
        Ok(success) => {
            println!("Hidden field added successfully: {}", success);
        }
        Err(e) => {
            println!("Failed to add hidden field: {}", e);
        }
    }

    // Example 5: Publish the poll and get its embed URL
    match client.publish_poll(&poll_id, None).await {
        Ok(embed_url) => {
            println!("Poll published! Embed URL: {}", embed_url);
        }
        Err(e) => {
            println!("Poll publishing failed: {}", e);
        }
    }
    //
    // // Example 6: Set custom CSS on the poll
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

    match client.set_custom_css(&poll_id, custom_css).await {
        Ok(success) => {
            println!("Custom CSS applied successfully: {}", success);
        }
        Err(e) => {
            println!("Custom CSS application failed: {}", e);
        }
    }
    //
    // // Example 7: Get form details
    // match client.get_form(&poll_id).await {
    //     Ok(form) => {
    //         println!(
    //             "Form details retrieved: {} ({})",
    //             form.name.unwrap_or_default(),
    //             form.id
    //         );
    //     }
    //     Err(e) => {
    //         println!("Failed to get form details: {}", e);
    //     }
    // }
    //
    // // Example 8: Clone/duplicate the form
    // match client.clone_form(&poll_id).await {
    //     Ok(cloned_form_id) => {
    //         println!("Form cloned successfully! New form ID: {}", cloned_form_id);
    //     }
    //     Err(e) => {
    //         println!("Failed to clone form: {}", e);
    //     }
    // }
    //
    Ok(())
}
