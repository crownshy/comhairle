use chrono::{Duration, Utc};
use sea_query::{Expr, LockType, OnConflict, Order, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{Acquire, PgConnection, PgPool};
use uuid::Uuid;

use super::reconcile::{ReconciliationCounts, reconcile_locked};
use super::source::load_source;
use super::tables::{
    Pipeline, PipelineIden, PipelineStatus, RunStatus, Snapshot, SnapshotIden, SnapshotKind,
};
use super::{PipelineDefinition, PipelineSource, state};
use crate::bulk_storage_service::BulkStorageService;
use crate::error::ComhairleError;

type Result<T> = std::result::Result<T, ComhairleError>;

/// A claim is created only after the Running transition has committed.
struct RunClaim {
    pipeline_id: Uuid,
    lease_id: Uuid,
    definition: PipelineDefinition,
}

/// Captures the draft revision before fetching outside the database transaction.
struct PendingPreview {
    pipeline_id: Uuid,
    generation: i64,
    definition: PipelineDefinition,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RunOutcome {
    NotDue,
    Published,
    Superseded,
}

impl Pipeline {
    fn advance(&mut self) -> Result<()> {
        self.generation = self.generation.checked_add(1).ok_or_else(|| {
            ComhairleError::CorruptedData("Pipeline generation counter overflowed".into())
        })?;
        self.updated_at = Utc::now();
        Ok(())
    }

    fn stage(&mut self, definition: PipelineDefinition) -> Result<()> {
        self.status = match self.status {
            PipelineStatus::Active | PipelineStatus::Staged
                if self.active_definition.as_ref() != Some(&definition) =>
            {
                PipelineStatus::Staged
            }
            PipelineStatus::Active | PipelineStatus::Staged => PipelineStatus::Active,
            other => other,
        };
        self.draft_definition = definition;
        self.advance()
    }

    fn clear_lease(&mut self) {
        self.lease_id = None;
        self.lease_expires_at = None;
        self.run_started_at = None;
    }

    fn complete(&mut self, definition: &PipelineDefinition) -> Result<()> {
        let now = Utc::now();
        self.run_status = RunStatus::Waiting;
        self.next_run_at = Some(now + Duration::minutes(i64::from(definition.interval_minutes)));
        self.last_success_at = Some(now);
        self.last_error = None;
        self.clear_lease();
        self.advance()
    }

    fn owns(&self, claim: &RunClaim) -> bool {
        self.status.is_active()
            && self.run_status == RunStatus::Running
            && self.lease_id == Some(claim.lease_id)
            && self.active_definition.as_ref() == Some(&claim.definition)
    }

    async fn save(&self, connection: &mut PgConnection) -> Result<()> {
        let (sql, values) = Query::update()
            .table(PipelineIden::Table)
            .value(
                PipelineIden::DraftDefinition,
                serde_json::to_value(&self.draft_definition)?,
            )
            .value(
                PipelineIden::ActiveDefinition,
                self.active_definition
                    .as_ref()
                    .map(serde_json::to_value)
                    .transpose()?,
            )
            .value(PipelineIden::Status, self.status.as_ref())
            .value(PipelineIden::RunStatus, self.run_status.as_ref())
            .value(PipelineIden::Generation, self.generation)
            .value(PipelineIden::NextRunAt, self.next_run_at)
            .value(PipelineIden::LeaseId, self.lease_id)
            .value(PipelineIden::LeaseExpiresAt, self.lease_expires_at)
            .value(PipelineIden::RunStartedAt, self.run_started_at)
            .value(PipelineIden::LastSuccessAt, self.last_success_at)
            .value(PipelineIden::LastError, self.last_error.clone())
            .value(PipelineIden::UpdatedAt, self.updated_at)
            .and_where(Expr::col(PipelineIden::Id).eq(self.id))
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values).execute(connection).await?;
        Ok(())
    }
}

pub async fn list(db: &PgPool) -> Result<Vec<Pipeline>> {
    let (sql, values) = Pipeline::select()
        .order_by(PipelineIden::CreatedAt, Order::Asc)
        .order_by(PipelineIden::Id, Order::Asc)
        .build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_as_with(&sql, values).fetch_all(db).await?)
}

