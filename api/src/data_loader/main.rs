use clap::Parser;
use comhairle::hash_pw;
use comhairle::models::conversation::ConversationIden;
use comhairle::models::{conversation::Conversation, users::UserIden};

use comhairle::models::users::User;
use comhairle::models::workflow::{Workflow, WorkflowIden};
use comhairle::models::workflow_step::{WorkflowStep, WorkflowStepIden};
use sea_query::{PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use sqlx::PgPool;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use tracing::info;

use comhairle::db::{run_migrations, setup_db};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'f', help = "The fixture file to load")]
    pub fixture_file: std::path::PathBuf,

    #[arg(
        short = 'd',
        help = "Optionally, delete exisitng content in the database before loading"
    )]
    pub drop_db: Option<bool>,
}

async fn empty_db(pool: &PgPool) -> Result<(), Box<dyn Error>> {
    // Disconnect any active connections to the target database (Postgres-specific)
    for table in [
        "user_participation",
        "user_progress",
        "workflow_step",
        "workflow",
        "conversation",
        "comhairle_user",
    ] {
        let query = format!("DELETE FROM {table}");
        sqlx::query(&query).bind(table).execute(pool).await?;
    }

    Ok(())
}

fn load_fixtures(file_path: &std::path::PathBuf) -> Result<Fixtures, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let fixtures: Fixtures = serde_json::from_reader(reader)?;
    Ok(fixtures)
}

#[derive(Deserialize, Debug)]
struct Fixtures {
    users: Option<Vec<User>>,
    conversations: Option<Vec<Conversation>>,
    workflows: Option<Vec<Workflow>>,
    workflow_steps: Option<Vec<WorkflowStep>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let config = comhairle::config::load()?;
    let args = Args::parse();

    let fixtures = load_fixtures(&args.fixture_file)?;

    let db = setup_db(&config.database_url).await?;

    // Run any pending migrations
    run_migrations(&db).await?;

    // If we have been asked to drop the db
    // drop and recreate it
    if args.drop_db.unwrap_or(false) {
        empty_db(&db).await?;
    }

    if let Some(users) = &fixtures.users {
        info!("Creating users");
        for user in users {
            let password = user.password.clone().map(|p| hash_pw(&p).unwrap());
            let (sql, values) = Query::insert()
                .into_table(UserIden::Table)
                .columns([
                    UserIden::Id,
                    UserIden::Username,
                    UserIden::Password,
                    UserIden::AuthType,
                    UserIden::AvatarUrl,
                    UserIden::Email,
                ])
                .values([
                    user.id.into(),
                    user.username.clone().into(),
                    password.into(),
                    user.auth_type.clone().into(),
                    user.avatar_url.clone().into(),
                    user.email.clone().into(),
                ])
                .unwrap()
                .build_sqlx(PostgresQueryBuilder);

            sqlx::query_with(&sql, values).execute(&db).await?;
        }
    }

    if let Some(conversations) = &fixtures.conversations {
        info!("Creating conversations");
        for conversation in conversations {
            let (sql, values) = Query::insert()
                .into_table(ConversationIden::Table)
                .columns([
                    ConversationIden::Id,
                    ConversationIden::Title,
                    ConversationIden::ShortDescription,
                    ConversationIden::Description,
                    ConversationIden::VideoUrl,
                    ConversationIden::ImageUrl,
                    ConversationIden::Tags,
                    ConversationIden::IsPublic,
                    ConversationIden::IsComplete,
                    ConversationIden::OwnerId,
                    ConversationIden::IsInviteOnly,
                    ConversationIden::Slug,
                    ConversationIden::CreatedAt,
                    ConversationIden::UpdatedAt,
                ])
                .values([
                    conversation.id.into(),
                    conversation.title.clone().into(),
                    conversation.short_description.clone().into(),
                    conversation.description.clone().into(),
                    conversation.video_url.clone().into(),
                    conversation.image_url.clone().into(),
                    conversation.tags.clone().into(),
                    conversation.is_public.clone().into(),
                    conversation.is_complete.clone().into(),
                    conversation.owner_id.clone().into(),
                    conversation.is_invite_only.clone().into(),
                    conversation.slug.clone().into(),
                    conversation.created_at.clone().into(),
                    conversation.updated_at.clone().into(),
                ])
                .unwrap()
                .build_sqlx(PostgresQueryBuilder);

            sqlx::query_with(&sql, values).execute(&db).await?;
        }
    }

    if let Some(workflows) = &fixtures.workflows {
        info!("Creating workflows");
        for workflow in workflows {
            let (sql, values) = Query::insert()
                .into_table(WorkflowIden::Table)
                .columns([
                    WorkflowIden::Id,
                    WorkflowIden::ConversationId,
                    WorkflowIden::Name,
                    WorkflowIden::Description,
                    WorkflowIden::IsActive,
                    WorkflowIden::IsPublic,
                    WorkflowIden::OwnerId,
                    WorkflowIden::CreatedAt,
                    WorkflowIden::UpdatedAt,
                ])
                .values([
                    workflow.id.clone().into(),
                    workflow.conversation_id.to_owned().clone().into(),
                    workflow.name.to_owned().clone().into(),
                    workflow.description.to_owned().clone().into(),
                    workflow.is_active.to_owned().clone().into(),
                    workflow.is_public.to_owned().clone().into(),
                    workflow.owner_id.to_owned().clone().into(),
                    workflow.created_at.to_owned().clone().into(),
                    workflow.updated_at.to_owned().clone().into(),
                ])
                .unwrap()
                .build_sqlx(PostgresQueryBuilder);
            sqlx::query_with(&sql, values).execute(&db).await?;
        }
    }

    if let Some(workflow_steps) = &fixtures.workflow_steps {
        info!("Creating workflow steps");
        for workflow_step in workflow_steps {
            let (sql, values) = Query::insert()
                .into_table(WorkflowStepIden::Table)
                .columns([
                    WorkflowStepIden::Id,
                    WorkflowStepIden::WorkflowId,
                    WorkflowStepIden::Name,
                    WorkflowStepIden::StepOrder,
                    WorkflowStepIden::ActivationRule,
                    WorkflowStepIden::Description,
                    WorkflowStepIden::IsOffline,
                    WorkflowStepIden::ToolConfig,
                    WorkflowStepIden::CreatedAt,
                    WorkflowStepIden::UpdatedAt,
                ])
                .values([
                    workflow_step.id.clone().into(),
                    workflow_step.workflow_id.into(),
                    workflow_step.name.clone().into(),
                    workflow_step.step_order.clone().into(),
                    serde_json::to_value(&workflow_step.activation_rule)
                        .unwrap()
                        .into(),
                    workflow_step.description.to_owned().clone().into(),
                    workflow_step.is_offline.to_owned().clone().into(),
                    serde_json::to_value(&workflow_step.tool_config)
                        .unwrap()
                        .into(),
                    workflow_step.created_at.to_owned().clone().into(),
                    workflow_step.updated_at.to_owned().clone().into(),
                ])
                .unwrap()
                .build_sqlx(PostgresQueryBuilder);
            sqlx::query_with(&sql, values).execute(&db).await?;
        }
    }
    Ok(())
}
