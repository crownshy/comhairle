use async_trait::async_trait;
use aws_config::SdkConfig;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::{RequestChecksumCalculation, ResponseChecksumValidation};
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::ObjectCannedAcl;
use std::time::Duration;

use crate::bulk_storage_service::{
    BulkStorageService, FileMetadata, MultipartUploadPartNumber, StorageEntityTag, StorageUploadID,
    error::BulkStorageError,
};
/// Presigned URL expiration time for PUT operations (in seconds)
const PUT_EXPIRES: u64 = 600;
/// Presigned URL expiration time for GET operations (in seconds)
const GET_EXPIRES: u64 = 600;

/// AWS S3 implementation of the BulkStorageService.
///
/// This service provides file storage operations using Amazon S3 as the backend.
/// It supports uploading, downloading, deleting files, and generating presigned URLs
/// for direct client access.
pub struct S3StorageService {
    /// The S3 client for making API calls
    pub s3_client: Client,
    /// The name of the S3 bucket to use for storage
    pub bucket: String,
}

impl S3StorageService {
    /// Creates a new S3StorageService instance.
    ///
    /// # Arguments
    ///
    /// * `config` - AWS SDK configuration
    /// * `bucket` - The name of the S3 bucket to use for file storage
    pub fn new(config: &SdkConfig, bucket: String) -> Self {
        // aws-sdk-s3 enables "flexible checksums" by default, which signs
        // x-amz-sdk-checksum-algorithm (and a body checksum header) into the
        // presigned URL's SignedHeaders. Browsers don't send these headers
        // when doing a plain PUT of a File, causing SignatureDoesNotMatch.
        let s3_config = aws_sdk_s3::config::Builder::from(config)
            .request_checksum_calculation(RequestChecksumCalculation::WhenRequired)
            .response_checksum_validation(ResponseChecksumValidation::WhenRequired)
            .build();
        let client = Client::from_conf(s3_config);
        Self {
            s3_client: client,
            bucket,
        }
    }
}

#[async_trait]
impl BulkStorageService for S3StorageService {
    /// Uploads a file to S3.
    ///
    /// If the file is marked as public, it will be uploaded with the `public-read` ACL
    /// and a direct S3 URL will be returned. Otherwise, a presigned URL will be generated.
    async fn upload_file(
        &self,
        path: &str,
        data: Vec<u8>,
        metadata: FileMetadata,
    ) -> Result<super::UploadResult, super::BulkStorageError> {
        let body = ByteStream::from(data);

        let mut put_object = self
            .s3_client
            .put_object()
            .bucket(&self.bucket)
            .key(path)
            .content_type(&metadata.content_type)
            .body(body);

        if metadata.is_public {
            put_object = put_object.acl(ObjectCannedAcl::PublicRead);
        }

        put_object
            .send()
            .await
            .inspect_err(|e| {
                eprintln!("S3 upload error: {e:#?}");
            })
            .map_err(|e| BulkStorageError::FailedToUpload(e.to_string()))?;

        let url = if metadata.is_public {
            format!("https://{}.s3.amazonaws.com/{}", self.bucket, path)
        } else {
            self.get_read_file_url(path).await?
        };

        Ok(super::UploadResult { url })
    }

    /// Downloads a file from S3 and returns its contents as bytes.
    async fn get_file(&self, path: &str) -> Result<Vec<u8>, super::BulkStorageError> {
        let result = self
            .s3_client
            .get_object()
            .bucket(&self.bucket)
            .key(path)
            .send()
            .await
            .map_err(|e| BulkStorageError::FailedToGetFile(e.to_string()))?;

        let data = result
            .body
            .collect()
            .await
            .map_err(|e| BulkStorageError::FailedToGetFile(e.to_string()))?;

        Ok(data.into_bytes().to_vec())
    }

    /// Deletes a file from S3.
    async fn delete_file(&self, path: &str) -> Result<(), super::BulkStorageError> {
        self.s3_client
            .delete_object()
            .bucket(&self.bucket)
            .key(path)
            .send()
            .await
            .map_err(|e| BulkStorageError::FailedToDelete(e.to_string()))?;

        Ok(())
    }

    /// Generates a presigned URL for uploading a file directly to S3.
    ///
    /// The URL will be valid for 10 minutes (600 seconds).
    async fn get_write_file_url(
        &self,
        target_dest: &str,
    ) -> Result<String, super::BulkStorageError> {
        let expires_in: Duration = Duration::from_secs(PUT_EXPIRES);

        let expires_in: PresigningConfig = PresigningConfig::expires_in(expires_in).unwrap();

        let presigned_request = self
            .s3_client
            .put_object()
            .bucket(&self.bucket)
            .key(target_dest)
            .presigned(expires_in)
            .await
            .map_err(|e| BulkStorageError::FailedToGetUploadPresign(e.to_string()))?;

        Ok(presigned_request.uri().into())
    }