async fn locked(connection: &mut PgConnection, id: Uuid) -> Result<Pipeline> {
    let (sql, values) = Pipeline::select()
        .and_where(Expr::col(PipelineIden::Id).eq(id))
        .lock(LockType::Update)
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_as_with(&sql, values)
        .fetch_optional(connection)
        .await?
        .ok_or_else(|| ComhairleError::ResourceNotFound("Pipeline".into()))
}

pub async fn create(db: &PgPool, definition: &PipelineDefinition) -> Result<Pipeline> {
    definition.validate()?;
    let (sql, values) = Query::insert()
        .into_table(PipelineIden::Table)
        .columns([PipelineIden::DraftDefinition])
        .values([serde_json::to_value(definition)?.into()])?
        .returning_all()
        .build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_as_with(&sql, values).fetch_one(db).await?)
}

pub async fn replace_draft(
    db: &PgPool,
    id: Uuid,
    definition: &PipelineDefinition,
) -> Result<Pipeline> {
    definition.validate()?;
    let mut tx = db.begin().await?;
    let mut pipeline = locked(&mut tx, id).await?;
    pipeline.stage(definition.clone())?;
    pipeline.save(&mut tx).await?;
    delete_preview(&mut tx, id).await?;
    tx.commit().await?;
    Ok(pipeline)
}

pub async fn preview(
    db: &PgPool,
    id: Uuid,
    storage: Option<&dyn BulkStorageService>,
) -> Result<ReconciliationCounts> {
    let pending = {
        let mut tx = db.begin().await?;
        let mut pipeline = locked(&mut tx, id).await?;
        pipeline.advance()?;
        pipeline.save(&mut tx).await?;
        delete_preview(&mut tx, id).await?;
        tx.commit().await?;
        PendingPreview {
            pipeline_id: id,
            generation: pipeline.generation,
            definition: pipeline.draft_definition,
        }
    };
    pending.prepare(db, storage).await
}

impl PendingPreview {
    async fn prepare(
        self,
        db: &PgPool,
        storage: Option<&dyn BulkStorageService>,
    ) -> Result<ReconciliationCounts> {
        let bytes = load_source(&self.definition.source, storage).await?;
        let mut tx = db.begin().await?;
        let pipeline = locked(&mut tx, self.pipeline_id).await?;
        if pipeline.generation != self.generation || pipeline.draft_definition != self.definition {
            return Err(stale_preview());
        }
        let state_hash = state::lock_and_hash(&mut tx, pipeline.id).await?;
        let mut simulation = tx.begin().await?;
        let result = reconcile_locked(&mut simulation, pipeline.id, &self.definition, &bytes).await;
        simulation.rollback().await?;
        let snapshot = Snapshot {
            pipeline_id: pipeline.id,
            kind: SnapshotKind::Preview,
            definition: self.definition,
            snapshot: bytes,
            preview_generation: Some(self.generation),
            state_hash: Some(state_hash),
            summary: result.map_err(reconciliation_error)?,
            created_at: Utc::now(),
        };
        snapshot.save(&mut tx).await?;
        tx.commit().await?;
        Ok(snapshot.summary)
    }
}

pub async fn get_preview(db: &PgPool, id: Uuid) -> Result<Option<ReconciliationCounts>> {
    let mut tx = db.begin().await?;
    locked(&mut tx, id).await?;
    let preview = Snapshot::get(&mut tx, id, SnapshotKind::Preview).await?;
    tx.commit().await?;
    Ok(preview.map(|snapshot| snapshot.summary))
}

