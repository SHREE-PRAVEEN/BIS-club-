use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
    pub code: String,
}

#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Conflict(String),
    InternalServerError(String),
    FileTooLarge,
    InvalidFileType,
    MultipartError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::FileTooLarge => write!(f, "File size exceeds maximum allowed size"),
            AppError::InvalidFileType => write!(f, "Invalid file type"),
            AppError::MultipartError(msg) => write!(f, "Multipart error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, code) = match self {
            AppError::DatabaseError(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR",
            ),
            AppError::NotFound(_) => (
                actix_web::http::StatusCode::NOT_FOUND,
                "NOT_FOUND",
            ),
            AppError::BadRequest(_) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                "BAD_REQUEST",
            ),
            AppError::Unauthorized(_) => (
                actix_web::http::StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED",
            ),
            AppError::Conflict(_) => (
                actix_web::http::StatusCode::CONFLICT,
                "CONFLICT",
            ),
            AppError::InternalServerError(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR",
            ),
            AppError::FileTooLarge => (
                actix_web::http::StatusCode::PAYLOAD_TOO_LARGE,
                "FILE_TOO_LARGE",
            ),
            AppError::InvalidFileType => (
                actix_web::http::StatusCode::BAD_REQUEST,
                "INVALID_FILE_TYPE",
            ),
            AppError::MultipartError(_) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                "MULTIPART_ERROR",
            ),
        };

        let error_msg = self.to_string();
        
        HttpResponse::build(status).json(ErrorResponse {
            error: error_msg,
            details: None,
            code: code.to_string(),
        })
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        log::error!("Database error: {:?}", err);
        match err {
            sqlx::Error::RowNotFound => {
                AppError::NotFound("Resource not found".to_string())
            }
            sqlx::Error::Configuration(_) => {
                AppError::DatabaseError("Database configuration error".to_string())
            }
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}
