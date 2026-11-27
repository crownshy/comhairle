use std::collections::BTreeSet;

use schemars::JsonSchema;
use sea_query::extension::postgres::PgFunc;
use sea_query::{Alias, Expr, Func, LockType, OnConflict, PostgresQueryBuilder, Query, SimpleExpr};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use super::tables::{Feature, FeatureStatus};
use crate::error::ComhairleError;
use crate::models::region::RegionIden;
use crate::models::region_area::RegionAreaIden;
use crate::models::region_area_pipeline::{
    AreaLinkIden, FeatureIden, PipelineDefinition, PipelineIden, SourceFeature, normalize_snapshot,
};
use crate::models::translations::{
    CreateTextTranslation, TextContentId, TextContentIden, TextFormat, TextTranslationIden,
    TranslationScope, create_text_translation, new_translation,
};

#[derive(Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReconciliationCounts {
    pub created: usize,
    pub updated: usize,
    pub unchanged: usize,
    pub retired: usize,
    pub restored: usize,
    pub repaired: Vec<String>,
    pub created_ids: Vec<String>,
    pub updated_ids: Vec<String>,
    pub retired_ids: Vec<String>,
    pub restored_ids: Vec<String>,
    pub unmatched_ids: Vec<String>,
    pub tag_changes: Vec<ManagedTagChange>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ManagedTagChange {
    pub external_id: String,
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

#[derive(sqlx::FromRow)]
struct ExistingFeature {
    #[sqlx(flatten)]
    mapping: Feature,
    name: Option<String>,
    geometry_equal: bool,
    region_name: TextContentId,
}

#[derive(Clone, Copy)]
enum ReconciliationMode {
    Publish,
    Preview,
}

enum FeatureChange {
    Created,
    Updated,
    Unchanged,
    Restored,
}

enum GeometryOutcome {
    Original,
    Repaired,
}

struct PreparedFeature {
    feature: SourceFeature,
    geometry: GeometryOutcome,
}

fn postgis(name: &'static str, arguments: impl IntoIterator<Item = SimpleExpr>) -> SimpleExpr {
    Func::cust(Alias::new(name)).args(arguments).into()
}

fn geometry(feature: &SourceFeature) -> SimpleExpr {
    postgis(
        "st_multi",
        [postgis(
            "st_geomfromgeojson",
            [Expr::val(feature.geometry.to_string()).into()],
        )],
    )
}

fn valid_boundary(geometry: SimpleExpr) -> SimpleExpr {
    postgis("st_isvalid", [geometry.clone()]).and(postgis("st_isempty", [geometry]).not())
}

pub async fn apply(
    db: &PgPool,
    pipeline_id: Uuid,
    definition: &PipelineDefinition,
    snapshot: &[u8],
) -> Result<ReconciliationCounts, ComhairleError> {
    reconcile(
        db,
        pipeline_id,
        definition,
        snapshot,
        ReconciliationMode::Publish,
    )
    .await
}

pub async fn preview(
    db: &PgPool,
    pipeline_id: Uuid,
    definition: &PipelineDefinition,
    snapshot: &[u8],
) -> Result<ReconciliationCounts, ComhairleError> {
    reconcile(
        db,
        pipeline_id,
        definition,
        snapshot,
        ReconciliationMode::Preview,
    )
    .await
}

async fn reconcile(
    db: &PgPool,
    pipeline_id: Uuid,
    definition: &PipelineDefinition,
    snapshot: &[u8],
    mode: ReconciliationMode,
) -> Result<ReconciliationCounts, ComhairleError> {
    let mut tx = db.begin().await?;
    let (sql, values) = Query::select()
        .column(PipelineIden::Id)
        .from(PipelineIden::Table)
        .and_where(Expr::col(PipelineIden::Id).eq(pipeline_id))
        .lock(LockType::Update)
        .build_sqlx(PostgresQueryBuilder);
    let exists: Option<Uuid> = sqlx::query_scalar_with(&sql, values)
        .fetch_optional(&mut *tx)
        .await?;
    if exists.is_none() {
        return Err(ComhairleError::ResourceNotFound("Pipeline".into()));
    }

    let counts = reconcile_locked(&mut tx, pipeline_id, definition, snapshot).await?;
    match mode {
        ReconciliationMode::Publish => tx.commit().await?,
        ReconciliationMode::Preview => tx.rollback().await?,
    }
    Ok(counts)
}

pub(super) async fn reconcile_locked(
    connection: &mut PgConnection,
    pipeline_id: Uuid,
    definition: &PipelineDefinition,
    snapshot: &[u8],
) -> Result<ReconciliationCounts, ComhairleError> {
    let mut features = Vec::new();
    for mut feature in normalize_snapshot(definition, snapshot)? {
        let geometry =
            prepare_geometry(connection, &mut feature, definition.repair_invalid_geometry).await?;
        features.push(PreparedFeature { feature, geometry });
    }

    let mut counts = ReconciliationCounts {
        repaired: features
            .iter()
            .filter(|prepared| matches!(prepared.geometry, GeometryOutcome::Repaired))
            .map(|prepared| prepared.feature.external_id.clone())
            .collect(),
        ..Default::default()
    };
    for prepared in &features {
        let feature = &prepared.feature;
        if !definition.tags.rules.is_empty() && feature.tags.matched_rules == 0 {
            counts.unmatched_ids.push(feature.external_id.clone());
        }
        let (sql, values) = Feature::owned_entities(pipeline_id)
            .column((FeatureIden::Table, sea_query::Asterisk))
            .column((RegionAreaIden::Table, RegionAreaIden::Name))
            .expr_as(
                Func::coalesce([
                    postgis(
                        "st_equals",
                        [
                            Expr::col((RegionAreaIden::Table, RegionAreaIden::AreaGeometry)).into(),
                            geometry(feature),
                        ],
                    ),
                    Expr::val(false).into(),
                ]),
                Alias::new("geometry_equal"),
            )
            .expr_as(
                Expr::col((RegionIden::Table, RegionIden::Name)),
                Alias::new("region_name"),
            )
            .and_where(
                Expr::col((FeatureIden::Table, FeatureIden::ExternalId)).eq(&feature.external_id),
            )
            .build_sqlx(PostgresQueryBuilder);
        let existing = sqlx::query_as_with::<_, ExistingFeature, _>(&sql, values)
            .fetch_optional(&mut *connection)
            .await?;
        let previous_tags = existing
            .as_ref()
            .map(|existing| existing.mapping.managed_tags.as_slice())
            .unwrap_or_default();
        if let Some(change) = managed_tag_change(feature, previous_tags) {
            counts.tag_changes.push(change);
        }
        let change = match existing {
            Some(existing) => {
                update_feature(connection, pipeline_id, definition, prepared, existing).await?
            }
            None => {
                create_feature(connection, pipeline_id, definition, prepared).await?;
                FeatureChange::Created
            }
        };
        match change {
            FeatureChange::Created => {
                counts.created += 1;
                counts.created_ids.push(feature.external_id.clone());
            }
            FeatureChange::Updated => {
                counts.updated += 1;
                counts.updated_ids.push(feature.external_id.clone());
            }
            FeatureChange::Restored => {
                counts.restored += 1;
                counts.restored_ids.push(feature.external_id.clone());
            }
            FeatureChange::Unchanged => counts.unchanged += 1,
        }
    }

    let ids: Vec<_> = features
        .iter()
        .map(|prepared| prepared.feature.external_id.clone())
        .collect();
    let (sql, values) = Query::update()
        .table(FeatureIden::Table)
        .value(FeatureIden::RetiredAt, Expr::current_timestamp())
        .value(FeatureIden::Status, FeatureStatus::Retired.as_ref())
        .and_where(Expr::col(FeatureIden::PipelineId).eq(pipeline_id))
        .and_where(Expr::col(FeatureIden::Status).eq(FeatureStatus::Live.as_ref()))
        .and_where(Expr::col(FeatureIden::ExternalId).ne(PgFunc::all(ids)))
        .returning_col(FeatureIden::ExternalId)
        .build_sqlx(PostgresQueryBuilder);
    counts.retired_ids = sqlx::query_scalar_with(&sql, values)
        .fetch_all(connection)
        .await?;
    counts.retired_ids.sort();
    counts.retired = counts.retired_ids.len();
    Ok(counts)
}

fn managed_tag_change(feature: &SourceFeature, previous: &[String]) -> Option<ManagedTagChange> {
    let before: BTreeSet<_> = previous.iter().cloned().collect();
    let after: BTreeSet<_> = feature.tags.tags.iter().cloned().collect();
    let added = after.difference(&before).cloned().collect::<Vec<_>>();
    let removed = before.difference(&after).cloned().collect::<Vec<_>>();
    if added.is_empty() && removed.is_empty() {
        return None;
    }
    Some(ManagedTagChange {
        external_id: feature.external_id.clone(),
        added,
        removed,
    })
}

async fn prepare_geometry(
    connection: &mut PgConnection,
    feature: &mut SourceFeature,
    allow_repair: bool,
) -> Result<GeometryOutcome, ComhairleError> {
    let (sql, values) = Query::select()
        .expr(valid_boundary(geometry(feature)))
        .build_sqlx(PostgresQueryBuilder);
    let valid: bool = sqlx::query_scalar_with(&sql, values)
        .fetch_one(&mut *connection)
        .await?;
    if valid {
        return Ok(GeometryOutcome::Original);
    }
    if !allow_repair {
        return Err(ComhairleError::BadRequest(format!(
            "Feature {} has an invalid boundary",
            feature.external_id
        )));
    }
    let repaired = Query::select()
        .expr_as(
            postgis("st_makevalid", [geometry(feature)]),
            Alias::new("geometry"),
        )
        .to_owned();
    let candidate: SimpleExpr = Expr::col((Alias::new("repaired"), Alias::new("geometry"))).into();
    let polygons = postgis(
        "st_multi",
        [postgis(
            "st_collectionextract",
            [candidate.clone(), Expr::val(3).into()],
        )],
    );
    let (sql, values) = Query::select()
        .expr(
            postgis(
                "st_isempty",
                [postgis(
                    "st_collectionextract",
                    [candidate.clone(), Expr::val(1).into()],
                )],
            )
            .and(postgis(
                "st_isempty",
                [postgis(
                    "st_collectionextract",
                    [candidate, Expr::val(2).into()],
                )],
            )),
        )
        .expr(valid_boundary(polygons.clone()))
        .expr(
            Expr::expr(postgis("st_asgeojson", [polygons, Expr::val(15).into()]))
                .cast_as(Alias::new("jsonb")),
        )
        .from_subquery(repaired, Alias::new("repaired"))
        .build_sqlx(PostgresQueryBuilder);
    let (only_polygons, valid, repaired): (bool, bool, serde_json::Value) =
        sqlx::query_as_with(&sql, values)
            .fetch_one(connection)
            .await?;
    if !only_polygons {
        return Err(ComhairleError::BadRequest(format!(
            "Repairing feature {} would discard non-polygon geometry",
            feature.external_id
        )));
    }
    if !valid || !super::valid_geometry_structure(&repaired) {
        return Err(ComhairleError::BadRequest(format!(
            "Repairing feature {} did not produce a valid polygon",
            feature.external_id
        )));
    }
    feature.geometry = repaired;
    Ok(GeometryOutcome::Repaired)
}

async fn create_feature(
    connection: &mut PgConnection,
    pipeline_id: Uuid,
    definition: &PipelineDefinition,
    prepared: &PreparedFeature,
) -> Result<(), ComhairleError> {
    let feature = &prepared.feature;
    let primary_name = feature
        .region_names
        .get(&definition.primary_locale)
        .ok_or_else(|| ComhairleError::BadRequest("Missing primary Region name".into()))?;
    let name = new_translation(
        connection,
        TranslationScope::System,
        &definition.primary_locale,
        primary_name,
        TextFormat::Plain,
    )
    .await?;
    let description = new_translation(
        connection,
        TranslationScope::System,
        &definition.primary_locale,
        "",
        TextFormat::Plain,
    )
    .await?;
    update_names(connection, name.id, feature, &definition.primary_locale).await?;
    let (sql, values) = Query::insert()
        .into_table(RegionIden::Table)
        .columns([
            RegionIden::Name,
            RegionIden::Description,
            RegionIden::RegionType,
            RegionIden::OfficialId,
        ])
        .values([
            name.id.into(),
            description.id.into(),
            "official".into(),
            feature.external_id.clone().into(),
        ])?
        .returning_col(RegionIden::Id)
        .build_sqlx(PostgresQueryBuilder);
    let region_id: Uuid = sqlx::query_scalar_with(&sql, values)
        .fetch_one(&mut *connection)
        .await?;
    let (sql, values) = Query::insert()
        .into_table(RegionAreaIden::Table)
        .columns([
            RegionAreaIden::Name,
            RegionAreaIden::Tags,
            RegionAreaIden::AreaGeometry,
        ])
        .values([
            feature.area_name.clone().into(),
            feature.tags.tags.clone().into(),
            geometry(feature),
        ])?
        .returning_col(RegionAreaIden::Id)
        .build_sqlx(PostgresQueryBuilder);
    let area_id: Uuid = sqlx::query_scalar_with(&sql, values)
        .fetch_one(&mut *connection)
        .await?;
    let (sql, values) = Query::insert()
        .into_table(AreaLinkIden::Table)
        .columns([AreaLinkIden::RegionId, AreaLinkIden::RegionAreaId])
        .values([region_id.into(), area_id.into()])?
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values)
        .execute(&mut *connection)
        .await?;
    let (sql, values) = Query::insert()
        .into_table(FeatureIden::Table)
        .columns([
            FeatureIden::PipelineId,
            FeatureIden::ExternalId,
            FeatureIden::RegionAreaId,
            FeatureIden::RegionId,
            FeatureIden::ManagedTags,
        ])
        .values([
            pipeline_id.into(),
            feature.external_id.clone().into(),
            area_id.into(),
            region_id.into(),
            feature.tags.tags.clone().into(),
        ])?
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values)
        .execute(&mut *connection)
        .await?;
    Ok(())
}

