//! Request-scoped authorization glue.
//!
//! The resource types a route authorizes against are extracted from the request,
//! which means they implement [`FromRequestParts`] against [`ComhairleState`].
//! That is web-layer machinery, so it lives here rather than in
//! [`crate::models::permissions`], which keeps the roles, actions and the queries
//! that read them.

use std::sync::Arc;

use aide::OperationIo;
use axum::extract::{FromRequestParts, Path};
use serde::Deserialize;
use uuid::Uuid;

use crate::models;
use crate::models::error::DataError;
use crate::models::permissions::{OwnedResource, ResourceType, SYSTEM_RESOURCE_ID};
use crate::{ComhairleState, error::ComhairleError};

// ---------- //
// * MACROS * //
// ---------- //

macro_rules! define_unowned_resource {
    ($resource_struct:ident, $resource_id_field:ident, $extract_logic:expr) => {
        #[derive(Debug, OperationIo)]
        pub struct $resource_struct {
            pub $resource_id_field: Uuid,
        }

        impl FromRequestParts<Arc<ComhairleState>> for $resource_struct {
            type Rejection = ComhairleError;

            async fn from_request_parts(
                parts: &mut axum::http::request::Parts,
                state: &Arc<ComhairleState>,
            ) -> Result<Self, Self::Rejection> {
                ($extract_logic)(parts, state).await
            }
        }

        impl ExtractResourceId for $resource_struct {
            fn resource_id(&self) -> Uuid {
                self.$resource_id_field
            }
        }

        impl OwnedResource for $resource_struct {}
    };
}

macro_rules! define_owned_resource {
    ($resource_struct:ident, $resource_id_field:ident, $owner_id_field:ident, $extract_logic:expr) => {
        #[derive(Debug, OperationIo)]
        pub struct $resource_struct {
            pub $resource_id_field: Uuid,
            pub $owner_id_field: Uuid,
        }

        impl FromRequestParts<Arc<ComhairleState>> for $resource_struct {
            type Rejection = ComhairleError;

            async fn from_request_parts(
                parts: &mut axum::http::request::Parts,
                state: &Arc<ComhairleState>,
            ) -> Result<Self, Self::Rejection> {
                ($extract_logic)(parts, state).await
            }
        }

        impl ExtractResourceId for $resource_struct {
            fn resource_id(&self) -> Uuid {
                self.$resource_id_field
            }
        }

        impl OwnedResource for $resource_struct {
            fn owner_id(&self) -> Option<Uuid> {
                Some(self.$owner_id_field)
            }
        }
    };
}

/// A trait for extracting a resource ID from a request.
pub trait ExtractResourceId:
    FromRequestParts<Arc<ComhairleState>> + 'static + Send + Sync + OwnedResource
{
    fn resource_id(&self) -> Uuid;
}

// -- SYSTEM RESOURCE -- //

define_unowned_resource!(
    SystemResource,
    resource_id,
    |_parts: &mut axum::http::request::Parts, _state: &Arc<ComhairleState>| async {
        Ok(SystemResource {
            resource_id: SYSTEM_RESOURCE_ID,
        })
    }
);

#[derive(Debug, Deserialize)]
pub struct PermissionTargetPath {
    pub resource_type: String,
    pub resource_id: Uuid,
}

#[derive(Debug, OperationIo)]
pub struct PermissionTargetResource {
    pub resource_type: String,
    pub resource_id: Uuid,
    pub owner_id: Option<Uuid>,
}

impl FromRequestParts<Arc<ComhairleState>> for PermissionTargetResource {
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let Path(PermissionTargetPath {
            resource_type,
            resource_id,
        }) = Path::<PermissionTargetPath>::from_request_parts(parts, state)
            .await
            .map_err(|_| {
                DataError::ResourceNotFound(
                    "Path must contain resource_type and resource_id".to_string(),
                )
            })?;

        let owner_id = if resource_type == ResourceType::Conversation.as_ref() {
            models::conversation::get_by_id(&state.db, &resource_id)
                .await
                .ok()
                .map(|conversation| conversation.owner_id)
        } else {
            None
        };

        Ok(PermissionTargetResource {
            resource_type,
            resource_id,
            owner_id,
        })
    }
}

impl ExtractResourceId for PermissionTargetResource {
    fn resource_id(&self) -> Uuid {
        self.resource_id
    }
}

impl OwnedResource for PermissionTargetResource {
    fn owner_id(&self) -> Option<Uuid> {
        self.owner_id
    }
}

// -- CONVERSATION RESOURCE -- //

/// A struct representing the path parameters for a conversation resource.
#[derive(Deserialize)]
pub struct ConversationPath {
    pub conversation_id: Uuid,
}

async fn extract_conversation_resource(
    parts: &mut axum::http::request::Parts,
    state: &Arc<ComhairleState>,
) -> Result<ConversationResource, ComhairleError> {
    let Path(ConversationPath { conversation_id }) =
        Path::<ConversationPath>::from_request_parts(parts, state)
            .await
            .map_err(|_| {
                DataError::ResourceNotFound("Path must contain a conversation_id".to_string())
            })?;

    let conversation = models::conversation::get_by_id(&state.db, &conversation_id).await?;

    Ok(ConversationResource {
        conversation_id,
        owner_id: conversation.owner_id,
    })
}

define_owned_resource!(
    ConversationResource,
    conversation_id,
    owner_id,
    extract_conversation_resource
);
