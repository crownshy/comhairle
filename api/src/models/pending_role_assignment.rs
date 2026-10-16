use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use schemars::JsonSchema;
use sea_query::{Expr, Func, IntoIden, OnConflict, Order, PostgresQueryBuilder, Query, enum_def};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tracing::warn;
use uuid::Uuid;

use super::permissions::{self, ActorId, ResourceType, assignments};
use super::users::{self, UserIden};
use super::{conversation, organization};
use crate::ComhairleState;
use crate::error::ComhairleError;

pub const INVITE_LIFETIME_DAYS: i64 = 14;
const INVITE_PURPOSE: &str = "role_invite";

#[derive(Debug, Serialize, Deserialize)]
struct InviteClaims {
    purpose: String,
    email: String,
    exp: usize,
}

#[derive(Debug, FromRow)]
#[enum_def(table_name = "pending_role_assignment")]
pub struct PendingRoleAssignment {
    pub id: Uuid,
    pub email: Option<String>,
    pub organization_id: Option<Uuid>,
    pub resource_type: String,
    pub resource_id: Uuid,
    pub role_names: Vec<String>,
    pub invited_by: Option<Uuid>,
    pub grant_reason: String,
    pub expires_at: DateTime<Utc>,
}

pub async fn propose(
    connection: &mut sqlx::PgConnection,
    proposal: PendingRoleAssignment,
) -> Result<Uuid, ComhairleError> {
    use PendingRoleAssignmentIden as P;
    let recipient_column = if proposal.organization_id.is_some() {
        P::OrganizationId
    } else {
        P::Email
    };
    let (sql, values) = Query::insert()
        .into_table(P::Table)
        .columns([
            P::Id,
            P::Email,
            P::OrganizationId,
            P::ResourceType,
            P::ResourceId,
            P::RoleNames,
            P::InvitedBy,
            P::GrantReason,
            P::ExpiresAt,
        ])
        .values([
            proposal.id.into(),
            proposal.email.into(),
            proposal.organization_id.into(),
            proposal.resource_type.into(),
            proposal.resource_id.into(),
            proposal.role_names.into(),
            proposal.invited_by.into(),
            proposal.grant_reason.into(),
            proposal.expires_at.into(),
        ])?
        .on_conflict(
            OnConflict::columns([recipient_column, P::ResourceType, P::ResourceId])
                .update_columns([P::RoleNames, P::InvitedBy, P::GrantReason, P::ExpiresAt])
                .to_owned(),
        )
        .returning_col(P::Id)
        .build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_scalar_with(&sql, values)
        .fetch_one(connection)
        .await?)
}

pub async fn find(
    connection: &mut sqlx::PgConnection,
    id: Uuid,
) -> Result<PendingRoleAssignment, ComhairleError> {
    use PendingRoleAssignmentIden as P;
    let (sql, values) = Query::select()
        .columns([
            P::Id,
            P::Email,
            P::OrganizationId,
            P::ResourceType,
            P::ResourceId,
            P::RoleNames,
            P::InvitedBy,
            P::GrantReason,
            P::ExpiresAt,
        ])
        .from(P::Table)
        .and_where(Expr::col(P::Id).eq(id))
        .and_where(Expr::col(P::ExpiresAt).gt(Utc::now()))
        .lock_exclusive()
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_as_with(&sql, values)
        .fetch_optional(connection)
        .await?
        .ok_or_else(|| ComhairleError::ResourceNotFound("invitation".into()))
}

pub async fn delete(connection: &mut sqlx::PgConnection, id: Uuid) -> Result<(), ComhairleError> {
    use PendingRoleAssignmentIden as P;
    let (sql, values) = Query::delete()
        .from_table(P::Table)
        .and_where(Expr::col(P::Id).eq(id))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(connection).await?;
    Ok(())
}