async fn update_feature(
    connection: &mut PgConnection,
    pipeline_id: Uuid,
    definition: &PipelineDefinition,
    prepared: &PreparedFeature,
    existing: ExistingFeature,
) -> Result<FeatureChange, ComhairleError> {
    let feature = &prepared.feature;
    let tags: Vec<String> = feature
        .tags
        .tags
        .iter()
        .chain(&existing.mapping.manual_tags)
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let old_tags: Vec<String> = existing
        .mapping
        .managed_tags
        .iter()
        .chain(&existing.mapping.manual_tags)
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let area_changed = existing.name.as_deref() != Some(&feature.area_name)
        || !existing.geometry_equal
        || tags != old_tags;
    if area_changed {
        let (sql, values) = Query::update()
            .table(RegionAreaIden::Table)
            .value(RegionAreaIden::Name, feature.area_name.clone())
            .value(RegionAreaIden::Tags, tags)
            .value(RegionAreaIden::AreaGeometry, geometry(feature))
            .value(RegionAreaIden::UpdatedAt, Expr::current_timestamp())
            .and_where(Expr::col(RegionAreaIden::Id).eq(existing.mapping.region_area_id))
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values)
            .execute(&mut *connection)
            .await?;
    }
    let names_changed = update_names(
        connection,
        existing.region_name,
        feature,
        &definition.primary_locale,
    )
    .await?;
    let (sql, values) = Query::insert()
        .into_table(AreaLinkIden::Table)
        .columns([AreaLinkIden::RegionId, AreaLinkIden::RegionAreaId])
        .values([
            existing.mapping.region_id.into(),
            existing.mapping.region_area_id.into(),
        ])?
        .on_conflict(
            OnConflict::columns([AreaLinkIden::RegionId, AreaLinkIden::RegionAreaId])
                .do_nothing()
                .to_owned(),
        )
        .build_sqlx(PostgresQueryBuilder);
    let link = sqlx::query_with(&sql, values)
        .execute(&mut *connection)
        .await?;
    if existing.mapping.status == FeatureStatus::Retired
        || existing.mapping.managed_tags != feature.tags.tags
    {
        let (sql, values) = Query::update()
            .table(FeatureIden::Table)
            .value(FeatureIden::ManagedTags, feature.tags.tags.clone())
            .value(FeatureIden::Status, FeatureStatus::Live.as_ref())
            .value(
                FeatureIden::RetiredAt,
                Option::<chrono::DateTime<chrono::Utc>>::None,
            )
            .and_where(Expr::col(FeatureIden::PipelineId).eq(pipeline_id))
            .and_where(Expr::col(FeatureIden::ExternalId).eq(&feature.external_id))
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values)
            .execute(&mut *connection)
            .await?;
    }
    Ok(match existing.mapping.status {
        FeatureStatus::Retired => FeatureChange::Restored,
        FeatureStatus::Live if area_changed || names_changed || link.rows_affected() > 0 => {
            FeatureChange::Updated
        }
        FeatureStatus::Live => FeatureChange::Unchanged,
    })
}

