use fmt::Debug;
use mongodb::error::Error;

use core::fmt;

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use chrono::ParseError;
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum ApiErrorType {
    DbError,
    ValidationError,
    NotFoundError,
    ParseError,
    Rejected,
    InternalError,
}

#[derive(Debug)]
pub struct ApiError {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: ApiErrorType,
}

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
}

impl ApiError {
    pub fn create_validation_error(message: &str) -> Self {
        Self {
            cause: None,
            message: Some(message.to_string()),
            error_type: ApiErrorType::ValidationError,
        }
    }
    pub fn create(message: &str, error_type: ApiErrorType) -> Self {
        Self {
            cause: None,
            message: Some(message.to_string()),
            error_type,
        }
    }

    // we are handling the none. function name should match field name
    fn message(&self) -> String {
        match &*self {
            // Error message is found then clone otherwise default message
            ApiError {
                cause: _,
                message: Some(message),
                error_type: _,
            } => message.clone(),
            ApiError {
                cause: _,
                message: None,
                error_type: ApiErrorType::NotFoundError,
            } => "The requested item was not found".to_string(),
            ApiError {
                cause: Some(cause),
                message: None,
                error_type: ApiErrorType::ValidationError,
            } => cause.clone(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ApiError {
    //error_response and status_code are the provided methods for ResponseError Trait

    fn status_code(&self) -> StatusCode {
        match self.error_type {
            ApiErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorType::NotFoundError => StatusCode::NOT_FOUND,
            ApiErrorType::ValidationError => StatusCode::BAD_REQUEST,
            ApiErrorType::ParseError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorType::Rejected => StatusCode::NOT_ACCEPTABLE,
            ApiErrorType::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ApiErrorResponse {
            error: self.message(),
        })
    }
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError {
            message: None,
            cause: Some(error.to_string()),
            error_type: ApiErrorType::DbError,
        }
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(error: ValidationErrors) -> ApiError {
        ApiError {
            message: None,
            cause: Some(error.to_string()),
            error_type: ApiErrorType::ValidationError,
        }
    }
}

// Convert ParseErrors to ApiErrors
impl From<ParseError> for ApiError {
    fn from(error: ParseError) -> ApiError {
        ApiError {
            message: None,
            cause: Some(error.to_string()),
            error_type: ApiErrorType::ParseError,
        }
    }
}

impl From<url::ParseError> for ApiError {
    fn from(error: url::ParseError) -> Self {
        ApiError {
            message: None,
            cause: Some(error.to_string()),
            error_type: ApiErrorType::ParseError,
        }
    }
}

impl From<Error> for ApiErrorType {
    fn from(_error: Error) -> ApiErrorType {
        ApiErrorType::DbError
    }
}

impl fmt::Display for ApiErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ApiErrorType {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::{ApiError, ApiErrorType};
    use actix_web::error::ResponseError;

    #[test]
    fn test_default_db_error() {
        let db_error = ApiError {
            message: None,
            cause: None,
            error_type: ApiErrorType::DbError,
        };

        assert_eq!(
            db_error.message(),
            "An unexpected error has occurred".to_string(),
            "Default message should be shown"
        );
    }

    #[test]
    fn test_default_not_found_error() {
        let db_error = ApiError {
            message: None,
            cause: None,
            error_type: ApiErrorType::NotFoundError,
        };

        assert_eq!(
            db_error.message(),
            "The requested item was not found".to_string(),
            "Default message should be shown"
        );
    }

    #[test]
    fn test_user_db_error() {
        let user_message = "User-facing message".to_string();

        let db_error = ApiError {
            message: Some(user_message.clone()),
            cause: None,
            error_type: ApiErrorType::DbError,
        };

        assert_eq!(
            db_error.message(),
            user_message,
            "User-facing message should be shown"
        );
    }

    #[test]
    fn test_db_error_status_code() {
        let expected = 500;

        let db_error = ApiError {
            message: None,
            cause: None,
            error_type: ApiErrorType::DbError,
        };

        assert_eq!(
            db_error.status_code(),
            expected,
            "Status code for DbError should be {}",
            expected
        );
    }
}
