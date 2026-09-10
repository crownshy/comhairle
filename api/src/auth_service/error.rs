use aide::OperationIo;
use thiserror::Error;

#[derive(Error, Debug, OperationIo)]
#[aide(output)]
pub enum AuthServiceError {
    #[error("Keycloak error: {0}")]
    KeycloakError(#[from] keycloak::KeycloakError),

    #[error("Access token failure: {0}")]
    AccessTokenFailure(String),

    #[error("Sync user error: {0}")]
    SyncUserError(String),

    #[error("Credential extraction failure: {0}")]
    CredentialExtractionFailure(String),
}

// TODO: status code implementation