async fn update_names(
    connection: &mut PgConnection,
    content_id: TextContentId,
    feature: &SourceFeature,
    primary_locale: &str,
) -> Result<bool, ComhairleError> {
    let (sql, values) = Query::update()
        .table(TextContentIden::Table)
        .value(TextContentIden::PrimaryLocale, primary_locale.to_owned())
        .value(TextContentIden::UpdatedAt, Expr::current_timestamp())
        .and_where(Expr::col(TextContentIden::Id).eq(content_id))
        .and_where(Expr::col(TextContentIden::PrimaryLocale).ne(primary_locale))
        .build_sqlx(PostgresQueryBuilder);
    let mut changed = sqlx::query_with(&sql, values)
        .execute(&mut *connection)
        .await?
        .rows_affected()
        > 0;
    for (locale, content) in &feature.region_names {
        let (sql, values) = Query::select()
            .columns([TextTranslationIden::Id, TextTranslationIden::Content])
            .from(TextTranslationIden::Table)
            .and_where(Expr::col(TextTranslationIden::ContentId).eq(content_id))
            .and_where(Expr::col(TextTranslationIden::Locale).eq(locale))
            .build_sqlx(PostgresQueryBuilder);
        let existing: Option<(Uuid, String)> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *connection)
            .await?;
        match existing {
            Some((id, old_content)) if old_content != *content => {
                let (sql, values) = Query::update()
                    .table(TextTranslationIden::Table)
                    .value(TextTranslationIden::Content, content.clone())
                    .value(TextTranslationIden::UpdatedAt, Expr::current_timestamp())
                    .and_where(Expr::col(TextTranslationIden::Id).eq(id))
                    .build_sqlx(PostgresQueryBuilder);
                sqlx::query_with(&sql, values)
                    .execute(&mut *connection)
                    .await?;
                changed = true;
            }
            None => {
                create_text_translation(
                    &mut *connection,
                    &CreateTextTranslation {
                        content_id,
                        locale: locale.clone(),
                        content: content.clone(),
                        ai_generated: Some(false),
                        requires_validation: Some(false),
                    },
                )
                .await?;
                changed = true;
            }
            _ => {}
        }
    }
    Ok(changed)
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;

    fn definition() -> Result<PipelineDefinition, serde_json::Error> {
        serde_json::from_value(json!({
            "name": "Example",
            "source": {"type": "uploaded_file", "upload_id": "6226a2a3-f323-43f3-93a7-bca0585e940d"},
            "external_id_field": "code",
            "area_name_field": "name",
            "primary_locale": "en",
            "region_name_fields": {"en": "name", "cy": "welsh_name"},
            "tags": {"fixed": ["kind:country"]},
            "interval_minutes": 60,
            "attribution": "Example attribution"
        }))
    }

    fn feature(code: &str, name: &str, size: i32) -> Value {
        json!({
            "type": "Feature",
            "properties": {"code": code, "name": name},
            "geometry": {"type": "Polygon", "coordinates": [
                [[0,0],[size,0],[size,size],[0,0]]
            ]}
        })
    }

    fn snapshot(features: Vec<Value>) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&json!({"type": "FeatureCollection", "features": features}))
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn reconciles_without_replacing_references(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let definition = definition()?;
        let (sql, values) = Query::insert()
            .into_table(PipelineIden::Table)
            .columns([PipelineIden::DraftDefinition])
            .values([serde_json::to_value(&definition)?.into()])?
            .returning_col(PipelineIden::Id)
            .build_sqlx(PostgresQueryBuilder);
        let pipeline_id: Uuid = sqlx::query_scalar_with(&sql, values)
            .fetch_one(&pool)
            .await?;
        let mut alpha = feature("a", "Alpha", 2);
        alpha["properties"]["welsh_name"] = json!("Alffa");
        let initial = snapshot(vec![alpha, feature("b", "Beta", 3)])?;
        let features_query = || {
            Query::select()
                .columns([
                    FeatureIden::ExternalId,
                    FeatureIden::RegionAreaId,
                    FeatureIden::RegionId,
                ])
                .from(FeatureIden::Table)
                .and_where(Expr::col(FeatureIden::PipelineId).eq(pipeline_id))
                .order_by(FeatureIden::ExternalId, sea_query::Order::Asc)
                .build_sqlx(PostgresQueryBuilder)
        };
        assert_eq!(
            preview(&pool, pipeline_id, &definition, &initial)
                .await?
                .created,
            2
        );
        let (sql, values) = features_query();
        let after_preview: Vec<(String, Uuid, Uuid)> =
            sqlx::query_as_with(&sql, values).fetch_all(&pool).await?;
        assert!(after_preview.is_empty());
        assert_eq!(
            apply(&pool, pipeline_id, &definition, &initial)
                .await?
                .created,
            2
        );
        let (sql, values) = features_query();
        let before: Vec<(String, Uuid, Uuid)> =
            sqlx::query_as_with(&sql, values).fetch_all(&pool).await?;
        assert_eq!(
            apply(&pool, pipeline_id, &definition, &initial)
                .await?
                .unchanged,
            2
        );
        let mut invalid = feature("b", "Beta", 3);
        invalid["geometry"]["coordinates"] = json!([[[0, 0], [2, 2], [0, 2], [2, 0], [0, 0]]]);
        let invalid_snapshot = snapshot(vec![feature("a", "Invalid change", 4), invalid])?;
        assert!(
            apply(&pool, pipeline_id, &definition, &invalid_snapshot)
                .await
                .is_err()
        );
        let (sql, values) = Query::select()
            .column(RegionAreaIden::Name)
            .from(RegionAreaIden::Table)
            .and_where(Expr::col(RegionAreaIden::Id).eq(before[0].1))
            .build_sqlx(PostgresQueryBuilder);
        let name: Option<String> = sqlx::query_scalar_with(&sql, values)
            .fetch_one(&pool)
            .await?;
        assert_eq!(name.as_deref(), Some("Alpha"));

        let manual_tags = vec!["local:important".to_owned()];
        let (sql, values) = Query::update()
            .table(FeatureIden::Table)
            .value(FeatureIden::ManualTags, manual_tags.clone())
            .and_where(Expr::col(FeatureIden::PipelineId).eq(pipeline_id))
            .and_where(Expr::col(FeatureIden::ExternalId).eq("a"))
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values).execute(&pool).await?;
        let changed = snapshot(vec![feature("a", "Alpha revised", 4)])?;
        let result = apply(&pool, pipeline_id, &definition, &changed).await?;
        assert_eq!((result.updated, result.retired), (1, 1));
        let (sql, values) = Query::select()
            .column(RegionAreaIden::Tags)
            .from(RegionAreaIden::Table)
            .and_where(Expr::col(RegionAreaIden::Id).eq(before[0].1))
            .build_sqlx(PostgresQueryBuilder);
        let tags: Vec<String> = sqlx::query_scalar_with(&sql, values)
            .fetch_one(&pool)
            .await?;
        assert!(tags.contains(&manual_tags[0]));
        let (sql, values) = Query::select()
            .column(RegionIden::Name)
            .from(RegionIden::Table)
            .and_where(Expr::col(RegionIden::Id).eq(before[0].2))
            .build_sqlx(PostgresQueryBuilder);
        let name_content: TextContentId = sqlx::query_scalar_with(&sql, values)
            .fetch_one(&pool)
            .await?;
        let (sql, values) = Query::select()
            .column(TextTranslationIden::Content)
            .from(TextTranslationIden::Table)
            .and_where(Expr::col(TextTranslationIden::ContentId).eq(name_content))
            .and_where(Expr::col(TextTranslationIden::Locale).eq("cy"))
            .build_sqlx(PostgresQueryBuilder);
        let welsh_name: String = sqlx::query_scalar_with(&sql, values)
            .fetch_one(&pool)
            .await?;
        assert_eq!(welsh_name, "Alffa");
        let result = apply(&pool, pipeline_id, &definition, &initial).await?;
        assert_eq!(result.restored, 1);
        let (sql, values) = features_query();
        let after: Vec<(String, Uuid, Uuid)> =
            sqlx::query_as_with(&sql, values).fetch_all(&pool).await?;
        assert_eq!(before, after);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn repairs_polygons_without_discarding_lower_dimensional_parts(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let definition = definition()?;
        let mut connection = pool.acquire().await?;
        let mut crossed = feature("crossed", "Crossed", 1);
        crossed["geometry"]["coordinates"] = json!([[[0, 0], [1, 1], [0, 1], [1, 0], [0, 0]]]);
        let mut features = normalize_snapshot(&definition, &snapshot(vec![crossed])?)?;
        let crossed_feature = &mut features[0];
        assert!(
            prepare_geometry(&mut connection, crossed_feature, false)
                .await
                .is_err()
        );
        assert!(matches!(
            prepare_geometry(&mut connection, crossed_feature, true).await?,
            GeometryOutcome::Repaired
        ));
        assert!(matches!(
            prepare_geometry(&mut connection, crossed_feature, true).await?,
            GeometryOutcome::Original
        ));
        let mut collapsed = feature("collapsed", "Collapsed", 1);
        collapsed["geometry"]["coordinates"] = json!([[[0, 0], [1, 0], [2, 0], [0, 0]]]);
        let mut features = normalize_snapshot(&definition, &snapshot(vec![collapsed])?)?;
        assert!(matches!(
            prepare_geometry(&mut connection, &mut features[0], true).await,
            Err(ComhairleError::BadRequest(message)) if message.contains("non-polygon")
        ));
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn deletes_managed_regions_only_through_their_area(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let definition = definition()?;
        let (sql, values) = Query::insert()
            .into_table(PipelineIden::Table)
            .columns([PipelineIden::DraftDefinition])
            .values([serde_json::to_value(&definition)?.into()])?
            .returning_col(PipelineIden::Id)
            .build_sqlx(PostgresQueryBuilder);
        let pipeline_id: Uuid = sqlx::query_scalar_with(&sql, values)
            .fetch_one(&pool)
            .await?;
        apply(
            &pool,
            pipeline_id,
            &definition,
            &snapshot(vec![feature("a", "Alpha", 2)])?,
        )
        .await?;
        let (sql, values) = Query::select()
            .columns([FeatureIden::RegionAreaId, FeatureIden::RegionId])
            .from(FeatureIden::Table)
            .build_sqlx(PostgresQueryBuilder);
        let (area_id, region_id): (Uuid, Uuid) =
            sqlx::query_as_with(&sql, values).fetch_one(&pool).await?;

        assert!(matches!(
            crate::models::region::delete(&pool, &region_id).await,
            Err(ComhairleError::PipelineManagedRegion)
        ));

        crate::models::region_area::delete(&pool, &area_id).await?;
        assert!(matches!(
            crate::models::region::get_by_id(&pool, &region_id).await,
            Err(ComhairleError::ResourceNotFound(_))
        ));
        let (sql, values) = Query::select()
            .expr(Expr::col(FeatureIden::RegionId).count())
            .from(FeatureIden::Table)
            .build_sqlx(PostgresQueryBuilder);
        let remaining: i64 = sqlx::query_scalar_with(&sql, values)
            .fetch_one(&pool)
            .await?;
        assert_eq!(remaining, 0);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn deletes_pipelines_with_or_without_their_areas(
        pool: PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let definition = definition()?;
        let mut owned = Vec::new();
        for code in ["kept", "removed"] {
            let (sql, values) = Query::insert()
                .into_table(PipelineIden::Table)
                .columns([PipelineIden::DraftDefinition])
                .values([serde_json::to_value(&definition)?.into()])?
                .returning_col(PipelineIden::Id)
                .build_sqlx(PostgresQueryBuilder);
            let pipeline_id: Uuid = sqlx::query_scalar_with(&sql, values)
                .fetch_one(&pool)
                .await?;
            let size = if code == "kept" { 2 } else { 5 };
            apply(
                &pool,
                pipeline_id,
                &definition,
                &snapshot(vec![feature(code, code, size)])?,
            )
            .await?;
            let (sql, values) = Query::select()
                .columns([FeatureIden::RegionAreaId, FeatureIden::RegionId])
                .from(FeatureIden::Table)
                .and_where(Expr::col(FeatureIden::PipelineId).eq(pipeline_id))
                .build_sqlx(PostgresQueryBuilder);
            let (area_id, region_id): (Uuid, Uuid) =
                sqlx::query_as_with(&sql, values).fetch_one(&pool).await?;
            owned.push((pipeline_id, area_id, region_id));
        }
        let [
            (kept_pipeline, kept_area, kept_region),
            (removed_pipeline, removed_area, removed_region),
        ] = owned[..]
        else {
            unreachable!()
        };

        assert_eq!(
            super::super::lifecycle::delete(&pool, kept_pipeline, false).await?,
            0
        );
        crate::models::region_area::get_by_id(&pool, &kept_area).await?;
        crate::models::region::delete(&pool, &kept_region).await?;

        assert_eq!(
            super::super::lifecycle::delete(&pool, removed_pipeline, true).await?,
            1
        );
        assert!(matches!(
            crate::models::region_area::get_by_id(&pool, &removed_area).await,
            Err(ComhairleError::ResourceNotFound(_))
        ));
        assert!(matches!(
            crate::models::region::get_by_id(&pool, &removed_region).await,
            Err(ComhairleError::ResourceNotFound(_))
        ));
        let (sql, values) = Query::select()
            .expr(Expr::col(PipelineIden::Id).count())
            .from(PipelineIden::Table)
            .build_sqlx(PostgresQueryBuilder);
        let pipelines: i64 = sqlx::query_scalar_with(&sql, values)
            .fetch_one(&pool)
            .await?;
        assert_eq!(pipelines, 0);
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    #[ignore = "requires a locally supplied pipeline definition and snapshot"]
    async fn reconciles_local_snapshot(pool: PgPool) -> Result<(), Box<dyn std::error::Error>> {
        let definition_path = std::env::var("PIPELINE_TEST_DEFINITION")?;
        let snapshot_path = std::env::var("PIPELINE_TEST_SNAPSHOT")?;
        let expected_count: usize = std::env::var("PIPELINE_TEST_COUNT")?.parse()?;
        let definition: PipelineDefinition =
            serde_json::from_slice(&std::fs::read(definition_path)?)?;
        let snapshot = std::fs::read(snapshot_path)?;
        let (sql, values) = Query::insert()
            .into_table(PipelineIden::Table)
            .columns([PipelineIden::DraftDefinition])
            .values([serde_json::to_value(&definition)?.into()])?
            .returning_col(PipelineIden::Id)
            .build_sqlx(PostgresQueryBuilder);
        let pipeline_id: Uuid = sqlx::query_scalar_with(&sql, values)
            .fetch_one(&pool)
            .await?;
        let first = apply(&pool, pipeline_id, &definition, &snapshot).await?;
        let second = apply(&pool, pipeline_id, &definition, &snapshot).await?;
        assert_eq!(first.created, expected_count);
        assert_eq!(second.unchanged, expected_count);
        if let Ok(expected_repair) = std::env::var("PIPELINE_TEST_REPAIRED_ID") {
            assert!(first.repaired.contains(&expected_repair));
            assert!(second.repaired.contains(&expected_repair));
        }
        Ok(())
    }
}
