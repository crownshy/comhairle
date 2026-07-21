pub mod config;
pub mod error;
pub mod s3_storage;

use async_trait::async_trait;

use error::BulkStorageError;

#[cfg(test)]
use mockall::automock;

/// Result of a file upload operation.
#[derive(Debug)]
pub struct UploadResult {
    /// The URL where the uploaded file can be accessed.
    pub url: String,
}

/// A wrapper around a storage service upload ID.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct StorageUploadID(pub String);

/// A wrapper type around a storage service entity tag.
#[derive(Debug)]
pub struct StorageEntityTag(pub String);

/// A wrapper type around a storage service part number for a multipart upload.
#[derive(Debug, Copy, Clone)]
pub struct MultipartUploadPartNumber(pub i32);

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

    /// Lists object keys found under the given prefix.
    ///
    /// Analogous to listing a directory: when called with a delimiter of `/`.
    ///
    /// # Arguments
    ///
    /// * `store` - Store name to list under, e.g. `"media"`
    /// * `prefix` - Optional key prefix within the store to list under, e.g. `"events/abc123/"`
    ///
    /// # Returns
    ///
    /// A vector of Strings representing the entries found under the prefix.
    ///
    /// ## Example
    /// ```bash
    /// [
    ///   "recording.wav",
    ///   "rooms/abc123/recording.wav",
    ///   "rooms/abc234/recording.wav",
    ///   "rooms/abc345/recording.wav"
    /// ]
    /// ```
    async fn list_keys(
        &self,
        store: &str,
        prefix: Option<&str>,
    ) -> Result<Vec<String>, BulkStorageError>;

    /// Initiates a multipart upload for a large file and returns the storage service's upload ID.
    ///
    /// This allows clients to upload large files in multiple parts without going through the server.
    ///
    /// # Arguments
    ///
    /// * `path` - The target path where the file will be stored.
    /// * `metadata` - Metadata for the file being uploaded.
    ///
    /// # Returns
    ///
    /// The upload ID that can be used to upload parts and complete the multipart upload.
    async fn create_multipart_upload(
        &self,
        _path: &str,
        _metadata: FileMetadata,
    ) -> Result<StorageUploadID, BulkStorageError>;

    /// Generates a presigned URL to upload a specific part of a multipart upload.
    ///
    /// # Arguments
    ///
    /// * `upload_id` - The ID of the multipart upload.
    /// * `part_number` - The part number (1-based index) of the part
    ///
    /// # Returns
    ///
    /// A presigned URL that can be used to upload the specified part.
    async fn get_multipart_file_write_url(
        &self,
        _upload_id: &StorageUploadID,
        _part_number: MultipartUploadPartNumber,
    ) -> Result<String, BulkStorageError>;

    /// Completes a multipart upload by providing the list of uploaded parts.
    ///
    /// This finalizes the upload and makes the file available at the specified path.
    ///
    /// # Arguments
    ///
    /// * `upload_id` - The ID of the multipart upload.
    /// * `parts` - A vector of `MultipartUploadPart` containing the part numbers and their corresponding tags.
    async fn complete_multipart_upload(
        &self,
        _upload_id: &StorageUploadID,
        _parts: &[(MultipartUploadPartNumber, StorageEntityTag)],
    ) -> Result<(), BulkStorageError>;

    /// Aborts a multipart upload, discarding any uploaded parts.
    ///
    /// # Arguments
    ///
    /// * `upload_id` - The ID of the multipart upload to abort.
    async fn abort_multipart_upload(
        &self,
        _upload_id: &StorageUploadID,
    ) -> Result<(), BulkStorageError>;
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

        storage
            .expect_get_file()
            .returning(|_| Box::pin(async move { Ok(vec![0u8; 100]) }));

        storage
            .expect_delete_file()
            .returning(|_| Box::pin(async move { Ok(()) }));

        storage.expect_get_write_file_url().returning(|_| {
            Box::pin(async move { Ok("https://storage.com/signed_upload_path".to_owned()) })
        });

        storage.expect_get_read_file_url().returning(|_| {
            Box::pin(async move { Ok("https://storage.com/signed_dowload_path".to_owned()) })
        });

        storage
            .expect_list_keys()
            .returning(|_, _| Box::pin(async move { Ok(vec!["recording.wav".to_string()]) }));

        storage.expect_create_multipart_upload().returning(|_, _| {
            Box::pin(async move { Ok(StorageUploadID("mock-upload-id".to_string())) })
        });

        storage
            .expect_get_multipart_file_write_url()
            .returning(|_, _| {
                Box::pin(async move {
                    Ok("https://storage.com/signed_multipart_upload_path".to_owned())
                })
            });

        storage
            .expect_complete_multipart_upload()
            .returning(|_, _| Box::pin(async move { Ok(()) }));

        storage
            .expect_abort_multipart_upload()
            .returning(|_| Box::pin(async move { Ok(()) }));

        storage
    }
}

pub fn extract_room_id_from_key(key: &str) -> Option<&str> {
    match key.split("/").collect::<Vec<_>>().as_slice() {
        ["rooms", room_id, ..] => Some(room_id),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_room_id_from_key() {
        let key = "rooms/abc123/recording.wav";

        let result = extract_room_id_from_key(key).unwrap();

        assert_eq!(result, "abc123", "incorrect room_id");
    }
}
