use chrono::{DateTime, Utc};
use comhairle_macros::Translatable;
use partially::Partial;
use schemars::JsonSchema;
use sea_query::{enum_def, Alias, Expr, Func, PostgresQueryBuilder, Query};
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
        organization::OrganizationIden,
        pagination::{Order, PageOptions, PaginatedResults},
        translations::{new_translation, TextContentId, TextFormat},
    },
};

#[derive(Partial, Debug, Deserialize, Serialize, FromRow, Clone, JsonSchema, Translatable)]
#[enum_def(table_name = "region")]
#[partially(derive(Serialize, Deserialize, Debug, JsonSchema, Default))]
pub struct Region {
    #[partially(omit)]
    pub id: Uuid,
    #[partially(omit)]
    pub name: TextContentId,
    #[partially(omit)]
    pub description: TextContentId,
    pub region_type: RegionType,
    #[partially(transparent)]
    pub official_id: Option<String>,
    #[partially(omit)]
    pub created_at: DateTime<Utc>,
    #[partially(omit)]
    pub updated_at: DateTime<Utc>,
}

#[derive(
    Debug, Default, Serialize, Deserialize, PartialEq, PartialOrd, sqlx::Type, Clone, JsonSchema,
)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(Dummy))]
pub enum RegionType {
    #[sqlx(rename = "custom")]
    Custom,
    #[sqlx(rename = "official")]
    #[default]
    Official,
}

impl From<RegionType> for sea_query::Value {
    fn from(val: RegionType) -> Self {
        sea_query::Value::String(Some(Box::new(val.to_string())))
    }
}

impl std::fmt::Display for RegionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            RegionType::Custom => "custom",
            RegionType::Official => "official",
        };
        write!(f, "{}", value)
    }
}

const DEFAULT_COLUMNS: [RegionIden; 7] = [
    RegionIden::Id,
    RegionIden::Name,
    RegionIden::Description,
    RegionIden::RegionType,
    RegionIden::OfficialId,
    RegionIden::CreatedAt,
    RegionIden::UpdatedAt,
];

#[derive(Serialize, Deserialize, JsonSchema, Debug, Default)]
pub struct CreateRegion {
    pub name: String,
    pub description: String,
    pub region_type: RegionType,
    pub official_id: Option<String>,
}

impl CreateRegion {
    fn columns(&self) -> Vec<RegionIden> {
        let mut columns = vec![RegionIden::RegionType];
        if self.official_id.is_some() {
            columns.push(RegionIden::OfficialId);
        }

        columns
    }

    fn values(&self) -> Vec<sea_query::SimpleExpr> {
        let mut values = vec![self.region_type.clone().into()];
        if let Some(value) = &self.official_id {
            values.push(value.into());
        }

        values
    }
}

#[instrument(err(Debug))]
pub async fn create(
    db: &PgPool,
    new_region: &CreateRegion,
    locale: &str,
) -> Result<Region, ComhairleError> {
    let mut columns = new_region.columns();
    let mut values = new_region.values();

    let name_translation = new_translation(db, locale, &new_region.name, TextFormat::Plain).await?;
    let description_translation =
        new_translation(db, locale, &new_region.description, TextFormat::Plain).await?;

    columns.push(RegionIden::Name);
    values.push(name_translation.id.into());

    columns.push(RegionIden::Description);
    values.push(description_translation.id.into());

    let (sql, values) = Query::insert()
        .into_table(RegionIden::Table)
        .columns(columns)
        .values(values)?
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let region = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(region)
}

impl PartialRegion {
    pub fn to_values(&self) -> Vec<(RegionIden, sea_query::SimpleExpr)> {
        let mut values = vec![];
        if let Some(value) = &self.region_type {
            values.push((RegionIden::RegionType, value.clone().into()));
        }
        if let Some(value) = &self.official_id {
            values.push((RegionIden::OfficialId, value.into()));
        }

        values
    }
}