pub async fn grant(
    connection: &mut sqlx::PgConnection,
    proposal: &PendingRoleAssignment,
    actor: ActorId,
) -> Result<(), ComhairleError> {
    use permissions::ResourcePermissionIden as R;
    let (table, recipient_column) = permissions::permission_table(&proposal.resource_type, actor)?;
    let recipient_id = match actor {
        ActorId::User(id) | ActorId::Group(id) => id,
    };
    for role in &proposal.role_names {
        let mut columns = vec![
            recipient_column.clone(),
            R::RoleName.into_iden(),
            R::GrantedBy.into_iden(),
            R::GrantReason.into_iden(),
        ];
        let mut values = vec![
            recipient_id.into(),
            role.clone().into(),
            proposal.invited_by.into(),
            proposal.grant_reason.clone().into(),
        ];
        if proposal.resource_type != ResourceType::System.as_ref() {
            columns.push(R::ResourceId.into_iden());
            values.push(proposal.resource_id.into());
        }
        let (sql, values) = Query::insert()
            .into_table(table.clone())
            .columns(columns)
            .values(values)?
            .on_conflict(OnConflict::new().do_nothing().to_owned())
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values)
            .execute(&mut *connection)
            .await?;
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InviteRequest {
    pub emails: Vec<String>,
    pub roles: Vec<String>,
    pub grant_reason: String,
}

pub fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

fn normalize_unique(emails: &[String]) -> Vec<String> {
    let mut unique: Vec<String> = Vec::new();
    for email in emails.iter().map(|email| normalize_email(email)) {
        if !email.is_empty() && !unique.contains(&email) {
            unique.push(email);
        }
    }
    unique
}

/// A signed token proving the holder received an invitation sent to `email`.
pub fn invite_token(secret: &str, email: &str) -> Result<String, ComhairleError> {
    let claims = InviteClaims {
        purpose: INVITE_PURPOSE.to_string(),
        email: normalize_email(email),
        exp: (Utc::now() + Duration::days(INVITE_LIFETIME_DAYS)).timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|error| ComhairleError::AuthJWTError(error.to_string()))
}

/// The email address an invite token was issued for, if the token is genuine and unexpired.
pub fn email_from_invite_token(secret: &str, token: &str) -> Option<String> {
    let data = decode::<InviteClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .ok()?;
    (data.claims.purpose == INVITE_PURPOSE).then_some(data.claims.email)
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct RegisteredEmail {
    pub email: String,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct EmailLookup {
    pub registered: Vec<RegisteredEmail>,
    pub unregistered: Vec<String>,
}

/// Splits the normalized addresses from `emails` into those with and without an account.
pub async fn lookup_emails(
    state: &Arc<ComhairleState>,
    emails: &[String],
) -> Result<EmailLookup, ComhairleError> {
    let candidates = normalize_unique(emails);
    if candidates.is_empty() {
        return Err(ComhairleError::BadRequest(
            "Enter at least one email address".into(),
        ));
    }
    for email in &candidates {
        email
            .parse::<lettre::Address>()
            .map_err(|_| ComhairleError::BadRequest(format!("Invalid email address: {email}")))?;
    }
    let (sql, values) = Query::select()
        .expr(Func::lower(Expr::col(UserIden::Email)))
        .column(UserIden::Id)
        .from(UserIden::Table)
        .and_where(Expr::expr(Func::lower(Expr::col(UserIden::Email))).is_in(candidates.clone()))
        .build_sqlx(PostgresQueryBuilder);
    let accounts: Vec<(String, Uuid)> = sqlx::query_as_with(&sql, values)
        .fetch_all(&state.db)
        .await?;

    let mut lookup = EmailLookup {
        registered: Vec::new(),
        unregistered: Vec::new(),
    };
    for email in candidates {
        match accounts.iter().find(|(account, _)| *account == email) {
            Some((_, user_id)) => lookup.registered.push(RegisteredEmail {
                email,
                user_id: *user_id,
            }),
            None => lookup.unregistered.push(email),
        }
    }
    Ok(lookup)
}

async fn resource_label(
    state: &Arc<ComhairleState>,
    resource_type: ResourceType,
    resource_id: Uuid,
) -> Result<String, ComhairleError> {
    let label = match resource_type {
        ResourceType::Organization => organization::get_by_id(&state.db, &resource_id)
            .await
            .map(|organization| organization.name),
        ResourceType::Conversation => {
            conversation::get_localised_by_id(&state.db, &resource_id, "en")
                .await
                .map(|conversation| conversation.title)
        }
        ResourceType::System => Ok("the platform".to_string()),
    };
    label
}

/// Records the proposed roles for each unregistered email and emails them an invitation.
/// Returns the addresses that were invited; addresses that already have an account are skipped.
pub async fn invite(
    state: &Arc<ComhairleState>,
    inviter_id: Uuid,
    resource_type: ResourceType,
    resource_id: Uuid,
    request: InviteRequest,
) -> Result<Vec<String>, ComhairleError> {
    let catalogue = assignments::role_catalogue(resource_type);
    if request.roles.is_empty() || request.roles.iter().any(|role| !catalogue.contains(role)) {
        return Err(ComhairleError::BadRequest(
            "Roles must belong to this resource type".into(),
        ));
    }
    let emails = lookup_emails(state, &request.emails).await?.unregistered;
    let inviter_name = users::get_user_by_id(&inviter_id, &state.db)
        .await?
        .username
        .unwrap_or_else(|| "A Comhairle administrator".to_string());
    let label = resource_label(state, resource_type, resource_id).await?;

    let mut transaction = state.db.begin().await?;
    permissions::lock_permission_mutation(&mut transaction).await?;
    assignments::validate_target(&mut transaction, resource_type, resource_id).await?;
    assignments::authorize(
        &mut transaction,
        resource_type,
        resource_id,
        inviter_id,
        "grant_permission",
    )
    .await?;
    for email in &emails {
        propose(
            &mut transaction,
            PendingRoleAssignment {
                id: Uuid::new_v4(),
                email: Some(email.clone()),
                organization_id: None,
                resource_type: resource_type.to_string(),
                resource_id,
                role_names: request.roles.clone(),
                invited_by: Some(inviter_id),
                grant_reason: request.grant_reason.clone(),
                expires_at: Utc::now() + Duration::days(INVITE_LIFETIME_DAYS),
            },
        )
        .await?;
    }
    transaction.commit().await?;

    for email in &emails {
        send_invitation(
            state,
            email,
            &inviter_name,
            &label,
            resource_type,
            &request.roles,
        )?;
    }
    Ok(emails)
}

fn send_invitation(
    state: &Arc<ComhairleState>,
    email: &str,
    inviter_name: &str,
    resource_label: &str,
    resource_type: ResourceType,
    roles: &[String],
) -> Result<(), ComhairleError> {
    let signup_link = format!(
        "{}/auth/signup?invite={}&email={}",
        state.config.domain,
        invite_token(&state.config.jwt_secret, email)?,
        urlencoding::encode(email),
    );
    let subject = "You have been invited to Comhairle";
    state.mailer.send_email(
        email,
        subject,
        "platform_invitation.html",
        minijinja::context! {
            subject,
            inviter_name,
            resource_label,
            resource_type => resource_type.as_ref(),
            roles,
            signup_link,
            expires_in_days => INVITE_LIFETIME_DAYS,
        },
        None,
    )
}

/// Grants the roles proposed for `email` to `user_id`. Callers must already know the user
/// controls the address. Roles whose inviter has since lost the right to grant them are dropped.
pub async fn claim(state: &Arc<ComhairleState>, user_id: Uuid, email: &str) {
    if let Err(error) = try_claim(state, user_id, &normalize_email(email)).await {
        warn!("Could not claim pending role assignments for {user_id}: {error}");
    }
}

async fn try_claim(
    state: &Arc<ComhairleState>,
    user_id: Uuid,
    email: &str,
) -> Result<(), ComhairleError> {
    use PendingRoleAssignmentIden as P;
    let (sql, values) = Query::select()
        .column(P::Id)
        .from(P::Table)
        .and_where(Expr::col(P::Email).eq(email))
        .and_where(Expr::col(P::OrganizationId).is_null())
        .and_where(Expr::col(P::ExpiresAt).gt(Utc::now()))
        .order_by(P::Id, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);
    let pending: Vec<Uuid> = sqlx::query_scalar_with(&sql, values)
        .fetch_all(&state.db)
        .await?;

    for id in pending {
        if let Err(error) = grant_pending(state, user_id, id).await {
            warn!("Skipped pending role {id}: {error}");
        }
    }
    let (sql, values) = Query::delete()
        .from_table(P::Table)
        .and_where(Expr::col(P::Email).eq(email))
        .and_where(Expr::col(P::ExpiresAt).lte(Utc::now()))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(&state.db).await?;
    Ok(())
}

async fn grant_pending(
    state: &Arc<ComhairleState>,
    user_id: Uuid,
    id: Uuid,
) -> Result<(), ComhairleError> {
    let mut transaction = state.db.begin().await?;
    permissions::lock_permission_mutation(&mut transaction).await?;
    let assignment = match find(&mut transaction, id).await {
        Ok(assignment) => assignment,
        Err(ComhairleError::ResourceNotFound(_)) => return Ok(()),
        Err(error) => return Err(error),
    };
    if assignment.organization_id.is_some() {
        return Err(ComhairleError::UserNotAuthorized);
    }
    let resource_type: ResourceType = assignment
        .resource_type
        .parse()
        .map_err(|_| ComhairleError::BadRequest("Unknown permission resource type".into()))?;

    let authorization = match assignment.invited_by {
        Some(inviter_id) => {
            assignments::authorize(
                &mut transaction,
                resource_type,
                assignment.resource_id,
                inviter_id,
                "grant_permission",
            )
            .await
        }
        None => Err(ComhairleError::UserNotAuthorized),
    };
    match authorization {
        Ok(()) => {
            assignments::validate_target(&mut transaction, resource_type, assignment.resource_id)
                .await?;
            grant(&mut transaction, &assignment, ActorId::User(user_id)).await?;
        }
        Err(ComhairleError::UserNotAuthorized) => {
            warn!(
                "Dropped pending role {} because its inviter no longer has authority",
                assignment.id
            );
        }
        Err(error) => return Err(error),
    }
    delete(&mut transaction, assignment.id).await?;
    transaction
        .commit()
        .await
        .map_err(permissions::permission_database_error)?;
    permissions::invalidate_actor_roles(
        state,
        resource_type.as_ref(),
        &assignment.resource_id,
        ActorId::User(user_id),
    )
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use axum::body::Body;
    use serde_json::json;

    use super::*;
    use crate::mailer::{MockComhairleMailer, SentEmail};
    use crate::models::model_test_helpers::{
        get_random_conversation_id, get_random_organization_id, setup_app_and_session_with_state,
    };
    use crate::routes::auth::{EmailLinkClaims, generate_jwt};
    use crate::test_helpers::{TEST_PASSWORD, UserSession, test_state};

    struct Fixture {
        state: Arc<ComhairleState>,
        app: crate::App,
        admin: UserSession,
        organization_id: Uuid,
        sent: Arc<Mutex<Vec<SentEmail>>>,
    }

    async fn fixture(pool: sqlx::PgPool) -> Result<Fixture, Box<dyn std::error::Error>> {
        let sent: Arc<Mutex<Vec<SentEmail>>> = Arc::default();
        let state = Arc::new(
            test_state()
                .db(pool)
                .mailer(Arc::new(MockComhairleMailer::recording(sent.clone())))
                .call()?,
        );
        let (app, mut admin) = setup_app_and_session_with_state(state.clone()).await?;
        let organization_id = get_random_organization_id(&app, &mut admin).await?;
        Ok(Fixture {
            state,
            app,
            admin,
            organization_id,
            sent,
        })
    }

    fn invite_request(emails: &[&str]) -> InviteRequest {
        InviteRequest {
            emails: emails.iter().map(|email| email.to_string()).collect(),
            roles: vec!["organization_member".into()],
            grant_reason: "Test".into(),
        }
    }

    async fn roles_of(fixture: &Fixture, user_id: Uuid) -> Vec<String> {
        assignments::load(
            &fixture.state,
            ResourceType::Organization,
            fixture.organization_id,
            fixture.admin.id.unwrap(),
            assignments::Recipient::User(user_id),
        )
        .await
        .unwrap()
        .roles
    }

    async fn signup(
        fixture: &Fixture,
        email: &str,
        invite_token: Option<String>,
    ) -> Result<UserSession, Box<dyn std::error::Error>> {
        let mut session = UserSession::new("invitee", TEST_PASSWORD, email);
        let body = json!({
            "username": "invitee",
            "password": TEST_PASSWORD,
            "email": email,
            "invite_token": invite_token,
        });
        let (status, _, _) = session
            .post(&fixture.app, "/auth/signup", Body::from(body.to_string()))
            .await?;
        assert!(status.is_success());
        let me = users::get_user_by_email(email, &fixture.state.db).await?;
        session.id = Some(me.id);
        Ok(session)
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn conversation_and_system_roles_are_claimed_for_their_resources(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut fixture = fixture(pool).await?;
        let conversation_id = get_random_conversation_id(&fixture.app, &mut fixture.admin).await?;
        let resources = [
            (ResourceType::Conversation, conversation_id, "observer"),
            (
                ResourceType::System,
                permissions::SYSTEM_RESOURCE_ID,
                "translator",
            ),
        ];
        for (resource_type, resource_id, role) in resources {
            invite(
                &fixture.state,
                fixture.admin.id.unwrap(),
                resource_type,
                resource_id,
                InviteRequest {
                    emails: vec!["multi-resource@example.com".into()],
                    roles: vec![role.into()],
                    grant_reason: "Test".into(),
                },
            )
            .await?;
        }
        let token = invite_token(
            &fixture.state.config.jwt_secret,
            "multi-resource@example.com",
        )?;
        let invitee = signup(&fixture, "multi-resource@example.com", Some(token)).await?;
        for (resource_type, resource_id, role) in resources {
            let snapshot = assignments::load(
                &fixture.state,
                resource_type,
                resource_id,
                fixture.admin.id.unwrap(),
                assignments::Recipient::User(invitee.id.unwrap()),
            )
            .await?;
            assert_eq!(snapshot.roles, vec![role.to_string()]);
        }
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn expired_proposals_are_not_claimed(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = fixture(pool).await?;
        propose(
            &mut *fixture.state.db.acquire().await?,
            PendingRoleAssignment {
                id: Uuid::new_v4(),
                email: Some("expired@example.com".into()),
                organization_id: None,
                resource_type: ResourceType::Organization.to_string(),
                resource_id: fixture.organization_id,
                role_names: vec!["organization_member".into()],
                invited_by: fixture.admin.id,
                grant_reason: "Test".into(),
                expires_at: Utc::now() - Duration::days(1),
            },
        )
        .await?;
        let token = invite_token(&fixture.state.config.jwt_secret, "expired@example.com")?;
        let invitee = signup(&fixture, "expired@example.com", Some(token)).await?;
        assert!(roles_of(&fixture, invitee.id.unwrap()).await.is_empty());
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn only_unregistered_emails_are_invited(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = fixture(pool).await?;
        let mut colleague = UserSession::new("colleague", TEST_PASSWORD, "colleague@example.com");
        colleague.signup(&fixture.app).await?;

        let lookup = lookup_emails(
            &fixture.state,
            &[
                "Colleague@Example.com".to_string(),
                "new@example.com".to_string(),
            ],
        )
        .await?;
        assert_eq!(lookup.registered.len(), 1);
        assert_eq!(lookup.registered[0].email, "colleague@example.com");
        assert_eq!(Some(lookup.registered[0].user_id), colleague.id);
        assert_eq!(lookup.unregistered, vec!["new@example.com".to_string()]);

        let invited = invite(
            &fixture.state,
            fixture.admin.id.unwrap(),
            ResourceType::Organization,
            fixture.organization_id,
            invite_request(&[
                "Colleague@Example.com",
                "New@Example.com",
                "new@example.com",
            ]),
        )
        .await?;

        assert_eq!(invited, vec!["new@example.com".to_string()]);
        let sent = fixture.sent.lock().unwrap();
        let invitations: Vec<_> = sent
            .iter()
            .filter(|email| email.template == "platform_invitation.html")
            .collect();
        assert_eq!(invitations.len(), 1);
        assert_eq!(invitations[0].to, "new@example.com");
        assert!(invitations[0].context.contains("/auth/signup?invite="));
        assert!(invitations[0].context.contains("organization_member"));
        assert!(invitations[0].context.contains("resource_label"));
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn signing_up_with_the_invite_token_grants_the_proposed_roles(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = fixture(pool).await?;
        invite(
            &fixture.state,
            fixture.admin.id.unwrap(),
            ResourceType::Organization,
            fixture.organization_id,
            invite_request(&["new@example.com"]),
        )
        .await?;

        let token = invite_token(&fixture.state.config.jwt_secret, "new@example.com")?;
        let invitee = signup(&fixture, "new@example.com", Some(token)).await?;

        assert_eq!(
            roles_of(&fixture, invitee.id.unwrap()).await,
            vec!["organization_member".to_string()]
        );
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn a_token_for_another_address_grants_nothing_until_the_email_is_verified(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = fixture(pool).await?;
        invite(
            &fixture.state,
            fixture.admin.id.unwrap(),
            ResourceType::Organization,
            fixture.organization_id,
            invite_request(&["new@example.com"]),
        )
        .await?;

        let wrong_token = invite_token(&fixture.state.config.jwt_secret, "other@example.com")?;
        let mut invitee = signup(&fixture, "new@example.com", Some(wrong_token)).await?;
        let invitee_id = invitee.id.unwrap();
        assert!(roles_of(&fixture, invitee_id).await.is_empty());

        let user = users::get_user_by_id(&invitee_id, &fixture.state.db).await?;
        let verify_token = generate_jwt()
            .user(&user)
            .secret(&fixture.state.config.jwt_secret)
            .custom_claims(EmailLinkClaims {
                email: user.email.clone(),
            })
            .call()?;
        let (status, _, _) = invitee
            .verify_email_token(&fixture.app, verify_token)
            .await?;
        assert!(status.is_success());

        assert_eq!(
            roles_of(&fixture, invitee_id).await,
            vec!["organization_member".to_string()]
        );
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn roles_from_an_inviter_without_authority_are_not_granted(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = fixture(pool).await?;
        let mut bystander = UserSession::new("bystander", TEST_PASSWORD, "bystander@example.com");
        bystander.signup(&fixture.app).await?;
        propose(
            &mut *fixture.state.db.acquire().await?,
            PendingRoleAssignment {
                id: Uuid::new_v4(),
                email: Some("new@example.com".into()),
                organization_id: None,
                resource_type: ResourceType::Organization.to_string(),
                resource_id: fixture.organization_id,
                role_names: vec!["organization_admin".into()],
                invited_by: bystander.id,
                grant_reason: "Test".into(),
                expires_at: Utc::now() + Duration::days(INVITE_LIFETIME_DAYS),
            },
        )
        .await?;

        let token = invite_token(&fixture.state.config.jwt_secret, "new@example.com")?;
        let invitee = signup(&fixture, "new@example.com", Some(token)).await?;

        assert!(roles_of(&fixture, invitee.id.unwrap()).await.is_empty());
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn invites_require_the_right_to_grant_and_valid_roles(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = fixture(pool).await?;
        let mut bystander = UserSession::new("bystander", TEST_PASSWORD, "bystander@example.com");
        bystander.signup(&fixture.app).await?;

        let unauthorized = invite(
            &fixture.state,
            bystander.id.unwrap(),
            ResourceType::Organization,
            fixture.organization_id,
            invite_request(&["new@example.com"]),
        )
        .await;
        assert!(matches!(
            unauthorized,
            Err(ComhairleError::UserNotAuthorized)
        ));

        let wrong_role = invite(
            &fixture.state,
            fixture.admin.id.unwrap(),
            ResourceType::Organization,
            fixture.organization_id,
            InviteRequest {
                roles: vec!["observer".into()],
                ..invite_request(&["new@example.com"])
            },
        )
        .await;
        assert!(matches!(wrong_role, Err(ComhairleError::BadRequest(_))));
        Ok(())
    }

    #[test]
    fn invite_tokens_only_validate_with_the_signing_secret() {
        let token = invite_token("secret", "  New@Example.com ").unwrap();
        assert_eq!(
            email_from_invite_token("secret", &token).as_deref(),
            Some("new@example.com")
        );
        assert_eq!(email_from_invite_token("other", &token), None);
        assert_eq!(email_from_invite_token("secret", "garbage"), None);
    }

    #[test]
    fn emails_are_normalized_and_deduplicated() {
        let emails = vec![
            " A@Example.com".to_string(),
            "a@example.com".to_string(),
            "".to_string(),
            "b@example.com".to_string(),
        ];
        assert_eq!(
            normalize_unique(&emails),
            vec!["a@example.com", "b@example.com"]
        );
    }
}