pub async fn activate(
    db: &PgPool,
    id: Uuid,
    storage: Option<&dyn BulkStorageService>,
) -> Result<Pipeline> {
    let mut tx = db.begin().await?;
    let mut pipeline = locked(&mut tx, id).await?;
    let mut preview = Snapshot::get(&mut tx, id, SnapshotKind::Preview)
        .await?
        .ok_or_else(stale_preview)?;
    if preview.preview_generation != Some(pipeline.generation)
        || preview.definition != pipeline.draft_definition
        || preview.state_hash != Some(state::lock_and_hash(&mut tx, id).await?)
    {
        return Err(stale_preview());
    }
    if matches!(
        preview.definition.source,
        PipelineSource::UploadedFile { .. }
    ) && storage.is_none()
    {
        return Err(ComhairleError::NoBulkStorageServiceConfigured);
    }
    // The fingerprint guards database state; publication uses the exact previewed bytes.
    reconcile_locked(&mut tx, id, &preview.definition, &preview.snapshot)
        .await
        .map_err(reconciliation_error)?;
    pipeline.status = PipelineStatus::Active;
    pipeline.active_definition = Some(preview.definition.clone());
    pipeline.complete(&preview.definition)?;
    pipeline.save(&mut tx).await?;
    preview.kind = SnapshotKind::Successful;
    preview.preview_generation = None;
    preview.state_hash = None;
    preview.created_at = Utc::now();
    preview.save(&mut tx).await?;
    delete_preview(&mut tx, id).await?;
    tx.commit().await?;
    Ok(pipeline)
}

pub async fn deactivate(db: &PgPool, id: Uuid) -> Result<Pipeline> {
    let mut tx = db.begin().await?;
    let mut pipeline = locked(&mut tx, id).await?;
    pipeline.status = match pipeline.status {
        PipelineStatus::Draft => PipelineStatus::Draft,
        _ => PipelineStatus::Deactivated,
    };
    pipeline.run_status = RunStatus::Unscheduled;
    pipeline.next_run_at = None;
    pipeline.clear_lease();
    pipeline.advance()?;
    pipeline.save(&mut tx).await?;
    delete_preview(&mut tx, id).await?;
    tx.commit().await?;
    Ok(pipeline)
}

impl Snapshot {
    async fn get(
        connection: &mut PgConnection,
        id: Uuid,
        kind: SnapshotKind,
    ) -> Result<Option<Self>> {
        let (sql, values) = Query::select()
            .column(sea_query::Asterisk)
            .from(SnapshotIden::Table)
            .and_where(Expr::col(SnapshotIden::PipelineId).eq(id))
            .and_where(Expr::col(SnapshotIden::Kind).eq(kind.as_ref()))
            .build_sqlx(PostgresQueryBuilder);
        Ok(sqlx::query_as_with(&sql, values)
            .fetch_optional(connection)
            .await?)
    }

    async fn save(&self, connection: &mut PgConnection) -> Result<()> {
        let columns = [
            SnapshotIden::Definition,
            SnapshotIden::Snapshot,
            SnapshotIden::PreviewGeneration,
            SnapshotIden::StateHash,
            SnapshotIden::Summary,
            SnapshotIden::CreatedAt,
        ];
        let (sql, values) = Query::insert()
            .into_table(SnapshotIden::Table)
            .columns(
                [SnapshotIden::PipelineId, SnapshotIden::Kind]
                    .into_iter()
                    .chain(columns),
            )
            .values([
                self.pipeline_id.into(),
                self.kind.as_ref().into(),
                serde_json::to_value(&self.definition)?.into(),
                self.snapshot.clone().into(),
                self.preview_generation.into(),
                self.state_hash.clone().into(),
                serde_json::to_value(&self.summary)?.into(),
                self.created_at.into(),
            ])?
            .on_conflict(
                OnConflict::columns([SnapshotIden::PipelineId, SnapshotIden::Kind])
                    .update_columns(columns)
                    .to_owned(),
            )
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values).execute(connection).await?;
        Ok(())
    }
}

async fn delete_preview(connection: &mut PgConnection, id: Uuid) -> Result<()> {
    let (sql, values) = Query::delete()
        .from_table(SnapshotIden::Table)
        .and_where(Expr::col(SnapshotIden::PipelineId).eq(id))
        .and_where(Expr::col(SnapshotIden::Kind).eq(SnapshotKind::Preview.as_ref()))
        .build_sqlx(PostgresQueryBuilder);
    sqlx::query_with(&sql, values).execute(connection).await?;
    Ok(())
}

