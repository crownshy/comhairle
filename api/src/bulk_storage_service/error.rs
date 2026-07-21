use thiserror::Error;

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

    #[error("Failed list: {0}")]
    FailedList(String),

    #[error("Failed to create multipart upload: {0}")]
    FailedToCreateMultipartUpload(String),

    #[error("Failed to complete multipart upload: {0}")]
    FailedToCompleteMultipartUpload(String),

    #[error("Failed to abort multipart upload: {0}")]
    FailedToAbortMultipartUpload(String),
}
