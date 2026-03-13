use chrono::{DateTime, Utc};
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Expr, PostgresQueryBuilder, Query, SelectStatement, SimpleExpr};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, query_as_with, PgPool};
use tracing::instrument;
use uuid::Uuid;

#[cfg(test)]
use fake::Dummy;

use crate::{
    error::ComhairleError,
    models::{
        pagination::{Order, PageOptions, PaginatedResults},
        users::Role,
    },
};

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema)]
#[enum_def(table_name = "resource_permission")]
#[partially(derive(Deserialize, Debug, JsonSchema, Default))]
pub struct ResourcePermission {
    #[partially(omit)]
    pub id: Uuid,
    pub entity_type: EntityType,
    pub entity_id: Uuid,
    pub resource_type: ResourceType,
    pub resource_id: Uuid,
    pub role: Role,
    pub granted_by_entity_type: EntityType,
    pub granted_by_entity_id: Uuid,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    #[sqlx(rename = "organization")]
    Organization,
    #[sqlx(rename = "user")]
    User,
}

impl From<EntityType> for sea_query::Value {
    fn from(val: EntityType) -> Self {
        Self::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            EntityType::Organization => "organization",
            EntityType::User => "user",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(Dummy))]
pub enum ResourceType {
    #[sqlx(rename = "conversation")]
    Conversation,
    #[sqlx(rename = "workflow")]
    Workflow,
}

impl From<ResourceType> for sea_query::Value {
    fn from(val: ResourceType) -> Self {
        Self::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ResourceType::Conversation => "conversation",
            ResourceType::Workflow => "workflow",
        };
        write!(f, "{}", value)
    }
}

const DEFAULT_COLUMNS: [ResourcePermissionIden; 10] = [
    ResourcePermissionIden::Id,
    ResourcePermissionIden::EntityType,
    ResourcePermissionIden::EntityId,
    ResourcePermissionIden::ResourceType,
    ResourcePermissionIden::ResourceId,
    ResourcePermissionIden::Role,
    ResourcePermissionIden::GrantedByEntityType,
    ResourcePermissionIden::GrantedByEntityId,
    ResourcePermissionIden::CreatedAt,
    ResourcePermissionIden::UpdatedAt,
];

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CreateResourcePermission {
    pub entity_type: EntityType,
    pub entity_id: Uuid,
    pub resource_type: ResourceType,
    pub resource_id: Uuid,
    pub role: Role,
    pub granted_by_entity_type: EntityType,
    pub granted_by_entity_id: Uuid,
}

impl CreateResourcePermission {
    fn columns(&self) -> Vec<ResourcePermissionIden> {
        vec![
            ResourcePermissionIden::EntityType,
            ResourcePermissionIden::EntityId,
            ResourcePermissionIden::ResourceType,
            ResourcePermissionIden::ResourceId,
            ResourcePermissionIden::Role,
            ResourcePermissionIden::GrantedByEntityType,
            ResourcePermissionIden::GrantedByEntityId,
        ]
    }

    fn values(&self) -> Vec<SimpleExpr> {
        vec![
            self.entity_type.clone().into(),
            self.entity_id.into(),
            self.resource_type.clone().into(),
            self.resource_id.into(),
            self.role.clone().into(),
            self.granted_by_entity_type.clone().into(),
            self.granted_by_entity_id.into(),
        ]
    }
}

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    new_permission: &CreateResourcePermission,
) -> Result<ResourcePermission, ComhairleError> {
    let columns = new_permission.columns();
    let values = new_permission.values();

    let (sql, values) = Query::insert()
        .into_table(ResourcePermissionIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let permission = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(permission)
}

impl PartialResourcePermission {
    fn to_values(&self) -> Vec<(ResourcePermissionIden, SimpleExpr)> {
        let mut values = vec![];

        if let Some(value) = &self.entity_type {
            values.push((ResourcePermissionIden::EntityType, value.clone().into()));
        }
        if let Some(value) = &self.entity_id {
            values.push((ResourcePermissionIden::EntityId, (*value).into()));
        }
        if let Some(value) = &self.resource_type {
            values.push((ResourcePermissionIden::ResourceType, value.clone().into()));
        }
        if let Some(value) = &self.resource_id {
            values.push((ResourcePermissionIden::ResourceId, (*value).into()));
        }
        if let Some(value) = &self.role {
            values.push((ResourcePermissionIden::Role, value.clone().into()));
        }
        if let Some(value) = &self.granted_by_entity_type {
            values.push((
                ResourcePermissionIden::GrantedByEntityType,
                value.clone().into(),
            ));
        }
        if let Some(value) = &self.granted_by_entity_id {
            values.push((ResourcePermissionIden::GrantedByEntityId, (*value).into()));
        }

        values
    }
}

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    update_permission: &PartialResourcePermission,
) -> Result<ResourcePermission, ComhairleError> {
    let values = update_permission.to_values();

    let (sql, values) = Query::update()
        .table(ResourcePermissionIden::Table)
        .values(values)
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let permission = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(permission)
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct ResourcePermissionOrderOptions {
    created_at: Option<Order>,
}

impl ResourcePermissionOrderOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(order) = &self.created_at {
            query = query
                .order_by(
                    (
                        ResourcePermissionIden::Table,
                        ResourcePermissionIden::CreatedAt,
                    ),
                    order.into(),
                )
                .to_owned();
        }

        query
    }
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct ResourcePermissionFilterOptions {
    entity_id: Option<Uuid>,
    resource_id: Option<Uuid>,
}