    /// Generates a presigned URL for downloading a file directly from S3.
    ///
    /// The URL will be valid for 10 minutes (600 seconds).
    async fn get_read_file_url(&self, path: &str) -> Result<String, super::BulkStorageError> {
        let expires_in = Duration::from_secs(GET_EXPIRES);
        let url = self
            .s3_client
            .get_object()
            .bucket(&self.bucket)
            .key(path)
            .presigned(PresigningConfig::expires_in(expires_in).unwrap())
            .await
            .map_err(|e| BulkStorageError::FailedToGetDownloadPresign(e.to_string()))?;

        Ok(url.uri().into())
    }

    /// Returns a list of objects within an S3 bucket with an optional path prefix.
    async fn list_keys(
        &self,
        store: &str,
        prefix: Option<&str>,
    ) -> Result<Vec<String>, BulkStorageError> {
        let prefix = prefix.unwrap_or("");

        let result = self
            .s3_client
            .list_objects_v2()
            .bucket(store)
            .prefix(prefix)
            .send()
            .await
            .map_err(|e| BulkStorageError::FailedList(e.to_string()))?;

        // Normalise entries by removing prefix to allow consistent room_id extraction
        // across providers
        let entries = result
            .contents()
            .iter()
            .filter_map(|entry| entry.key())
            .map(|key| key.strip_prefix(prefix).unwrap_or(key).to_string())
            .collect();

        Ok(entries)
    }

    async fn create_multipart_upload(
        &self,
        path: &str,
        metadata: FileMetadata,
    ) -> Result<StorageUploadID, BulkStorageError> {
        let mut put_object = self
            .s3_client
            .create_multipart_upload()
            .bucket(&self.bucket)
            .key(path)
            .content_type(&metadata.content_type);

        if metadata.is_public {
            put_object = put_object.acl(ObjectCannedAcl::PublicRead);
        }

        let result = put_object
            .send()
            .await
            .map_err(|e| BulkStorageError::FailedToCreateMultipartUpload(e.to_string()))?;

        let upload_id = result
            .upload_id()
            .ok_or_else(|| {
                BulkStorageError::FailedToCreateMultipartUpload("Missing upload ID".to_string())
            })?
            .to_string();

        Ok(StorageUploadID(upload_id))
    }

    async fn get_multipart_file_write_url(
        &self,
        path: &str,
        upload_id: &StorageUploadID,
        part_number: MultipartUploadPartNumber,
    ) -> Result<String, BulkStorageError> {
        let expires_in: Duration = Duration::from_secs(PUT_EXPIRES);

        let presigned = self
            .s3_client
            .upload_part()
            .bucket(&self.bucket)
            .upload_id(&upload_id.0)
            .key(path)
            .part_number(part_number.0)
            .presigned(
                PresigningConfig::expires_in(expires_in)
                    .map_err(|e| BulkStorageError::FailedToGetUploadPresign(e.to_string()))?,
            )
            .await
            .map_err(|e| BulkStorageError::FailedToGetUploadPresign(e.to_string()))?;

        Ok(presigned.uri().to_string())
    }

    async fn complete_multipart_upload(
        &self,
        path: &str,
        upload_id: &StorageUploadID,
        parts: &[(MultipartUploadPartNumber, StorageEntityTag)],
    ) -> Result<(), BulkStorageError> {
        let completed_parts = parts
            .iter()
            .map(|(part_number, etag)| {
                aws_sdk_s3::types::CompletedPart::builder()
                    .set_part_number(Some(part_number.0))
                    .set_e_tag(Some(etag.0.clone()))
                    .build()
            })
            .collect::<Vec<_>>();

        let completed_upload = aws_sdk_s3::types::CompletedMultipartUpload::builder()
            .set_parts(Some(completed_parts))
            .build();

        self.s3_client
            .complete_multipart_upload()
            .bucket(&self.bucket)
            .upload_id(&upload_id.0)
            .key(path)
            .multipart_upload(completed_upload)
            .send()
            .await
            .map_err(|e| BulkStorageError::FailedToCompleteMultipartUpload(e.to_string()))?;

        Ok(())
    }

    async fn abort_multipart_upload(
        &self,
        path: &str,
        upload_id: &StorageUploadID,
    ) -> Result<(), BulkStorageError> {
        self.s3_client
            .abort_multipart_upload()
            .bucket(&self.bucket)
            .upload_id(&upload_id.0)
            .key(path)
            .send()
            .await
            .map_err(|e| BulkStorageError::FailedToAbortMultipartUpload(e.to_string()))?;

        Ok(())
    }
}