pub async fn due_pipeline_ids(db: &PgPool) -> Result<Vec<Uuid>> {
    let (sql, values) = Query::select()
        .column(PipelineIden::Id)
        .from(PipelineIden::Table)
        .cond_where(Pipeline::due_condition(Utc::now()))
        .order_by(PipelineIden::NextRunAt, Order::Asc)
        .order_by(PipelineIden::Id, Order::Asc)
        .limit(100)
        .build_sqlx(PostgresQueryBuilder);
    Ok(sqlx::query_scalar_with(&sql, values).fetch_all(db).await?)
}

async fn claim_due(db: &PgPool, id: Uuid) -> Result<Option<RunClaim>> {
    let mut tx = db.begin().await?;
    let (sql, values) = Pipeline::select()
        .and_where(Expr::col(PipelineIden::Id).eq(id))
        .cond_where(Pipeline::due_condition(Utc::now()))
        .lock(LockType::Update)
        .build_sqlx(PostgresQueryBuilder);
    let Some(mut pipeline) = sqlx::query_as_with::<_, Pipeline, _>(&sql, values)
        .fetch_optional(&mut *tx)
        .await?
    else {
        return Ok(None);
    };
    let definition = pipeline
        .active_definition
        .clone()
        .ok_or_else(|| ComhairleError::CorruptedData("Active pipeline has no definition".into()))?;
    let now = Utc::now();
    let lease_id = Uuid::new_v4();
    pipeline.run_status = RunStatus::Running;
    pipeline.lease_id = Some(lease_id);
    pipeline.lease_expires_at = Some(now + Duration::minutes(10));
    pipeline.run_started_at = Some(now);
    pipeline.updated_at = now;
    pipeline.save(&mut tx).await?;
    tx.commit().await?;
    Ok(Some(RunClaim {
        pipeline_id: id,
        lease_id,
        definition,
    }))
}

pub async fn run_due(
    db: &PgPool,
    id: Uuid,
    storage: Option<&dyn BulkStorageService>,
) -> Result<RunOutcome> {
    let Some(claim) = claim_due(db, id).await? else {
        return Ok(RunOutcome::NotDue);
    };
    let result = claim.run(db, storage).await;
    if let Err(error) = &result {
        let mut tx = db.begin().await?;
        let mut pipeline = locked(&mut tx, id).await?;
        if pipeline.owns(&claim) {
            pipeline.run_status = RunStatus::RetryPending;
            pipeline.next_run_at = Some(Utc::now() + Duration::minutes(5));
            pipeline.last_error = Some(error.to_string());
            pipeline.clear_lease();
            pipeline.updated_at = Utc::now();
            pipeline.save(&mut tx).await?;
        }
        tx.commit().await?;
    }
    result
}

impl RunClaim {
    async fn run(
        &self,
        db: &PgPool,
        storage: Option<&dyn BulkStorageService>,
    ) -> Result<RunOutcome> {
        let bytes = load_source(&self.definition.source, storage).await?;
        let mut tx = db.begin().await?;
        let mut pipeline = locked(&mut tx, self.pipeline_id).await?;
        if !pipeline.owns(self) {
            tracing::info!(pipeline_id = %self.pipeline_id, "Pipeline run superseded before publication");
            return Ok(RunOutcome::Superseded);
        }
        if matches!(self.definition.source, PipelineSource::UploadedFile { .. }) {
            let previous = Snapshot::get(&mut tx, pipeline.id, SnapshotKind::Successful)
                .await?
                .ok_or_else(|| {
                    ComhairleError::CorruptedData(
                        "Uploaded pipeline has no successful snapshot".into(),
                    )
                })?;
            if previous.definition != self.definition || previous.snapshot != bytes {
                return Err(ComhairleError::Conflict(
                    "The uploaded source changed. Upload and preview a replacement.".into(),
                ));
            }
        }
        let summary = reconcile_locked(&mut tx, pipeline.id, &self.definition, &bytes)
            .await
            .map_err(reconciliation_error)?;
        pipeline.complete(&self.definition)?;
        pipeline.save(&mut tx).await?;
        Snapshot {
            pipeline_id: pipeline.id,
            kind: SnapshotKind::Successful,
            definition: self.definition.clone(),
            snapshot: bytes,
            preview_generation: None,
            state_hash: None,
            summary,
            created_at: Utc::now(),
        }
        .save(&mut tx)
        .await?;
        delete_preview(&mut tx, pipeline.id).await?;
        tx.commit().await?;
        Ok(RunOutcome::Published)
    }
}