impl ResourcePermissionFilterOptions {
    fn apply(&self, mut query: SelectStatement) -> SelectStatement {
        if let Some(value) = self.entity_id {
            query = query
                .and_where(Expr::col(ResourcePermissionIden::EntityId).eq(value.to_owned()))
                .to_owned();
        }
        if let Some(value) = self.resource_id {
            query = query
                .and_where(Expr::col(ResourcePermissionIden::ResourceId).eq(value.to_owned()))
                .to_owned();
        }

        query
    }
}

#[instrument(err(Debug))]
pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    filter_options: ResourcePermissionFilterOptions,
    order_options: ResourcePermissionOrderOptions,
) -> Result<PaginatedResults<ResourcePermission>, ComhairleError> {
    let query = Query::select()
        .from(ResourcePermissionIden::Table)
        .columns(DEFAULT_COLUMNS)
        .to_owned();

    let query = filter_options.apply(query);
    let query = order_options.apply(query);

    let permissions = page_options.fetch_paginated_results(db, query).await?;

    Ok(permissions)
}

#[instrument(err(Debug))]
pub async fn get_by_id(db: &PgPool, id: &Uuid) -> Result<ResourcePermission, ComhairleError> {
    let (sql, values) = Query::select()
        .from(ResourcePermissionIden::Table)
        .columns(DEFAULT_COLUMNS)
        .and_where(Expr::col(ResourcePermissionIden::Id).eq(id.to_owned()))
        .build_sqlx(PostgresQueryBuilder);

    let permission = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ComhairleError::ResourceNotFound("ResourcePermission".into())
            }
            other => ComhairleError::DatabaseError(other),
        })?;

    Ok(permission)
}