#[instrument(err(Debug))]
pub async fn update(
    db: &PgPool,
    id: &Uuid,
    update_region: &PartialRegion,
) -> Result<Region, ComhairleError> {
    let values = update_region.to_values();

    if values.is_empty() {
        return Err(ComhairleError::NoValidUpdates);
    }

    let (sql, values) = Query::update()
        .table(RegionIden::Table)
        .values(values)
        .and_where(Expr::col(RegionIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let region = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(region)
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct RegionOrderOptions {
    name: Option<Order>,
    created_at: Option<Order>,
}

impl RegionOrderOptions {
    fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(order) = &self.created_at {
            query = query
                .order_by((RegionIden::Table, RegionIden::CreatedAt), order.into())
                .to_owned();
        }
        query
    }

    fn apply_to_localized(
        &self,
        mut query: sea_query::SelectStatement,
    ) -> sea_query::SelectStatement {
        use crate::models::translations::TextTranslationIden;
        use sea_query::Alias;

        if let Some(order) = &self.name {
            let tt_name_alias = Alias::new("tt_name");
            query = query
                .order_by((tt_name_alias, TextTranslationIden::Content), order.into())
                .to_owned();
        }
        self.apply(query)
    }
}

#[derive(Deserialize, Debug, JsonSchema, Default)]
pub struct RegionFilterOptions {
    organization_id: Option<Uuid>,
}

impl RegionFilterOptions {
    fn apply(&self, mut query: sea_query::SelectStatement) -> sea_query::SelectStatement {
        if let Some(value) = self.organization_id {
            let subquery = Query::select()
                .expr(Func::cust(Alias::new("UNNEST")).arg(Expr::col((
                    OrganizationIden::Table,
                    OrganizationIden::Regions,
                ))))
                .from(OrganizationIden::Table)
                .and_where(Expr::col((OrganizationIden::Table, OrganizationIden::Id)).eq(value))
                .to_owned();

            query = query
                .and_where(Expr::col((RegionIden::Table, RegionIden::Id)).in_subquery(subquery))
                .to_owned();
        }

        query
    }
}

#[instrument(err(Debug))]
pub async fn list(
    db: &PgPool,
    page_options: PageOptions,
    filter_options: RegionFilterOptions,
    order_options: RegionOrderOptions,
    locale: &str,
) -> Result<PaginatedResults<LocalizedRegion>, ComhairleError> {
    let query = Query::select()
        .from(RegionIden::Table)
        .columns(DEFAULT_COLUMNS.map(|col| (RegionIden::Table, col)))
        .to_owned();

    let query = LocalizedRegion::query_to_localisation(query, locale);

    let query = filter_options.apply(query);
    let query = order_options.apply_to_localized(query);
    let regions = page_options.fetch_paginated_results(db, query).await?;

    Ok(regions)
}

#[instrument(err(Debug))]
pub async fn get_localized_by_id(
    db: &PgPool,
    id: &Uuid,
    locale: &str,
) -> Result<LocalizedRegion, ComhairleError> {
    let query = Query::select()
        .columns(DEFAULT_COLUMNS.map(|col| (RegionIden::Table, col)))
        .from(RegionIden::Table)
        .and_where(Expr::col((RegionIden::Table, RegionIden::Id)).eq(id.to_owned()))
        .to_owned();

    let query = LocalizedRegion::query_to_localisation(query, locale);

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let region = query_as_with(&sql, values)
        .fetch_one(db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ComhairleError::ResourceNotFound("Region".into()),
            other => ComhairleError::DatabaseError(other),
        })?;

    Ok(region)
}

#[instrument(err(Debug))]
pub async fn delete(db: &PgPool, id: &Uuid) -> Result<Region, ComhairleError> {
    let (sql, values) = Query::delete()
        .from_table(RegionIden::Table)
        .and_where(Expr::col(RegionIden::Id).eq(id.to_owned()))
        .returning(Query::returning().columns(DEFAULT_COLUMNS))
        .build_sqlx(PostgresQueryBuilder);

    let region = query_as_with(&sql, values).fetch_one(db).await?;

    Ok(region)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        models::model_test_helpers::setup_default_app_and_session,
        routes::organizations::dto::OrganizationDto,
    };

    use super::*;
    use std::error::Error;

    #[sqlx::test]
    async fn should_create_a_region(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;

        let new_region = CreateRegion {
            name: "Glasgow".to_string(),
            description: "Largest city in Scotland".to_string(),
            region_type: RegionType::Official,
            official_id: Some("G".to_string()),
        };

        let region = create(&pool, &new_region, "en").await?;

        assert_eq!(
            region.region_type,
            RegionType::Official,
            "incorrect region_type"
        );
        assert_eq!(
            region.official_id.unwrap(),
            "G".to_string(),
            "incorrect official_id"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_update_a_region(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;
        let new_region = CreateRegion {
            name: "Glasgow".to_string(),
            description: "Largest city in Scotland".to_string(),
            region_type: RegionType::Official,
            official_id: Some("G".to_string()),
        };

        let region = create(&pool, &new_region, "en").await?;
        assert_eq!(
            region.region_type,
            RegionType::Official,
            "incorrect region after creation"
        );
        assert_eq!(
            region.official_id.unwrap(),
            "G".to_string(),
            "incorrect official_id after creation"
        );

        let update_region = PartialRegion {
            region_type: Some(RegionType::Custom),
            official_id: Some("G1".to_string()),
        };
        let updated_region = update(&pool, &region.id, &update_region).await?;

        assert_eq!(
            updated_region.region_type,
            RegionType::Custom,
            "incorrect region after update"
        );
        assert_eq!(
            updated_region.official_id.unwrap(),
            "G1".to_string(),
            "incorrect official_id after update"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_ordered_regions(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;

        let new_region_1 = CreateRegion {
            name: "region_a".to_string(),
            description: "region_a".to_string(),
            ..Default::default()
        };
        let new_region_2 = CreateRegion {
            name: "region_c".to_string(),
            description: "region_c".to_string(),
            ..Default::default()
        };
        let new_region_3 = CreateRegion {
            name: "region_d".to_string(),
            description: "region_d".to_string(),
            ..Default::default()
        };
        let new_region_4 = CreateRegion {
            name: "region_b".to_string(),
            description: "region_b".to_string(),
            ..Default::default()
        };
        let _ = create(&pool, &new_region_1, "en").await?;
        let _ = create(&pool, &new_region_2, "en").await?;
        let _ = create(&pool, &new_region_3, "en").await?;
        let _ = create(&pool, &new_region_4, "en").await?;

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = RegionFilterOptions {
            ..Default::default()
        };
        let order_options = RegionOrderOptions {
            created_at: Some(Order::Asc),
            ..Default::default()
        };
        let results = list(&pool, page_options, filter_options, order_options, "en").await?;

        assert_eq!(results.total, 4, "incorrect number of regions");
        assert_eq!(
            results.records[0].name,
            "region_a".to_string(),
            "incorrect first region [created_at: asc]"
        );
        assert_eq!(
            results.records[3].name,
            "region_b".to_string(),
            "incorrect last region [created_at: desc]"
        );

        let page_options = PageOptions {
            offset: None,
            limit: None,
        };
        let filter_options = RegionFilterOptions {
            ..Default::default()
        };
        let order_options = RegionOrderOptions {
            name: Some(Order::Desc),
            ..Default::default()
        };
        let results = list(&pool, page_options, filter_options, order_options, "en").await?;

        assert_eq!(results.total, 4, "incorrect number of regions");
        assert_eq!(
            results.records[0].name,
            "region_d".to_string(),
            "incorrect first region [name: desc]"
        );
        assert_eq!(
            results.records[3].name,
            "region_a".to_string(),
            "incorrect last region [name: desc]"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_list_regions_filtered_by_organization(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        let new_region_1 = CreateRegion {
            name: "Glasgow North".to_string(),
            description: "Glasgow North".to_string(),
            region_type: RegionType::Official,
            official_id: Some("G1".to_string()),
        };
        let new_region_2 = CreateRegion {
            name: "Glasgow South".to_string(),
            description: "Glasgow South".to_string(),
            region_type: RegionType::Official,
            official_id: Some("G2".to_string()),
        };
        let new_region_3 = CreateRegion {
            name: "Glasgow East".to_string(),
            description: "Glasgow East".to_string(),
            region_type: RegionType::Official,
            official_id: Some("G3".to_string()),
        };
        let region_1 = create(&pool, &new_region_1, "en").await?;
        let region_2 = create(&pool, &new_region_2, "en").await?;
        let _ = create(&pool, &new_region_3, "en").await?;

        let (_, org_res, _) = session.create_random_organization(&app).await?;
        let organization: OrganizationDto = serde_json::from_value(org_res)?;
        let _ = session
            .put(
                &app,
                &format!("/organizations/{}", organization.id),
                json!({
                    "regions": vec![region_1.id, region_2.id]
                })
                .to_string()
                .into(),
            )
            .await?;

        let page_options = PageOptions {
            limit: None,
            offset: None,
        };
        let order_options = RegionOrderOptions {
            ..Default::default()
        };
        let filter_options = RegionFilterOptions {
            organization_id: Some(organization.id),
        };
        let results = list(&pool, page_options, filter_options, order_options, "en").await?;

        assert_eq!(results.total, 2, "incorrect total");
        assert_eq!(results.records[0].id, region_1.id, "incorrect first id");
        assert_eq!(results.records[1].id, region_2.id, "incorrect second id");

        Ok(())
    }

    #[sqlx::test]
    async fn should_get_a_localized_region(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;
        let new_region = CreateRegion {
            name: "Glasgow".to_string(),
            description: "Largest city in Scotland".to_string(),
            region_type: RegionType::Official,
            official_id: Some("G".to_string()),
        };

        let region = create(&pool, &new_region, "en").await?;
        let region = get_localized_by_id(&pool, &region.id, "en").await?;

        assert_eq!(region.name, "Glasgow".to_string(), "incorrect name");
        assert_eq!(
            region.description,
            "Largest city in Scotland".to_string(),
            "incorrect description"
        );

        Ok(())
    }

    #[sqlx::test]
    async fn should_delete_a_region(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let _ = setup_default_app_and_session(&pool).await?;
        let new_region = CreateRegion {
            name: "Glasgow".to_string(),
            description: "Largest city in Scotland".to_string(),
            region_type: RegionType::Official,
            official_id: Some("G".to_string()),
        };

        let region = create(&pool, &new_region, "en").await?;
        let _ = delete(&pool, &region.id).await?;

        let err = get_localized_by_id(&pool, &region.id, "en")
            .await
            .unwrap_err();

        match err {
            ComhairleError::ResourceNotFound(e) => {
                assert_eq!(e, "Region".to_string(), "incorrect error message");
            }
            _ => panic!("Expected ResourceNotFound error"),
        }

        Ok(())
    }
}
