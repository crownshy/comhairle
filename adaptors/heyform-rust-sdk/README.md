# HeyForm Rust SDK

A Rust client library for interacting with the HeyForm GraphQL API. This SDK provides easy-to-use methods for user authentication, workspace management, form creation, and customization.

## Features

- ✅ User authentication (signup/login)
- ✅ Workspace (team) management
- ✅ Form/poll creation
- ✅ Form publishing with embed URLs
- ✅ Custom CSS styling
- ✅ Async/await support
- ✅ Type-safe GraphQL queries

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
heyform-sdk = "0.1.0"
```

## Quick Start

```rust
use heyform_sdk::{
    HeyFormClient, SignUpInput, CreateTeamInput, CreateFormInput,
    InteractiveMode, FormKind, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client
    let client = HeyFormClient::default()?;
    
    // Sign up a new account
    let signup = SignUpInput {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        password: "SecurePassword123!".to_string(),
        team_id: None,
        invite_code: None,
    };
    client.signup(signup).await?;
    
    // Create a workspace
    let workspace = CreateTeamInput {
        name: "My Company".to_string(),
        avatar: None,
        members: None,
    };
    let workspace_id = client.create_workspace(workspace).await?;
    
    // Create a poll
    let poll = CreateFormInput {
        project_id: "your-project-id".to_string(),
        name: Some("Customer Survey".to_string()),
        interactive_mode: InteractiveMode::Conversational,
        kind: FormKind::Poll,
    };
    let poll_id = client.create_poll(poll).await?;
    
    // Publish and get embed URL
    let embed_url = client.publish_poll(&poll_id, None).await?;
    println!("Poll available at: {}", embed_url);
    
    // Apply custom styling
    client.set_custom_css(&poll_id, "
        .heyform-container { background: #f0f0f0; }
        .heyform-question { color: #333; }
    ").await?;
    
    Ok(())
}
```

## API Reference

### Client Creation

```rust
// Use default HeyForm URL (https://app.heyform.net)
let client = HeyFormClient::default()?;

// Or specify a custom URL
let client = HeyFormClient::new("https://your-heyform-instance.com")?;
```

### Authentication

#### Sign Up
```rust
let signup_input = SignUpInput {
    name: "User Name".to_string(),
    email: "user@example.com".to_string(),
    password: "SecurePassword123!".to_string(),
    team_id: None, // Optional: join existing team
    invite_code: None, // Optional: team invite code
};

let success = client.signup(signup_input).await?;
```

#### Login
```rust
let login_input = LoginInput {
    email: "user@example.com".to_string(),
    password: "SecurePassword123!".to_string(),
};

let success = client.login(login_input).await?;
```

### Workspace Management

#### Create Workspace
```rust
let workspace_input = CreateTeamInput {
    name: "My Workspace".to_string(),
    avatar: Some("https://example.com/avatar.jpg".to_string()),
    members: Some(vec!["user@example.com".to_string()]),
};

let workspace_id = client.create_workspace(workspace_input).await?;
```

#### Get Workspaces
```rust
let teams = client.get_teams().await?;
for team in teams {
    println!("Team: {} (ID: {})", team.name, team.id);
}
```

### Form/Poll Management

#### Create Form
```rust
let form_input = CreateFormInput {
    project_id: "project-id".to_string(),
    name: Some("My Poll".to_string()),
    interactive_mode: InteractiveMode::Conversational,
    kind: FormKind::Poll, // Poll, Survey, Form, or Quiz
};

let form_id = client.create_poll(form_input).await?;
```

#### Publish Form
```rust
// Publishes the form and returns embed URL
let embed_url = client.publish_poll(&form_id, None).await?;

// Or specify custom base URL
let embed_url = client.publish_poll(&form_id, Some("https://mysite.com")).await?;
```

#### Apply Custom CSS
```rust
let css = r#"
.heyform-container {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-radius: 12px;
}

.heyform-question {
    color: #ffffff;
    font-family: 'Helvetica Neue', sans-serif;
    font-weight: 600;
}

.heyform-button {
    background-color: #4CAF50;
    border: none;
    color: white;
    padding: 15px 32px;
    border-radius: 8px;
    font-size: 16px;
    cursor: pointer;
    transition: background-color 0.3s;
}

.heyform-button:hover {
    background-color: #45a049;
}
"#;

let success = client.set_custom_css(&form_id, css).await?;
```

#### Get Form Details
```rust
let form = client.get_form(&form_id).await?;
println!("Form: {} ({})", form.name.unwrap_or_default(), form.id);
```

### User Information

```rust
let user = client.get_user().await?;
println!("User: {} <{}>", user.name, user.email);
```

## Types

### Form Types
- `FormKind::Form` - Standard form
- `FormKind::Survey` - Survey form
- `FormKind::Poll` - Poll form
- `FormKind::Quiz` - Quiz form

### Interactive Modes
- `InteractiveMode::Classic` - Traditional form layout
- `InteractiveMode::Conversational` - Chat-like experience

## Error Handling

The SDK uses a custom `Result<T>` type with `HeyFormError`:

```rust
use heyform_sdk::{HeyFormError, Result};

match client.signup(signup_input).await {
    Ok(success) => println!("Success: {}", success),
    Err(HeyFormError::Authentication(msg)) => eprintln!("Auth error: {}", msg),
    Err(HeyFormError::GraphQL(msg)) => eprintln!("GraphQL error: {}", msg),
    Err(HeyFormError::Http(e)) => eprintln!("HTTP error: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Examples

See the `examples/` directory for complete usage examples:

```bash
cargo run --example basic_usage
```

## Requirements

- Rust 1.70.0 or higher
- Tokio async runtime

## License

This project is licensed under the MIT License - see the LICENSE file for details.