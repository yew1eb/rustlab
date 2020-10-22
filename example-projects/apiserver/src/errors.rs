use actix_web::{
    error::{
        BlockingError, JsonPayloadError, ParseError, PayloadError, ResponseError,
    },
    http::StatusCode,
    HttpResponse,
};

use serde_derive::Serialize;
use std::convert::From;

use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};

use derive_more::Display;
#[allow(dead_code)]
#[derive(Debug, Display, PartialEq)]
pub enum ServiceError {
    InternalServerError(String),

    DataBaseError(String),
    BlockingError(String),
    BadRequest(String),

    NotFound(String),
}

#[derive(Debug, Deserialize, Serialize)]
struct ErrorResponseModel {
    code: i32,
    message: String,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError(ref message) => {
                let err = ErrorResponseModel {
                    code: 500,
                    message: format!("Internal Server Error, {}", message),
                };
                HttpResponse::InternalServerError().json(err)
            }
            ServiceError::BadRequest(ref message) => {
                let err = ErrorResponseModel {
                    code: 400,
                    message: message.to_string(),
                };
                HttpResponse::BadRequest().json(err)
            }
            ServiceError::NotFound(ref message) => {
                let err = ErrorResponseModel {
                    code: 404,
                    message: message.to_string(),
                };
                HttpResponse::NotFound().json(err)
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<ParseError> for ServiceError {
    fn from(err: ParseError) -> ServiceError {
        ServiceError::BadRequest(format!("Bad Request: {}", err.to_string()).to_string())
    }
}

impl From<PayloadError> for ServiceError {
    fn from(err: PayloadError) -> ServiceError {
        ServiceError::BadRequest(err.to_string())
    }
}

impl From<JsonPayloadError> for ServiceError {
    fn from(err: JsonPayloadError) -> ServiceError {
        ServiceError::BadRequest(err.to_string())
    }
}

/// Convert DBErrors to ServiceErrors
impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(message);
                }
                ServiceError::InternalServerError("Unknown database error".into())
            }
            _ => ServiceError::InternalServerError("Unknown database error".into()),
        }
    }
}

/// Convert PoolErrors to ServiceErrors
impl From<PoolError> for ServiceError {
    fn from(error: PoolError) -> ServiceError {
        ServiceError::DataBaseError(error.to_string())
    }
}

/// Convert Thread BlockingErrors to ServiceErrors
impl From<BlockingError<ServiceError>> for ServiceError {
    fn from(error: BlockingError<ServiceError>) -> ServiceError {
        match error {
            BlockingError::Error(api_error) => api_error,
            BlockingError::Canceled => ServiceError::BlockingError("Thread blocking error".into()),
        }
    }
}
