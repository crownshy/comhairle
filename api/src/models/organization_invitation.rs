use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};
use schemars::JsonSchema;
use sea_query::{Alias, Cond, Expr, JoinType, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::Serialize;
use sqlx::FromRow;
use tracing::warn;
use uuid::Uuid;

use super::conversation;
use super::organization::{self, OrganizationIden};
use super::pending_role_assignment::{self, PendingRoleAssignment};
use super::permissions::conversation::{
    GroupPermissionIden, Role as ConversationRole, UserPermissionIden,
};
use super::permissions::{self, ActorId, ResourceType, assignments};
use super::user_group::{self, OrganizationGroup};
use super::users::{self, UserIden};
use crate::ComhairleState;
use crate::error::ComhairleError;

const INVITATION_LIFETIME_DAYS: i64 = 14;

/// An organization that has a contact email to invite.
#[derive(Debug, FromRow)]
pub struct ContactableOrganization {
    pub id: Uuid,
    pub name: String,
    pub contact_email: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct InvitationDetails {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub conversation_title: String,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub roles: Vec<String>,
    pub expires_at: DateTime<Utc>,
    pub can_accept: bool,
}

/// The organization whose members group is `group_id`, if it has a contact email.
pub async fn contactable_organization(
    connection: &mut sqlx::PgConnection,
    group_id: Uuid,
) -> Result<Option<ContactableOrganization>, ComhairleError> {
    let (sql, values) = Query::select()
        .columns([
            OrganizationIden::Id,
            OrganizationIden::Name,
            OrganizationIden::ContactEmail,
        ])
        .from(OrganizationIden::Table)
        .and_where(Expr::col(OrganizationIden::UserGroupId).eq(group_id))
        .and_where(Expr::col(OrganizationIden::ContactEmail).is_not_null())
        .build_sqlx(PostgresQueryBuilder);
    let organization: Option<ContactableOrganization> = sqlx::query_as_with(&sql, values)
        .fetch_optional(connection)
        .await?;
    Ok(organization.filter(|organization| !organization.contact_email.trim().is_empty()))
}

/// Stores the proposed roles, replacing any earlier proposal for the same conversation and
/// organization, and returns the invitation id.
pub async fn propose(
    connection: &mut sqlx::PgConnection,
    conversation_id: Uuid,
    organization_id: Uuid,
    roles: &[String],
    grant_reason: &str,
    inviter_id: Uuid,
) -> Result<Uuid, ComhairleError> {
    pending_role_assignment::propose(
        connection,
        PendingRoleAssignment {
            id: Uuid::new_v4(),
            email: None,
            organization_id: Some(organization_id),
            resource_type: ResourceType::Conversation.to_string(),
            resource_id: conversation_id,
            role_names: roles.to_vec(),
            grant_reason: grant_reason.to_owned(),
            invited_by: Some(inviter_id),
            expires_at: Utc::now() + Duration::days(INVITATION_LIFETIME_DAYS),
        },
    )
    .await
}

async fn conversation_title(state: &Arc<ComhairleState>, conversation_id: Uuid) -> String {
    conversation::get_localised_by_id(&state.db, &conversation_id, "en")
        .await
        .map(|conversation| conversation.title)
        .unwrap_or_else(|error| {
            warn!("Could not load the conversation title: {error}");
            "a conversation".to_string()
        })
}

/// Emails the organization's contact address a link to accept the proposed access.
pub async fn send_invitation(
    state: &Arc<ComhairleState>,
    invitation_id: Uuid,
    organization: &ContactableOrganization,
    conversation_id: Uuid,
    inviter_id: Uuid,
) {
    let inviter_name = users::get_user_by_id(&inviter_id, &state.db)
        .await
        .ok()
        .and_then(|user| user.username)
        .unwrap_or_else(|| "A Comhairle administrator".to_string());
    let title = conversation_title(state, conversation_id).await;
    let subject = format!("{} has been invited to join {title}", organization.name);
    let sent = state.mailer.send_email(
        &organization.contact_email,
        &subject,
        "organization_conversation_invitation.html",
        minijinja::context! {
            subject => &subject,
            organization_name => &organization.name,
            conversation_title => title,
            inviter_name,
            accept_link => format!(
                "{}/organization-invitations/{invitation_id}",
                state.config.domain
            ),
            expires_in_days => INVITATION_LIFETIME_DAYS,
        },
        None,
    );
    if let Err(error) = sent {
        warn!("Could not send the organization conversation invitation: {error}");
    }
}

async fn find(
    connection: &mut sqlx::PgConnection,
    id: Uuid,
) -> Result<PendingRoleAssignment, ComhairleError> {
    let invitation = pending_role_assignment::find(connection, id).await?;
    if invitation.organization_id.is_none()
        || invitation.resource_type != ResourceType::Conversation.as_ref()
    {
        return Err(ComhairleError::ResourceNotFound(
            "organization invitation".into(),
        ));
    }
    Ok(invitation)
}

async fn is_organization_admin(
    state: &Arc<ComhairleState>,
    organization_id: Uuid,
    user_id: Uuid,
) -> Result<bool, ComhairleError> {
    Ok(
        user_group::organization_groups_of_user(&state.db, organization_id, user_id)
            .await?
            .contains(&OrganizationGroup::Admins),
    )
}

/// What an invitation offers, and whether `caller_id` is an administrator who may accept it.
pub async fn details(
    state: &Arc<ComhairleState>,
    id: Uuid,
    caller_id: Uuid,
) -> Result<InvitationDetails, ComhairleError> {
    let invitation = find(&mut *state.db.acquire().await?, id).await?;
    let organization_id = invitation
        .organization_id
        .ok_or(ComhairleError::UserNotAuthorized)?;
    let organization_name = organization::get_by_id(&state.db, &organization_id)
        .await?
        .name;
    Ok(InvitationDetails {
        id: invitation.id,
        conversation_id: invitation.resource_id,
        conversation_title: conversation_title(state, invitation.resource_id).await,
        organization_id,
        organization_name,
        roles: invitation.role_names,
        expires_at: invitation.expires_at,
        can_accept: is_organization_admin(state, organization_id, caller_id).await?,
    })
}

/// Grants the proposed roles to the organization's members group and tells the conversation
/// administrators. Only an administrator of the organization may accept, and the roles are
/// dropped if the inviter has since lost the right to grant them.
pub async fn accept(
    state: &Arc<ComhairleState>,
    id: Uuid,
    caller_id: Uuid,
) -> Result<InvitationDetails, ComhairleError> {
    let details = details(state, id, caller_id).await?;
    let mut transaction = state.db.begin().await?;
    permissions::lock_permission_mutation(&mut transaction).await?;
    let invitation = find(&mut transaction, id).await?;
    let organization_id = invitation
        .organization_id
        .ok_or(ComhairleError::UserNotAuthorized)?;
    let membership = Alias::new("user_group_member");
    let (sql, values) = Query::select()
        .expr(Expr::exists(
            Query::select()
                .expr(Expr::val(1))
                .from(membership.clone())
                .join(
                    JoinType::InnerJoin,
                    OrganizationIden::Table,
                    Expr::col((membership.clone(), Alias::new("group_id")))
                        .equals((OrganizationIden::Table, OrganizationIden::AdminGroupId)),
                )
                .and_where(Expr::col((membership, Alias::new("user_id"))).eq(caller_id))
                .and_where(Expr::col(OrganizationIden::Id).eq(organization_id))
                .to_owned(),
        ))
        .build_sqlx(PostgresQueryBuilder);
    let is_admin: bool = sqlx::query_scalar_with(&sql, values)
        .fetch_one(&mut *transaction)
        .await?;
    if !is_admin {
        return Err(ComhairleError::UserNotAuthorized);
    }
    let inviter_id = invitation
        .invited_by
        .ok_or(ComhairleError::UserNotAuthorized)?;

    assignments::validate_target(
        &mut transaction,
        ResourceType::Conversation,
        invitation.resource_id,
    )
    .await?;
    assignments::authorize(
        &mut transaction,
        ResourceType::Conversation,
        invitation.resource_id,
        inviter_id,
        "grant_permission",
    )
    .await?;
    let (sql, values) = Query::select()
        .column(OrganizationIden::UserGroupId)
        .from(OrganizationIden::Table)
        .and_where(Expr::col(OrganizationIden::Id).eq(organization_id))
        .build_sqlx(PostgresQueryBuilder);
    let group_id: Uuid = sqlx::query_scalar_with(&sql, values)
        .fetch_one(&mut *transaction)
        .await?;
    pending_role_assignment::grant(&mut transaction, &invitation, ActorId::Group(group_id)).await?;
    pending_role_assignment::delete(&mut transaction, id).await?;
    transaction
        .commit()
        .await
        .map_err(permissions::permission_database_error)?;
    permissions::invalidate_actor_roles(
        state,
        ResourceType::Conversation.as_ref(),
        &invitation.resource_id,
        ActorId::Group(group_id),
    )
    .await?;

    notify_conversation_admins(state, &details).await;
    Ok(details)
}

async fn notify_conversation_admins(state: &Arc<ComhairleState>, details: &InvitationDetails) {
    let membership = Alias::new("user_group_member");
    let group_id = Alias::new("group_id");
    let user_id = Alias::new("user_id");
    let direct = Query::select()
        .expr(Expr::val(1))
        .from(UserPermissionIden::Table)
        .and_where(
            Expr::col((UserPermissionIden::Table, UserPermissionIden::UserId))
                .equals((UserIden::Table, UserIden::Id)),
        )
        .and_where(Expr::col(UserPermissionIden::ResourceId).eq(details.conversation_id))
        .and_where(Expr::col(UserPermissionIden::RoleName).eq(ConversationRole::Admin.as_ref()))
        .to_owned();
    let inherited = Query::select()
        .expr(Expr::val(1))
        .from(GroupPermissionIden::Table)
        .join(
            JoinType::InnerJoin,
            membership.clone(),
            Expr::col((membership.clone(), group_id))
                .equals((GroupPermissionIden::Table, GroupPermissionIden::GroupId)),
        )
        .and_where(Expr::col((membership, user_id)).equals((UserIden::Table, UserIden::Id)))
        .and_where(Expr::col(GroupPermissionIden::ResourceId).eq(details.conversation_id))
        .and_where(Expr::col(GroupPermissionIden::RoleName).eq(ConversationRole::Admin.as_ref()))
        .to_owned();
    let (sql, values) = Query::select()
        .distinct()
        .column(UserIden::Email)
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Email).is_not_null())
        .cond_where(
            Cond::any()
                .add(Expr::exists(direct))
                .add(Expr::exists(inherited)),
        )
        .build_sqlx(PostgresQueryBuilder);
    let admin_emails: Result<Vec<String>, _> = sqlx::query_scalar_with(&sql, values)
        .fetch_all(&state.db)
        .await;
    let admin_emails = match admin_emails {
        Ok(emails) => emails,
        Err(error) => {
            warn!("Could not load the conversation administrators: {error}");
            return;
        }
    };

    let subject = format!(
        "{} has been added to {}",
        details.organization_name, details.conversation_title
    );
    for email in admin_emails {
        let sent = state.mailer.send_email(
            &email,
            &subject,
            "organization_added_to_conversation.html",
            minijinja::context! {
                subject => &subject,
                organization_name => &details.organization_name,
                conversation_title => &details.conversation_title,
                roles => &details.roles,
                conversation_link => format!(
                    "{}/admin/conversations/{}",
                    state.config.domain, details.conversation_id
                ),
            },
            None,
        );
        if let Err(error) = sent {
            warn!("Could not email a conversation administrator: {error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use crate::mailer::{MockComhairleMailer, SentEmail};
    use crate::models::model_test_helpers::{
        get_random_conversation_id, get_random_organization_id, setup_app_and_session_with_state,
    };
    use crate::models::organization;
    use crate::models::permissions::assignments::{Recipient, SaveAssignments, save};
    use crate::models::permissions::{GrantRoleRequest, PermissionTriplet};
    use crate::test_helpers::{TEST_PASSWORD, UserSession, test_state};

    struct Fixture {
        state: Arc<ComhairleState>,
        admin_id: Uuid,
        conversation_id: Uuid,
        group_id: Uuid,
        organization_id: Uuid,
        app: crate::App,
        sent: Arc<Mutex<Vec<SentEmail>>>,
    }

    async fn fixture(
        pool: sqlx::PgPool,
        contact_email: Option<&str>,
    ) -> Result<Fixture, Box<dyn std::error::Error>> {
        let sent: Arc<Mutex<Vec<SentEmail>>> = Arc::default();
        let state = Arc::new(
            test_state()
                .db(pool)
                .mailer(Arc::new(MockComhairleMailer::recording(sent.clone())))
                .call()?,
        );
        let (app, mut admin) = setup_app_and_session_with_state(state.clone()).await?;
        let admin_id = admin.id.unwrap();
        let conversation_id = get_random_conversation_id(&app, &mut admin).await?;
        let organization_id = get_random_organization_id(&app, &mut admin).await?;
        let (sql, values) = Query::update()
            .table(OrganizationIden::Table)
            .value(OrganizationIden::ContactEmail, contact_email)
            .and_where(Expr::col(OrganizationIden::Id).eq(organization_id))
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values).execute(&state.db).await?;
        let group_id = organization::get_by_id(&state.db, &organization_id)
            .await?
            .user_group_id;
        Ok(Fixture {
            state,
            admin_id,
            conversation_id,
            group_id,
            organization_id,
            app,
            sent,
        })
    }

    async fn assign(
        fixture: &Fixture,
        roles: &[&str],
    ) -> Result<assignments::Snapshot, ComhairleError> {
        let recipient = Recipient::Group(fixture.group_id);
        let current = assignments::load(
            &fixture.state,
            ResourceType::Conversation,
            fixture.conversation_id,
            fixture.admin_id,
            recipient,
        )
        .await?;
        save(
            &fixture.state,
            ResourceType::Conversation,
            fixture.conversation_id,
            fixture.admin_id,
            recipient,
            SaveAssignments {
                expected_version: current.version,
                roles: roles.iter().map(|role| role.to_string()).collect(),
                grant_reason: "Test".into(),
            },
        )
        .await
    }

    async fn group_roles(fixture: &Fixture) -> Vec<String> {
        assignments::load(
            &fixture.state,
            ResourceType::Conversation,
            fixture.conversation_id,
            fixture.admin_id,
            Recipient::Group(fixture.group_id),
        )
        .await
        .unwrap()
        .roles
    }

    fn emails_with(fixture: &Fixture, template: &str) -> Vec<SentEmail> {
        fixture
            .sent
            .lock()
            .unwrap()
            .iter()
            .filter(|email| email.template == template)
            .cloned()
            .collect()
    }

    async fn invitation_id(fixture: &Fixture) -> Uuid {
        use pending_role_assignment::PendingRoleAssignmentIden as P;
        let (sql, values) = Query::select()
            .column(P::Id)
            .from(P::Table)
            .and_where(Expr::col(P::OrganizationId).eq(fixture.organization_id))
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_scalar_with(&sql, values)
            .fetch_one(&fixture.state.db)
            .await
            .unwrap()
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn email_claims_do_not_consume_organization_proposals(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = fixture(pool, Some("contact@org.example")).await?;
        assign(&fixture, &["observer"]).await?;
        let id = invitation_id(&fixture).await;

        pending_role_assignment::claim(&fixture.state, fixture.admin_id, "contact@org.example")
            .await;

        assert!(group_roles(&fixture).await.is_empty());
        let invitation = details(&fixture.state, id, fixture.admin_id).await?;
        assert_eq!(invitation.roles, vec!["observer".to_string()]);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn proposing_again_replaces_roles_and_preserves_the_invitation_link(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = fixture(pool, Some("contact@org.example")).await?;
        assign(&fixture, &["observer"]).await?;
        let id = invitation_id(&fixture).await;
        assign(&fixture, &["moderator"]).await?;

        assert_eq!(invitation_id(&fixture).await, id);
        accept(&fixture.state, id, fixture.admin_id).await?;
        assert_eq!(group_roles(&fixture).await, vec!["moderator".to_string()]);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn first_roles_for_an_organization_are_proposed_not_granted(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = fixture(pool, Some("contact@org.example")).await?;

        let snapshot = assign(&fixture, &["observer"]).await?;

        assert_eq!(
            snapshot.invitation_sent_to.as_deref(),
            Some("contact@org.example")
        );
        assert!(group_roles(&fixture).await.is_empty());
        let invitations = emails_with(&fixture, "organization_conversation_invitation.html");
        assert_eq!(invitations.len(), 1);
        assert_eq!(invitations[0].to, "contact@org.example");
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn organizations_without_a_contact_email_are_granted_directly(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = fixture(pool, None).await?;

        let snapshot = assign(&fixture, &["observer"]).await?;

        assert_eq!(snapshot.invitation_sent_to, None);
        assert_eq!(group_roles(&fixture).await, vec!["observer".to_string()]);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn organizations_with_existing_roles_are_updated_without_an_invitation(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = fixture(pool, Some("contact@org.example")).await?;
        let (sql, values) = Query::insert()
            .into_table(GroupPermissionIden::Table)
            .columns([
                GroupPermissionIden::GroupId,
                GroupPermissionIden::ResourceId,
                GroupPermissionIden::RoleName,
                GroupPermissionIden::GrantReason,
            ])
            .values([
                fixture.group_id.into(),
                fixture.conversation_id.into(),
                "observer".into(),
                "Test".into(),
            ])?
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values)
            .execute(&fixture.state.db)
            .await?;

        let snapshot = assign(&fixture, &["observer", "moderator"]).await?;

        assert_eq!(snapshot.invitation_sent_to, None);
        assert_eq!(group_roles(&fixture).await.len(), 2);
        assert!(emails_with(&fixture, "organization_conversation_invitation.html").is_empty());
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn accepting_grants_the_roles_and_emails_conversation_admins(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = fixture(pool, Some("contact@org.example")).await?;
        permissions::grant_role(
            &fixture.state,
            GrantRoleRequest {
                actor_id: ActorId::User(fixture.admin_id),
                permission_triplet: PermissionTriplet(
                    "conversation",
                    &fixture.conversation_id,
                    "admin",
                ),
                granted_by: &fixture.admin_id,
                grant_reason: "Test",
            },
        )
        .await?;
        assign(&fixture, &["observer", "moderator"]).await?;
        let id = invitation_id(&fixture).await;

        let details = accept(&fixture.state, id, fixture.admin_id).await?;

        assert_eq!(details.organization_id, fixture.organization_id);
        let mut roles = group_roles(&fixture).await;
        roles.sort();
        assert_eq!(roles, vec!["moderator".to_string(), "observer".to_string()]);
        let notices = emails_with(&fixture, "organization_added_to_conversation.html");
        let admin = users::get_user_by_id(&fixture.admin_id, &fixture.state.db).await?;
        assert!(
            notices
                .iter()
                .any(|email| Some(&email.to) == admin.email.as_ref())
        );
        assert!(matches!(
            accept(&fixture.state, id, fixture.admin_id).await,
            Err(ComhairleError::ResourceNotFound(_))
        ));
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn only_organization_administrators_can_accept(
        pool: sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let fixture = fixture(pool, Some("contact@org.example")).await?;
        let mut outsider = UserSession::new("outsider", TEST_PASSWORD, "outsider@example.com");
        outsider.signup(&fixture.app).await?;
        assign(&fixture, &["observer"]).await?;
        let id = invitation_id(&fixture).await;

        let attempt = accept(&fixture.state, id, outsider.id.unwrap()).await;

        assert!(matches!(attempt, Err(ComhairleError::UserNotAuthorized)));
        assert!(group_roles(&fixture).await.is_empty());
        let details = details(&fixture.state, id, outsider.id.unwrap()).await?;
        assert!(!details.can_accept);
        Ok(())
    }
}
