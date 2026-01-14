use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
    pub code: String,
}

#[derive(Debug)]
pub enum AppError {
    // PERSONNALISER : Ajouter d'autres variantes d'erreur selon vos besoins
    DatabaseError(String),
    EmailError(String),
    ValidationError(String),
    NotFound(String),
    InternalError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DatabaseError(msg) => write!(f, "Erreur base de données: {}", msg),
            Self::EmailError(msg) => write!(f, "Erreur email: {}", msg),
            Self::ValidationError(msg) => write!(f, "Erreur validation: {}", msg),
            Self::NotFound(msg) => write!(f, "Non trouvé: {}", msg),
            Self::InternalError(msg) => write!(f, "Erreur interne: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, code) = match self {
            Self::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR"),
            Self::EmailError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "EMAIL_ERROR"),
            Self::ValidationError(_) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
            Self::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            Self::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
        };

        HttpResponse::build(status).json(ApiError {
            message: self.to_string(),
            code: code.to_string(),
        })
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EmailError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        log::error!("Erreur SQLx: {}", err);
        Self::DatabaseError(err.to_string())
    }
}
