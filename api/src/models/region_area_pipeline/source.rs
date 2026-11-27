use std::net::SocketAddr;
use std::time::Duration;

use reqwest::Url;
use reqwest::header::LOCATION;
use reqwest::redirect::Policy;
use uuid::Uuid;

use super::PipelineSource;
use crate::bulk_storage_service::{BulkStorageService, FileMetadata};
use crate::error::ComhairleError;

type Result<T> = std::result::Result<T, ComhairleError>;

// Don't lock out the server if someone points it at a bot hole.
const MAX_REDIRECTS: usize = 10;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

fn upload_key(upload_id: Uuid) -> String {
    format!("region-area-pipelines/uploads/{upload_id}.geojson")
}

pub async fn store_upload(storage: &dyn BulkStorageService, bytes: Vec<u8>) -> Result<Uuid> {
    if bytes.is_empty() {
        return Err(ComhairleError::BadRequest(
            "The uploaded pipeline source is empty".to_string(),
        ));
    }
    let upload_id = Uuid::new_v4();
    tokio::time::timeout(
        REQUEST_TIMEOUT,
        storage.upload_file(
            &upload_key(upload_id),
            bytes,
            FileMetadata::new("application/geo+json".to_string(), false),
        ),
    )
    .await
    .map_err(|_| {
        ComhairleError::BackgroundJobFailed("Pipeline source upload timed out".to_string())
    })??;
    Ok(upload_id)
}

pub async fn load_source(
    source: &PipelineSource,
    storage: Option<&dyn BulkStorageService>,
) -> Result<Vec<u8>> {
    match source {
        PipelineSource::Https { url } => tokio::time::timeout(REQUEST_TIMEOUT, load_https(url))
            .await
            .map_err(|_| {
                ComhairleError::BackgroundJobFailed("Pipeline source request timed out".to_string())
            })?,
        PipelineSource::UploadedFile { upload_id } => {
            let storage = storage.ok_or(ComhairleError::NoBulkStorageServiceConfigured)?;
            let bytes =
                tokio::time::timeout(REQUEST_TIMEOUT, storage.get_file(&upload_key(*upload_id)))
                    .await
                    .map_err(|_| {
                        ComhairleError::BackgroundJobFailed(
                            "Pipeline source download timed out".to_string(),
                        )
                    })??;
            Ok(bytes)
        }
    }
}

async fn load_https(source_url: &str) -> Result<Vec<u8>> {
    let mut url = Url::parse(source_url).map_err(|error| {
        ComhairleError::BadRequest(format!("Invalid pipeline source URL: {error}"))
    })?;

    for redirects in 0..=MAX_REDIRECTS {
        match PublicHttpsSource::resolve(url).await?.fetch().await? {
            FetchOutcome::Complete(bytes) => return Ok(bytes),
            FetchOutcome::Redirect(next) if redirects < MAX_REDIRECTS => url = next,
            FetchOutcome::Redirect(_) => {
                return Err(ComhairleError::BadRequest(
                    "Pipeline source exceeded the redirect limit".into(),
                ));
            }
        }
    }
    unreachable!("redirect loop always returns")
}

/// Only validated public endpoints can issue a request.
struct PublicHttpsSource {
    url: Url,
    client: reqwest::Client,
}

enum FetchOutcome {
    Redirect(Url),
    Complete(Vec<u8>),
}

impl PublicHttpsSource {
    async fn resolve(url: Url) -> Result<Self> {
        if url.scheme() != "https"
            || !url.username().is_empty()
            || url.password().is_some()
            || url.fragment().is_some()
        {
            return Err(ComhairleError::BadRequest(
                "Pipeline source must be an HTTPS URL without credentials or a fragment"
                    .to_string(),
            ));
        }
        let host = url.host_str().ok_or_else(|| {
            ComhairleError::BadRequest("Pipeline source URL has no host".to_string())
        })?;
        let port = url.port_or_known_default().ok_or_else(|| {
            ComhairleError::BadRequest("Pipeline source URL has no port".to_string())
        })?;
        let addresses = resolve_public_addresses(host, port).await?;
        let client = reqwest::Client::builder()
            .no_proxy()
            .redirect(Policy::none())
            .timeout(REQUEST_TIMEOUT)
            .resolve_to_addrs(host, &addresses)
            .build()
            .map_err(|error| {
                ComhairleError::BackgroundJobFailed(format!(
                    "Could not configure pipeline source client: {error}"
                ))
            })?;
        Ok(Self { url, client })
    }

