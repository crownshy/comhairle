//! Presigned URLs for stored resources.
//!
//! These talk to S3, so they sit above the model layer. The `Resource` row and
//! its queries stay in [`crate::models::resource`].

use std::time::Duration;

use aws_sdk_s3::Client;
use aws_sdk_s3::presigning::PresigningConfig;
use tracing::instrument;

use crate::error::{ComhairleError, TransportError};
use crate::models::resource::{Resource, ResourceResponse, ResourceSource};

const PUT_EXPIRES: u64 = 600;
const GET_EXPIRES: u64 = 600;

/// S3-backed operations on a [`Resource`]; an extension trait because the row
/// type lives in the model crate.
#[async_trait::async_trait]
pub trait ResourceExt {
    async fn to_resource_response(
        &self,
        client: &Client,
        bucket: &str,
    ) -> Result<ResourceResponse, ComhairleError>;

    async fn resolve_url(&self, client: &Client, bucket: &str) -> Result<String, ComhairleError>;
}

#[async_trait::async_trait]
impl ResourceExt for Resource {
    async fn to_resource_response(
        &self,
        client: &Client,
        bucket: &str,
    ) -> Result<ResourceResponse, ComhairleError> {
        let url = self.resolve_url(client, bucket).await?;
        Ok(ResourceResponse {
            id: self.id,
            url,
            media_type: self.media_type,
            owner_id: self.owner_id,
        })
    }

    async fn resolve_url(&self, client: &Client, bucket: &str) -> Result<String, ComhairleError> {
        match self.storage_type {
            ResourceSource::S3 => get_presigned_url(&self.url, client, bucket).await,
            ResourceSource::Url => Ok(self.url.clone()),
        }
    }
}

#[instrument(err(Debug), skip(client))]
pub async fn get_presigned_url(
    target: &str,
    client: &Client,
    bucket: &str,
) -> Result<String, ComhairleError> {
    let expires_in = Duration::from_secs(GET_EXPIRES);
    let url = client
        .get_object()
        .bucket(bucket)
        .key(target)
        .presigned(PresigningConfig::expires_in(expires_in).unwrap())
        .await
        .map_err(|e| TransportError::FailedToGetDownloadPresign(e.to_string()))?;

    Ok(url.uri().into())
}

#[instrument(err(Debug), skip(client))]
pub async fn get_signed_upload_url(
    target_dest: &str,
    client: &Client,
    bucket: &str,
) -> Result<String, ComhairleError> {
    let expires_in: Duration = std::time::Duration::from_secs(PUT_EXPIRES);

    let expires_in: aws_sdk_s3::presigning::PresigningConfig =
        PresigningConfig::expires_in(expires_in).unwrap();

    let presigned_request = client
        .put_object()
        .bucket(bucket)
        .key(target_dest)
        .presigned(expires_in)
        .await
        .map_err(|e| TransportError::FailedToGetUploadPresign(e.to_string()))?;

    Ok(presigned_request.uri().into())
}
