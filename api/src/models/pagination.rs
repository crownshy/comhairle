use std::{str::FromStr, sync::Arc};

use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use sea_query::{Alias, Expr, PostgresQueryBuilder, SelectStatement};
use sea_query_binder::SqlxBinder;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{FromRow, PgPool};

use crate::{error::ComhairleError, ComhairleState};

#[derive(Deserialize, Debug)]
pub struct Sort {
    sort: Option<String>,
}

/// Custom extractor for getting order parameters
/// Should be called as OrderParams<T> where T is some
/// type that has a series of keys and then Option<Order>
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
            .map_err(|e| ComhairleError::FailedToParseOrderParams(e.to_string()))?;
        if let Some(sort_string) = &sort.sort {
            let order_params = parse_sort_options_to_json(&sort_string);
            let order_params: T = serde_json::from_value(order_params)
                .map_err(|e| ComhairleError::FailedToParseOrderParams(e.to_string()))?;
            Ok(OrderParams(order_params))
        } else {
            Ok(OrderParams(T::default()))
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct PageOptions {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Order {
    Asc,
    Desc,
}

impl FromStr for Order {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "asc" => Ok(Order::Asc),
            "desc" => Ok(Order::Desc),
            _ => Err(()),
        }
    }
}

impl Into<sea_query::Order> for Order {
    fn into(self) -> sea_query::Order {
        match self {
            Order::Asc => sea_query::Order::Asc,
            Order::Desc => sea_query::Order::Desc,
        }
    }
}
impl Into<sea_query::Order> for &Order {
    fn into(self) -> sea_query::Order {
        match self {
            Order::Asc => sea_query::Order::Asc,
            Order::Desc => sea_query::Order::Desc,
        }
    }
}

fn parse_sort_options_to_json(order_str: &str) -> Value {
    let mut map = serde_json::Map::new();

    for part in order_str.split(',') {
        let mut iter = part.trim().split_whitespace();
        if let (Some(field), Some(order_str)) = (iter.next(), iter.next()) {
            if let Ok(order) = Order::from_str(order_str) {
                map.insert(field.to_string(), json!(order));
            }
        }
    }

    Value::Object(map)
}

#[derive(Serialize, Debug)]
pub struct PaginatedResults<T> {
    pub total: i32,
    pub records: Vec<T>,
}

impl PageOptions {
    pub async fn fetch_paginated_results<T>(
        &self,
        pool: &PgPool,
        base_query: SelectStatement,
    ) -> Result<PaginatedResults<T>, sqlx::Error>
    where
        T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        // Clone the base query to generate a count query
        let count_query = sea_query::Query::select()
            .expr(Expr::cust("COUNT(*)::INT as count"))
            .from_subquery(base_query.clone(), Alias::new("sub"))
            .to_owned();

        // Modify base query with limit and offset
        let mut paginated_query = base_query;

        if let Some(offset) = self.offset {
            paginated_query.offset(offset);
        }

        if let Some(limit) = self.limit {
            paginated_query.limit(limit);
        }

        // Generate SQL for count query
        let (count_sql, count_values) = count_query.build_sqlx(PostgresQueryBuilder);
        let total: i32 = sqlx::query_scalar_with(&count_sql, count_values)
            .fetch_one(pool)
            .await?;

        // Generate SQL for paginated query
        let (query_sql, query_values) = paginated_query.build_sqlx(PostgresQueryBuilder);
        let records: Vec<T> = sqlx::query_as_with(&query_sql, query_values)
            .fetch_all(pool)
            .await?;

        Ok(PaginatedResults { total, records })
    }
}
