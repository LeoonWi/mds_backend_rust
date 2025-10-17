use axum::{Json, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub id: Option<i64>,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Service {
    pub fn new(id: Option<i64>, name: Option<String>) -> Self {
        Service {
            id: id,
            name: name.unwrap_or_else(|| "".to_string()),
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub timestamp: DateTime<Utc>,
}

impl ErrorResponse {
    pub fn new(msg: String) -> Self {
        ErrorResponse {
            error: msg,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Conflict(String),
    BadRequest(String),
    NotFound(String),
    InternalServerError(String),
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Conflict(_) => StatusCode::CONFLICT,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn into_response(self) -> (StatusCode, Json<ErrorResponse>) {
        let message = match &self {
            Error::Conflict(msg) => msg,
            Error::BadRequest(msg) => msg,
            Error::NotFound(msg) => msg,
            Error::InternalServerError(msg) => msg,
        };
        (
            self.status_code(),
            Json(ErrorResponse::new(message.to_string())),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Employee {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
    pub services: Option<Vec<Service>>,
    pub active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