    async fn fetch(self) -> Result<FetchOutcome> {
        let mut response = self
            .client
            .get(self.url.clone())
            .send()
            .await
            .map_err(|error| {
                ComhairleError::BackgroundJobFailed(format!(
                    "Could not fetch pipeline source: {}",
                    error.without_url()
                ))
            })?;

        if response.status().is_redirection() {
            let location = response
                .headers()
                .get(LOCATION)
                .ok_or_else(|| {
                    ComhairleError::BadRequest(
                        "Pipeline source redirect has no location".to_string(),
                    )
                })?
                .to_str()
                .map_err(|_| {
                    ComhairleError::BadRequest(
                        "Pipeline source redirect has an invalid location".to_string(),
                    )
                })?;
            let next = self.url.join(location).map_err(|_| {
                ComhairleError::BadRequest(
                    "Pipeline source redirect has an invalid URL".to_string(),
                )
            })?;
            return Ok(FetchOutcome::Redirect(next));
        }
        if !response.status().is_success() {
            return Err(ComhairleError::BackgroundJobFailed(format!(
                "Pipeline source returned HTTP {}",
                response.status()
            )));
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|error| {
            ComhairleError::BackgroundJobFailed(format!(
                "Could not read pipeline source response: {}",
                error.without_url()
            ))
        })? {
            bytes.extend_from_slice(&chunk);
        }
        Ok(FetchOutcome::Complete(bytes))
    }
}

async fn resolve_public_addresses(host: &str, port: u16) -> Result<Vec<SocketAddr>> {
    let addresses: Vec<_> = tokio::time::timeout(
        REQUEST_TIMEOUT,
        tokio::net::lookup_host((host.trim_matches(['[', ']']), port)),
    )
    .await
    .map_err(|_| ComhairleError::BackgroundJobFailed("Pipeline source DNS timed out".to_string()))?
    .map_err(|error| {
        ComhairleError::BackgroundJobFailed(format!("Could not resolve pipeline source: {error}"))
    })?
    .collect();
    Ok(addresses)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bulk_storage_service::{MockBulkStorageService, UploadResult};

    #[tokio::test]
    async fn rejects_unsafe_remote_sources() {
        for url in [
            "http://example.com/data",
            "https://user@example.com/data",
            "https://127.0.0.1/data",
            "https://[::1]/data",
        ] {
            assert!(load_https(url).await.is_err(), "{url}");
        }
    }

    #[tokio::test]
    async fn uploads_and_loads_private_storage_objects() {
        let captured_key = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
        let upload_key_capture = captured_key.clone();
        let mut storage = MockBulkStorageService::new();
        storage
            .expect_upload_file()
            .once()
            .returning(move |key, bytes, metadata| {
                *upload_key_capture.lock().unwrap() = key.to_owned();
                assert_eq!(bytes, b"geojson");
                assert!(!metadata.is_public);
                assert_eq!(metadata.content_type, "application/geo+json");
                Box::pin(async { Ok(UploadResult { url: String::new() }) })
            });
        let upload_id = store_upload(&storage, b"geojson".to_vec()).await.unwrap();
        assert_eq!(*captured_key.lock().unwrap(), upload_key(upload_id));

        let expected_key = upload_key(upload_id);
        storage
            .expect_get_file()
            .once()
            .withf(move |key| key == expected_key)
            .returning(|_| Box::pin(async { Ok(b"geojson".to_vec()) }));
        let source = PipelineSource::UploadedFile { upload_id };
        assert_eq!(
            load_source(&source, Some(&storage)).await.unwrap(),
            b"geojson"
        );
        assert!(matches!(
            load_source(&source, None).await,
            Err(ComhairleError::NoBulkStorageServiceConfigured)
        ));
    }
}
