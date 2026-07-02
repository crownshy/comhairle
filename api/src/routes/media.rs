pub mod dto;

use std::sync::Arc;

use aide::axum::{
    ApiRouter,
    routing::{delete_with, get_with, post_with},
};
use axum::extract::{Json, Multipart, Path, Query, State};
use hyper::StatusCode;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    ComhairleState,
    bulk_storage_service::FileMetadata,
    error::ComhairleError,
    models::{
        media::{self, CreateMedia, MediaContentType, MediaFilterOptions, MediaOrderOptions},
        pagination::{PageOptions, PaginatedResults},
    },
    routes::{auth::RequiredAdminUser, media::dto::MediaDto},
    tools::id::gen_id,
};

#[instrument(err(Debug), skip(state))]
async fn list(
    State(state): State<Arc<ComhairleState>>,
    Query(order_options): Query<MediaOrderOptions>,
    Query(filter_options): Query<MediaFilterOptions>,
    Query(page_options): Query<PageOptions>,
    RequiredAdminUser(user): RequiredAdminUser,
) -> Result<(StatusCode, Json<PaginatedResults<MediaDto>>), ComhairleError> {
    let results = media::list(&state.db, page_options, order_options, filter_options).await?;

    Ok((StatusCode::OK, Json(results.into())))
}

#[instrument(err(Debug), skip(state))]
async fn get(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Path(media_id): Path<Uuid>,
) -> Result<(StatusCode, Json<MediaDto>), ComhairleError> {
    let media = media::get_by_id(&state.db, &media_id).await?;

    Ok((StatusCode::OK, Json(media.into())))
}

#[instrument(err(Debug), skip(state))]
async fn upload(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    mut form_data: Multipart,
) -> Result<(StatusCode, Json<MediaDto>), ComhairleError> {
    let bulk_storage_service = state.required_bulk_storage_service()?;
    let bulk_storage_config = state
        .config
        .bulk_storage_service
        .as_ref()
        .ok_or(ComhairleError::NoBulkStorageServiceConfigured)?;

    let (filename, content_type_header, bytes) = match form_data.next_field().await? {
        Some(field) => {
            let content_type = field.content_type().map(|ct| ct.to_string());
            let filename = field
                .file_name()
                .map(|f| f.to_string())
                .unwrap_or_else(gen_id);
            let bytes = field.bytes().await?.to_vec();

            (filename, content_type, bytes)
        }
        None => return Err(ComhairleError::BadRequest("Missing form field".to_string())),
    };

    if form_data.next_field().await?.is_some() {
        return Err(ComhairleError::BadRequest(
            "Only one file upload allowed".to_string(),
        ));
    }

    let content_type = content_type_header
        .map(|ct| MediaContentType::try_from_mime(&ct))
        .unwrap_or_else(|| {
            let extension = std::path::Path::new(&filename)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("");
            MediaContentType::try_from_extension(extension)
        })?;

    let metadata = FileMetadata {
        is_public: true,
        content_type: content_type.to_string(),
    };
    let prefix = match content_type {
        MediaContentType::Jpeg
        | MediaContentType::Png
        | MediaContentType::Gif
        | MediaContentType::Webp => "images",
        MediaContentType::Mp4 | MediaContentType::Mpeg | MediaContentType::Webm => "video",
        MediaContentType::Mp3 => "audio",
    };
    let storage_key = format!("{prefix}/{filename}");
    bulk_storage_service
        .upload_file(&storage_key, bytes, metadata)
        .await?;

    let create_media = CreateMedia {
        store_name: bulk_storage_config.store_name.to_string(),
        storage_key,
        filename,
        content_type,
    };
    let media = media::create(&state.db, &create_media, &user.id).await?;

    Ok((StatusCode::CREATED, Json(media.into())))
}

#[instrument(err(Debug), skip(state))]
async fn delete(
    State(state): State<Arc<ComhairleState>>,
    RequiredAdminUser(user): RequiredAdminUser,
    Path(media_id): Path<Uuid>,
) -> Result<(StatusCode, Json<MediaDto>), ComhairleError> {
    let bulk_storage_service = state.required_bulk_storage_service()?;

    let media = media::get_by_id(&state.db, &media_id).await?;

    bulk_storage_service.delete_file(&media.storage_key).await?;

    let media = media::delete(&state.db, &media_id).await?;

    Ok((StatusCode::OK, Json(media.into())))
}

