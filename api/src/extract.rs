//! Custom axum extractors.
//!
//! These live in `api` rather than the model layer because they are web-layer glue:
//! they implement [`FromRequestParts`] against [`ComhairleState`], which knows about
//! every service. The value types they produce (`PageOptions`, order enums, and so
//! on) stay in [`crate::models::pagination`], where the query builders that consume
//! them live.

use std::sync::Arc;

use aide::OperationIo;
use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, de::DeserializeOwned};

use crate::models::pagination::parse_sort_options_to_json;
use crate::{ComhairleState, error::ComhairleError, error::TransportError};

#[derive(Deserialize, Debug, OperationIo)]
pub struct Sort {
    sort: Option<String>,
}

/// Custom extractor for getting order parameters
/// Should be called as OrderParams<T> where T is some
/// type that has a series of keys and then Option<Order>
#[derive(OperationIo)]
pub struct OrderParams<T: DeserializeOwned>(pub T);

impl<T> FromRequestParts<Arc<ComhairleState>> for OrderParams<T>
where
    T: DeserializeOwned + Default,
{
    type Rejection = ComhairleError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &Arc<ComhairleState>,
    ) -> Result<Self, Self::Rejection> {
        let sort = parts
            .extract::<axum::extract::Query<Sort>>()
            .await
            .map_err(|e| TransportError::FailedToParseOrderParams(e.to_string()))?;
        if let Some(sort_string) = &sort.sort {
            let order_params = parse_sort_options_to_json(sort_string);
            let order_params: T = serde_json::from_value(order_params)
                .map_err(|e| TransportError::FailedToParseOrderParams(e.to_string()))?;
            Ok(OrderParams(order_params))
        } else {
            Ok(OrderParams(T::default()))
        }
    }
}
