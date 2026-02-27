use std::{str::FromStr, sync::Arc};

use aide::OperationIo;
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use schemars::JsonSchema;
use sea_query::{Alias, Expr, PostgresQueryBuilder, SelectStatement};
use sea_query_binder::SqlxBinder;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{FromRow, PgPool};

use crate::{error::ComhairleError, ComhairleState};

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
            .map_err(|e| ComhairleError::FailedToParseOrderParams(e.to_string()))?;
        if let Some(sort_string) = &sort.sort {
            let order_params = parse_sort_options_to_json(sort_string);
            let order_params: T = serde_json::from_value(order_params)
                .map_err(|e| ComhairleError::FailedToParseOrderParams(e.to_string()))?;
            Ok(OrderParams(order_params))
        } else {
            Ok(OrderParams(T::default()))
        }
    }
}

#[derive(Deserialize, Debug, Serialize, JsonSchema, Clone)]
pub struct PageOptions {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
#[serde(rename_all = "lowercase")]
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

impl From<Order> for sea_query::Order {
    fn from(val: Order) -> Self {
        match val {
            Order::Asc => sea_query::Order::Asc,
            Order::Desc => sea_query::Order::Desc,
        }
    }
}

impl From<&Order> for sea_query::Order {
    fn from(val: &Order) -> Self {
        match val {
            Order::Asc => sea_query::Order::Asc,
            Order::Desc => sea_query::Order::Desc,
        }
    }
}

fn parse_sort_options_to_json(order_str: &str) -> Value {
    let mut map = serde_json::Map::new();

    for part in order_str.split(',') {
        let mut iter = part.split_whitespace();
        if let (Some(field), Some(order_str)) = (iter.next(), iter.next()) {
            if let Ok(order) = Order::from_str(order_str) {
                map.insert(field.to_string(), json!(order));
            }
        }
    }

    Value::Object(map)
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct PaginatedResults<T: JsonSchema> {
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
        T: for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin + JsonSchema,
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

        println!("Final Count Query {count_sql}");

        let total: i32 = sqlx::query_scalar_with(&count_sql, count_values)
            .fetch_one(pool)
            .await?;

        // Generate SQL for paginated query
        let (query_sql, query_values) = paginated_query.build_sqlx(PostgresQueryBuilder);

        println!("Final Query {query_sql}");
        let records: Vec<T> = sqlx::query_as_with(&query_sql, query_values)
            .fetch_all(pool)
            .await?;

        Ok(PaginatedResults { total, records })
    }
}
