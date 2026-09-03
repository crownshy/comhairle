use aide::OperationIo;
use thiserror::Error;

#[derive(Error, Debug, OperationIo)]
#[aide(output)]
pub enum AuthServiceError {
    #[error("Keycloak error: {0}")]
    KeycloakError(#[from] keycloak::KeycloakError),
}

// TODO: status code implementation
