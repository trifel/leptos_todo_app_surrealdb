use http::status::StatusCode;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum TodoAppError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("SurrealDBError({0})")]
    SurrealDBError(String),
}

impl TodoAppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            TodoAppError::NotFound => StatusCode::NOT_FOUND,
            TodoAppError::InternalServerError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            TodoAppError::SurrealDBError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
