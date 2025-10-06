use thiserror::Error;

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
