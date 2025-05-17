use actix_web::{error::Error as ActixError, http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Not found: {0}")]
    NotFoundError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
    
    #[error("Unauthorized: {0}")]
    UnauthorizedError(String),
    
    #[error("Forbidden: {0}")]
    ForbiddenError(String),
    
    #[error("Bad request: {0}")]
    BadRequestError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    message: String,
    error_code: Option<String>,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        
        let error_response = ErrorResponse {
            success: false,
            message: self.to_string(),
            error_code: Some(status_code.as_str().to_string()),
        };
        
        HttpResponse::build(status_code).json(error_response)
    }
    
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFoundError(_) => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UnauthorizedError(_) => StatusCode::UNAUTHORIZED,
            AppError::ForbiddenError(_) => StatusCode::FORBIDDEN,
            AppError::BadRequestError(_) => StatusCode::BAD_REQUEST,
        }
    }
}

// Utility functions for creating specific errors
impl AppError {
    pub fn not_found(message: impl Into<String>) -> Self {
        AppError::NotFoundError(message.into())
    }
    
    pub fn validation_error(message: impl Into<String>) -> Self {
        AppError::ValidationError(message.into())
    }
    
    pub fn internal_error(message: impl Into<String>) -> Self {
        AppError::InternalError(message.into())
    }
    
    pub fn unauthorized(message: impl Into<String>) -> Self {
        AppError::UnauthorizedError(message.into())
    }
    
    pub fn forbidden(message: impl Into<String>) -> Self {
        AppError::ForbiddenError(message.into())
    }
    
    pub fn bad_request(message: impl Into<String>) -> Self {
        AppError::BadRequestError(message.into())
    }
}

// Type alias for Result with AppError
pub type AppResult<T> = Result<T, AppError>;

// Implement From<ActixError> for AppError
impl From<ActixError> for AppError {
    fn from(error: ActixError) -> Self {
        AppError::InternalError(error.to_string())
    }
}
