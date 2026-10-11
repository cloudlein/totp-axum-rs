use std::collections::HashMap;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use sqlx::Error;
use validator::ValidationErrors;
#[derive(Serialize)]
pub struct ErrorResponse<T = ()> {
    pub success: bool,
    pub  code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<T>
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,

    #[error("Bad Request")]
    BadRequest,

    #[error("Validation error")]
    Validation(ValidationErrors),

    #[error("Internal error")]
    InternalServerError
}

fn format_validation_error(err: ValidationErrors) -> HashMap<String, Vec<String>> {
    let mut errors = HashMap::new();
    for (field, field_errors) in err.into_errors() {
        if let validator::ValidationErrorsKind::Field(validation_errors) = field_errors {
            let mut messages = Vec::new();
            for e in validation_errors {
                if let Some(msg) = e.message{
                    messages.push(msg.into_owned());
                }else {
                    messages.push(e.code.into_owned())
                }
            }
            errors.insert(field.to_string(), messages);
        }
    }
    errors
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let message = self.to_string();

        let (status, code, errors) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND".to_string(), None),
            AppError::BadRequest => (StatusCode::BAD_REQUEST, "BAD_REQUEST".to_string(), None),
            AppError::Validation(err) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR".to_string(), Some(format_validation_error(err)) ),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR".to_string(), None)
        };

        let body = Json(ErrorResponse {
            success: false,
            code,
            message,
            errors
        });

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(inner: sqlx::Error) -> Self {
        tracing::error!("Database query failed: {:?}", inner);
        match inner {
            sqlx::Error::RowNotFound => AppError::NotFound,
            _ => AppError::InternalServerError,
        }
    }
}