pub fn router(state: Arc<ComhairleState>) -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(list, |op| {
                op.id("ListMedia")
                    .tag("Media")
                    .summary("List media records")
                    .description("List media records")
                    .security_requirement("JWT")
                    .response::<200, Json<PaginatedResults<MediaDto>>>()
            }),
        )
        .api_route(
            "/{media_id}",
            get_with(get, |op| {
                op.id("GetMedia")
                    .tag("Media")
                    .summary("Get media record")
                    .description("Get media record by id")
                    .security_requirement("JWT")
                    .response::<200, Json<MediaDto>>()
            }),
        )
        .api_route(
            "/",
            post_with(upload, |op| {
                op.tag("Media")
                    .summary("Upload media resource")
                    .description(
                        "
Upload a media resource to the bulk_storage_service 
and create a new record in the database.\n\n
This endpoint requires multipart/form-data.\n\n\
Generated API clients may not support file uploads.\n\n\
Use FormData and a raw HTTP request.\n\n\
**Example (curl):**\n\
```bash
curl -X POST \\
-H 'Cookie: auth-token=...;' \\
'localhost:3000/media' \\
--form 'file=@/path-to-document.pdf'
```
                            ",
                    )
                    .security_requirement("JWT")
                    .response::<201, Json<MediaDto>>()
            }),
        )
        .api_route(
            "/{media_id}",
            delete_with(delete, |op| {
                op.id("DeleteMedia")
                    .tag("Media")
                    .summary("Delete media record")
                    .description("Delete media record by id")
                    .security_requirement("JWT")
                    .response::<200, Json<MediaDto>>()
            }),
        )
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    use crate::{
        bulk_storage_service::{MockBulkStorageService, UploadResult},
        models::{
            media::Media, model_test_helpers::setup_default_app_and_session,
            pagination::PaginatedResults,
        },
        setup_server,
        test_helpers::{TEST_PASSWORD, UserSession, multipart_body_builder, test_state},
    };

    use sqlx::PgPool;

    async fn create_random_image_record(
        db: &PgPool,
        user_id: &Uuid,
    ) -> Result<Media, ComhairleError> {
        let random_name = gen_id();
        let params = CreateMedia {
            store_name: "comhairle-media-test".to_string(),
            storage_key: format!("images/{random_name}.jpg"),
            filename: format!("{random_name}.jpg"),
            content_type: MediaContentType::Jpeg,
        };

        media::create(db, &params, user_id).await
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_upload_media_to_bulk_storage_and_create_record(
        pool: PgPool,
    ) -> Result<(), Box<dyn Error>> {
        let boundary = "test-boundary";
        let filename = "test_file.jpg";
        let content_type = "image/jpeg";

        let mut bulk_storage_service = MockBulkStorageService::new();
        bulk_storage_service
            .expect_upload_file()
            .once()
            .returning(move |_, _, _| {
                Box::pin(async move {
                    Ok(UploadResult {
                        url: format!("https://storage.com/{}", filename),
                    })
                })
            });
        let state = test_state()
            .db(pool)
            .bulk_storage_service(Arc::new(bulk_storage_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        let body = multipart_body_builder()
            .content("test-content")
            .boundary(boundary)
            .filename(filename)
            .content_type(content_type)
            .call();
        let (_, value, _) = session
            .post_multipart(&app, "/media", boundary, body.into())
            .await?;
        let media: MediaDto = serde_json::from_value(value)?;

        assert_eq!(media.filename, filename.to_string(), "incorrect filename");
        assert_eq!(
            media.content_type,
            MediaContentType::Jpeg,
            "incorrect content_type"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_get_media_by_id(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        session.signup(&app).await?;

        session
            .login(&app, "admin@crown-shy.com", TEST_PASSWORD)
            .await?;

        let (_, user, _) = session.current_user(&app).await?;

        let created_media = create_random_image_record(&pool, &user.id).await?;

        let (_, value, _) = session
            .get(&app, &format!("/media/{}", created_media.id))
            .await?;
        let media: MediaDto = serde_json::from_value(value)?;

        assert_eq!(media.id, created_media.id, "ids do not match");

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_list_media(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let (app, mut session) = setup_default_app_and_session(&pool).await?;
        session.signup(&app).await?;

        session
            .login(&app, "admin@crown-shy.com", TEST_PASSWORD)
            .await?;

        let (_, user, _) = session.current_user(&app).await?;

        let created_media_1 = create_random_image_record(&pool, &user.id).await?;
        let _ = create_random_image_record(&pool, &user.id).await?;
        let _ = create_random_image_record(&pool, &user.id).await?;
        let _ = create_random_image_record(&pool, &user.id).await?;

        let (_, value, _) = session.get(&app, "/media").await?;
        let results: PaginatedResults<MediaDto> = serde_json::from_value(value)?;

        assert_eq!(results.total, 4, "incorrect total");
        assert_eq!(
            results.records[0].id, created_media_1.id,
            "incorrect first id"
        );

        Ok(())
    }

    #[sqlx::test(migrator = "crate::SQLX_MIGRATOR")]
    async fn should_delete_media(pool: PgPool) -> Result<(), Box<dyn Error>> {
        let mut bulk_storage_service = MockBulkStorageService::new();
        bulk_storage_service
            .expect_delete_file()
            .once()
            .returning(|_| Box::pin(async move { Ok(()) }));
        let state = test_state()
            .db(pool.clone())
            .bulk_storage_service(Arc::new(bulk_storage_service))
            .call()?;
        let app = setup_server(Arc::new(state)).await?;
        let mut session = UserSession::new_admin();
        session.signup(&app).await?;

        session
            .login(&app, "admin@crown-shy.com", TEST_PASSWORD)
            .await?;

        let (_, user, _) = session.current_user(&app).await?;

        let created_media = create_random_image_record(&pool, &user.id).await?;

        let _ = session
            .delete(&app, &format!("/media/{}", created_media.id))
            .await?;

        let (_, value, _) = session
            .get(&app, &format!("/media/{}", created_media.id))
            .await?;

        assert_eq!(
            value.get("err").and_then(|v| v.as_str()).unwrap(),
            "Media not found",
            "incorrect error message"
        );

        Ok(())
    }
}
