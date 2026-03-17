use async_trait::async_trait;
use thiserror::Error;
pub mod s3_storage;

#[cfg(test)]
use mockall::automock;

/// Result of a file upload operation.
pub struct UploadResult {
    /// The URL where the uploaded file can be accessed.
    pub url: String,
}

/// Metadata for a file upload operation.
#[derive(Debug, Clone)]
pub struct FileMetadata {
    /// MIME type of the file (e.g., "image/png", "application/pdf")
    pub content_type: String,
    /// Whether the file should be publicly readable
    pub is_public: bool,
}

impl FileMetadata {
    /// Creates a new FileMetadata instance.
    ///
    /// # Arguments
    ///
    /// * `content_type` - MIME type of the file
    /// * `is_public` - Whether the file should be publicly readable
    pub fn new(content_type: String, is_public: bool) -> Self {
        Self {
            content_type,
            is_public,
        }
    }
}

#[derive(Error, Debug)]
pub enum BulkStorageError {
    #[error("Failed to Upload file: {0}")]
    FailedToUpload(String),

    #[error("Failed to get presigned upload url: {0}")]
    FailedToGetUploadPresign(String),

    #[error("Failed to get presigned download url: {0}")]
    FailedToGetDownloadPresign(String),

    #[error("Failed to delete file: {0}")]
    FailedToDelete(String),

    #[error("Failed to get file: {0}")]
    FailedToGetFile(String),
}

/// Service for managing file storage operations.
///
/// This trait provides a unified interface for file storage operations including
/// uploading, downloading, deleting files, and generating presigned URLs for client-side operations.
#[async_trait]
#[cfg_attr(test, automock)]
pub trait BulkStorageService: Send + Sync {
    /// Uploads a file to the storage service.
    ///
    /// # Arguments
    ///
    /// * `path` - The target path where the file should be stored
    /// * `data` - The binary content of the file
    /// * `metadata` - File metadata including content type and access permissions
    ///
    /// # Returns
    ///
    /// Returns an `UploadResult` containing the URL where the file can be accessed.
    /// For public files, this will be a direct URL. For private files, this will be a presigned URL.
    async fn upload_file(
        &self,
        path: &str,
        data: Vec<u8>,
        metadata: FileMetadata,
    ) -> Result<UploadResult, BulkStorageError>;

    /// Retrieves a file from the storage service as binary data.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the file to retrieve
    ///
    /// # Returns
    ///
    /// Returns the file contents as a byte vector.
    async fn get_file(&self, path: &str) -> Result<Vec<u8>, BulkStorageError>;

    /// Deletes a file from the storage service.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the file to delete
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the file was successfully deleted.
    async fn delete_file(&self, path: &str) -> Result<(), BulkStorageError>;

    /// Generates a presigned URL for uploading a file.
    ///
    /// This allows clients to upload files directly to storage without going through the server.
    ///
    /// # Arguments
    ///
    /// * `path` - The target path where the file will be stored
    ///
    /// # Returns
    ///
    /// Returns a presigned URL that can be used to upload a file via HTTP PUT.
    async fn get_write_file_url(&self, path: &str) -> Result<String, BulkStorageError>;

    /// Generates a presigned URL for downloading a file.
    ///
    /// This allows clients to download files directly from storage without going through the server.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the file to download
    ///
    /// # Returns
    ///
    /// Returns a presigned URL that can be used to download the file via HTTP GET.
    async fn get_read_file_url(&self, path: &str) -> Result<String, BulkStorageError>;
}

#[cfg(test)]
impl MockBulkStorageService {
    pub fn base() -> MockBulkStorageService {
        let mut storage = MockBulkStorageService::new();

        storage.expect_upload_file().returning(|_, _, _| {
            Box::pin(async move {
                Ok(UploadResult {
                    url: "https://storage.com/some_file".to_owned(),
                })
            })
        });

        storage.expect_get_file().returning(|_| {
            Box::pin(async move { Ok(vec![0u8; 100]) })
        });

        storage.expect_delete_file().returning(|_| {
            Box::pin(async move { Ok(()) })
        });

        storage.expect_get_write_file_url().returning(|_| {
            Box::pin(async move { Ok("https://storage.com/signed_upload_path".to_owned()) })
        });

        storage.expect_get_read_file_url().returning(|_| {
            Box::pin(async move { Ok("https://storage.com/signed_dowload_path".to_owned()) })
        });

        storage
    }
}
