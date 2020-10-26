use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
pub use config::ConfigError as ConfError;
use deadpool_postgres::config::ConfigError;
use deadpool_postgres::PoolError;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    InternalServerError(String),
    NotFound(String),
    PoolError(String),
    ConfigError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::NotFound(message) => {
                HttpResponse::NotFound().json::<ErrorResponse>(message.into())
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

impl From<PoolError> for Error {
    fn from(error: PoolError) -> Self {
        Error::PoolError(error.to_string())
    }
}

impl From<ConfigError> for Error {
    fn from(error: ConfigError) -> Self {
        Error::ConfigError(error.to_string())
    }
}

impl From<ConfError> for Error {
    fn from(error: ConfError) -> Self {
        Error::ConfigError(error.to_string())
    }
}