fn stale_preview() -> ComhairleError {
    ComhairleError::Conflict("The pipeline or its data changed. Run a fresh preview.".into())
}

fn reconciliation_error(error: ComhairleError) -> ComhairleError {
    if let ComhairleError::DatabaseError(sqlx::Error::Database(ref database_error)) = error
        && database_error.code().as_deref() == Some("23505")
    {
        return ComhairleError::Conflict(
            "A Region Area conflicts with an existing boundary; no changes were published.".into(),
        );
    }
    error
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use serde_json::json;

    use super::*;
    use crate::bulk_storage_service::MockBulkStorageService;
    use crate::models::region_area::RegionAreaIden;

    fn definition() -> PipelineDefinition {
        serde_json::from_value(json!({
            "name": "Example",
            "source": {"type": "uploaded_file", "upload_id": Uuid::new_v4()},
            "external_id_field": "code", "area_name_field": "name",
            "primary_locale": "en", "region_name_fields": {"en": "name"},
            "tags": {"fixed": ["kind:country"]}, "interval_minutes": 60,
            "attribution": "Example attribution"
        }))
        .unwrap()
    }

    fn snapshot(name: &str) -> Vec<u8> {
        serde_json::to_vec(&json!({
            "type": "FeatureCollection", "features": [{
                "type": "Feature", "properties": {"code": "a", "name": name},
                "geometry": {"type": "Polygon", "coordinates": [[[0,0],[1,0],[1,1],[0,0]]]}
            }]
        }))
        .unwrap()
    }

    fn storage() -> (MockBulkStorageService, Arc<Mutex<Vec<u8>>>) {
        let bytes = Arc::new(Mutex::new(snapshot("Original")));
        let content = bytes.clone();
        let mut storage = MockBulkStorageService::new();
        storage.expect_get_file_limited().returning(move |_, _| {
            let bytes = content.lock().unwrap().clone();
            Box::pin(async move { Ok(bytes) })
        });
        (storage, bytes)
    }

    async fn due(db: &PgPool, id: Uuid) -> Result<()> {
        let (sql, values) = Query::update()
            .table(PipelineIden::Table)
            .value(PipelineIden::NextRunAt, Utc::now() - Duration::minutes(1))
            .and_where(Expr::col(PipelineIden::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values).execute(db).await?;
        Ok(())
    }

    async fn read(db: &PgPool, id: Uuid) -> Pipeline {
        let mut connection = db.acquire().await.unwrap();
        locked(&mut connection, id).await.unwrap()
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn publishes_exact_preview_and_invalidates_changed_state(db: PgPool) -> Result<()> {
        let (storage, bytes) = storage();
        let original = definition();
        let pipeline = create(&db, &original).await?;
        assert_eq!(pipeline.status, PipelineStatus::Draft);
        assert_eq!(pipeline.run_status, RunStatus::Unscheduled);
        assert!(activate(&db, pipeline.id, Some(&storage)).await.is_err());
        let summary = preview(&db, pipeline.id, Some(&storage)).await?;
        assert_eq!(summary.created, 1);
        assert_eq!(get_preview(&db, pipeline.id).await?, Some(summary));
        *bytes.lock().unwrap() = snapshot("Changed after preview");
        let pipeline = activate(&db, pipeline.id, Some(&storage)).await?;
        assert_eq!(pipeline.status, PipelineStatus::Active);
        assert_eq!(pipeline.run_status, RunStatus::Waiting);
        assert!(get_preview(&db, pipeline.id).await?.is_none());
        let (sql, values) = Query::select()
            .column(RegionAreaIden::Name)
            .from(RegionAreaIden::Table)
            .build_sqlx(PostgresQueryBuilder);
        let name: Option<String> = sqlx::query_scalar_with(&sql, values).fetch_one(&db).await?;
        assert_eq!(name.as_deref(), Some("Original"));

        preview(&db, pipeline.id, Some(&storage)).await?;
        let (sql, values) = Query::update()
            .table(RegionAreaIden::Table)
            .value(RegionAreaIden::Name, "External edit")
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values).execute(&db).await?;
        assert!(matches!(
            activate(&db, pipeline.id, Some(&storage)).await,
            Err(ComhairleError::Conflict(_))
        ));

        let mut replacement = original.clone();
        replacement.name = "Replacement".into();
        let staged = replace_draft(&db, pipeline.id, &replacement).await?;
        assert_eq!(staged.status, PipelineStatus::Staged);
        assert_eq!(staged.active_definition, Some(original.clone()));
        assert!(get_preview(&db, pipeline.id).await?.is_none());
        assert!(matches!(
            activate(&db, pipeline.id, Some(&storage)).await,
            Err(ComhairleError::Conflict(_))
        ));
        assert_eq!(
            replace_draft(&db, pipeline.id, &original).await?.status,
            PipelineStatus::Active
        );
        let stopped = deactivate(&db, pipeline.id).await?;
        assert_eq!(stopped.status, PipelineStatus::Deactivated);
        assert_eq!(stopped.run_status, RunStatus::Unscheduled);
        assert!(stopped.next_run_at.is_none());
        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn fences_reclaims_and_retries_runs(db: PgPool) -> Result<()> {
        let (storage, bytes) = storage();
        let pipeline = create(&db, &definition()).await?;
        preview(&db, pipeline.id, Some(&storage)).await?;
        activate(&db, pipeline.id, Some(&storage)).await?;
        assert_eq!(
            run_due(&db, pipeline.id, Some(&storage)).await?,
            RunOutcome::NotDue
        );
        due(&db, pipeline.id).await?;
        assert_eq!(due_pipeline_ids(&db).await?, vec![pipeline.id]);
        let (first, second) =
            tokio::join!(claim_due(&db, pipeline.id), claim_due(&db, pipeline.id));
        let mut claims: Vec<_> = [first?, second?].into_iter().flatten().collect();
        assert_eq!(claims.len(), 1);
        let old = claims.pop().unwrap();
        let (sql, values) = Query::update()
            .table(PipelineIden::Table)
            .value(
                PipelineIden::LeaseExpiresAt,
                Utc::now() - Duration::seconds(1),
            )
            .and_where(Expr::col(PipelineIden::Id).eq(pipeline.id))
            .build_sqlx(PostgresQueryBuilder);
        sqlx::query_with(&sql, values).execute(&db).await?;
        let reclaimed = claim_due(&db, pipeline.id).await?.unwrap();
        assert_eq!(old.run(&db, Some(&storage)).await?, RunOutcome::Superseded);
        deactivate(&db, pipeline.id).await?;
        assert_eq!(
            reclaimed.run(&db, Some(&storage)).await?,
            RunOutcome::Superseded
        );
        preview(&db, pipeline.id, Some(&storage)).await?;
        activate(&db, pipeline.id, Some(&storage)).await?;
        *bytes.lock().unwrap() = snapshot("Unreviewed change");
        due(&db, pipeline.id).await?;
        assert!(run_due(&db, pipeline.id, Some(&storage)).await.is_err());
        let failed = read(&db, pipeline.id).await;
        assert_eq!(failed.run_status, RunStatus::RetryPending);
        assert!(
            failed
                .last_error
                .unwrap()
                .contains("uploaded source changed")
        );
        assert!(failed.lease_id.is_none());
        *bytes.lock().unwrap() = snapshot("Original");
        due(&db, pipeline.id).await?;
        assert_eq!(
            run_due(&db, pipeline.id, Some(&storage)).await?,
            RunOutcome::Published
        );
        let recovered = read(&db, pipeline.id).await;
        assert_eq!(recovered.run_status, RunStatus::Waiting);
        assert!(recovered.last_error.is_none());
        Ok(())
    }
}
