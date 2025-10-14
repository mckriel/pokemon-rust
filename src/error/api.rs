use thiserror::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;


#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Pokemon not found: {name}")]
    PokemonNotFound { name: String },

    #[error("External API error: {0}")]
    ExternalApi(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Internal server error: {message}")]
    Internal { message: String },

    #[error("Bad request: {message}")]
    BadRequest { message: String },
}

impl ApiError {
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal {
            message: message.into()
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest {
            message: message.into()
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError::PokemonNotFound { name } => (
                StatusCode::NOT_FOUND,
                format!("Pokemon {} not found", name)
            ),
            ApiError::BadRequest { message } => (
                StatusCode::BAD_REQUEST,
                message,
            ),
            ApiError::ExternalApi(_) | ApiError::Json(_) | ApiError::Internal { .. } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(json! ({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