#[instrument(err(Debug))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<ResourcePermission, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(ResourcePermissionIden::Table)
        .and_where(Expr::col(ResourcePermissionIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let permission = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(permission)
}

#[cfg(test)]
mod tests {
    use crate::{
        models::model_test_helpers::setup_default_app_and_session,
        routes::{conversations::dto::ConversationDto, organizations::dto::OrganizationDto},
    };

    use super::*;

    use std::error::Error;

    #[sqlx::test]
    async fn should_create_resource_permission(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, org_res, _) = session.create_random_organization(&app).await?;
        let organization: OrganizationDto = serde_json::from_value(org_res)?;
        let (_, convo_res, _) = session.create_random_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(convo_res)?;
        let (_, user, _) = session.current_user(&app).await?;

        let new_permission = CreateResourcePermission {
            entity_type: EntityType::Organization,
            entity_id: organization.id,
            resource_type: ResourceType::Conversation,
            resource_id: conversation.id,
            role: Role::Contributor,
            granted_by_entity_type: EntityType::User,
            granted_by_entity_id: user.id,
        };

        let permission = create(&pool, &new_permission).await?;

        assert_eq!(
            permission.entity_type,
            EntityType::Organization,
            "incorrect entity_type"
        );
        assert_eq!(
            permission.resource_id, conversation.id,
            "incorrect entity_type"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_resource_permission(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, org_res, _) = session.create_random_organization(&app).await?;
        let organization: OrganizationDto = serde_json::from_value(org_res)?;
        let (_, convo_res, _) = session.create_random_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(convo_res)?;
        let (_, user, _) = session.current_user(&app).await?;

        let new_permission = CreateResourcePermission {
            entity_type: EntityType::Organization,
            entity_id: organization.id,
            resource_type: ResourceType::Conversation,
            resource_id: conversation.id,
            role: Role::Contributor,
            granted_by_entity_type: EntityType::User,
            granted_by_entity_id: user.id,
        };

        let permission = create(&pool, &new_permission).await?;

        assert_eq!(
            permission.role,
            Role::Contributor,
            "incorrect role before update"
        );

        let permission = update(
            &pool,
            &PartialResourcePermission {
                role: Some(Role::Translator),
                ..Default::default()
            },
        )
        .await?;

        assert_eq!(
            permission.role,
            Role::Translator,
            "incorrect role after update"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_filter_resource_permissions(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, org_res, _) = session.create_random_organization(&app).await?;
        let organization: OrganizationDto = serde_json::from_value(org_res)?;
        let (_, convo_res_1, _) = session.create_random_conversation(&app).await?;
        let conversation_1: ConversationDto = serde_json::from_value(convo_res_1)?;
        let (_, convo_res_2, _) = session.create_random_conversation(&app).await?;
        let conversation_2: ConversationDto = serde_json::from_value(convo_res_2)?;
        let (_, user, _) = session.current_user(&app).await?;

        let new_permission_1 = CreateResourcePermission {
            entity_type: EntityType::Organization,
            entity_id: organization.id,
            resource_type: ResourceType::Conversation,
            resource_id: conversation_1.id,
            role: Role::Contributor,
            granted_by_entity_type: EntityType::User,
            granted_by_entity_id: user.id,
        };
        let new_permission_2 = CreateResourcePermission {
            entity_type: EntityType::Organization,
            entity_id: user.id,
            resource_type: ResourceType::Conversation,
            resource_id: conversation_2.id,
            role: Role::Contributor,
            granted_by_entity_type: EntityType::User,
            granted_by_entity_id: organization.id,
        };

        let _ = create(&pool, &new_permission_1).await?;
        let _ = create(&pool, &new_permission_2).await?;

        let page_options = PageOptions {
            limit: None,
            offset: None,
        };
        let order_options = ResourcePermissionOrderOptions { created_at: None };
        let filter_options = ResourcePermissionFilterOptions {
            resource_id: Some(conversation_1.id),
            ..Default::default()
        };

        let results_1 = list(&pool, page_options.clone(), filter_options, order_options).await?;

        assert_eq!(results_1.total, 1, "incorrect total [resource_id]");
        assert_eq!(
            results_1.records[0].resource_id, conversation_1.id,
            "incorrect resource_id [resource_id]"
        );

        let order_options = ResourcePermissionOrderOptions { created_at: None };
        let filter_options = ResourcePermissionFilterOptions {
            entity_id: Some(user.id),
            ..Default::default()
        };
        let results_2 = list(&pool, page_options.clone(), filter_options, order_options).await?;

        assert_eq!(results_2.total, 1, "incorrect total [entity_id]");
        assert_eq!(
            results_2.records[0].entity_id, user.id,
            "incorrect resource_id [entity_id]"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_resource_permission_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, org_res, _) = session.create_random_organization(&app).await?;
        let organization: OrganizationDto = serde_json::from_value(org_res)?;
        let (_, convo_res, _) = session.create_random_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(convo_res)?;
        let (_, user, _) = session.current_user(&app).await?;

        let new_permission = CreateResourcePermission {
            entity_type: EntityType::Organization,
            entity_id: organization.id,
            resource_type: ResourceType::Conversation,
            resource_id: conversation.id,
            role: Role::Contributor,
            granted_by_entity_type: EntityType::User,
            granted_by_entity_id: user.id,
        };

        let created_permission = create(&pool, &new_permission).await?;

        let permission = get_by_id(&pool, &created_permission.id).await?;

        assert_eq!(permission.id, created_permission.id, "incorrect ids");

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_resource_permission(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let (_, org_res, _) = session.create_random_organization(&app).await?;
        let organization: OrganizationDto = serde_json::from_value(org_res)?;
        let (_, convo_res, _) = session.create_random_conversation(&app).await?;
        let conversation: ConversationDto = serde_json::from_value(convo_res)?;
        let (_, user, _) = session.current_user(&app).await?;

        let new_permission = CreateResourcePermission {
            entity_type: EntityType::Organization,
            entity_id: organization.id,
            resource_type: ResourceType::Conversation,
            resource_id: conversation.id,
            role: Role::Contributor,
            granted_by_entity_type: EntityType::User,
            granted_by_entity_id: user.id,
        };

        let created_permission = create(&pool, &new_permission).await?;
        let _ = delete(&pool, &created_permission.id).await?;

        let err = get_by_id(&pool, &created_permission.id).await.unwrap_err();

        match err {
            ComhairleError::ResourceNotFound(e) => {
                assert_eq!(
                    e,
                    "ResourcePermission".to_string(),
                    "incorrect error message"
                );
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
