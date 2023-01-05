use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde::Serialize;

pub type Result<T, E = ApiError> = core::result::Result<T, E>;

#[derive(Serialize, Debug)]
pub struct ApiError {
    status_code: u16,
    error: String,
}

impl From<std::time::SystemTimeError> for ApiError {
    fn from(value: std::time::SystemTimeError) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: value.to_string(),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: value.to_string(),
        }
    }
}

impl From<bb8_postgres::tokio_postgres::Error> for ApiError {
    fn from(value: bb8_postgres::tokio_postgres::Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: value.to_string(),
        }
    }
}

impl From<bb8_redis::redis::RedisError> for ApiError {
    fn from(value: bb8_redis::redis::RedisError) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: value.to_string(),
        }
    }
}

impl From<bb8_postgres::bb8::RunError<bb8_postgres::tokio_postgres::Error>> for ApiError {
    fn from(value: bb8_postgres::bb8::RunError<bb8_postgres::tokio_postgres::Error>) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: value.to_string(),
        }
    }
}

impl From<bb8_redis::bb8::RunError<bb8_redis::redis::RedisError>> for ApiError {
    fn from(value: bb8_redis::bb8::RunError<bb8_redis::redis::RedisError>) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: value.to_string(),
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: value.to_string(),
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: value.to_string(),
        }
    }
}

impl From<axum::extract::multipart::MultipartError> for ApiError {
    fn from(value: axum::extract::multipart::MultipartError) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: value.to_string(),
        }
    }
}

impl From<()> for ApiError {
    fn from(_: ()) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: "None".to_string(),
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(value: serde_json::Error) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: value.to_string(),
        }
    }
}

impl ApiError {
    pub fn new(status_code: u16, value: &str) -> Self {
        ApiError {
            status_code,
            error: value.to_string(),
        }
    }

    pub fn new_internal(value: &str) -> Self {
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: value.to_string(),
        }
    }

    pub fn new_bad_request(value: &str) -> Self {
        ApiError {
            status_code: StatusCode::BAD_REQUEST.as_u16(),
            error: value.to_string(),
        }
    }

    pub fn unauthorized() -> Self {
        ApiError {
            status_code: StatusCode::UNAUTHORIZED.as_u16(),
            error: "unauthorized access".to_string(),
        }
    }

    pub fn internal_error<E>(err: E) -> Self
    where
        E: std::error::Error,
    {
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            error: err.to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(self.status_code).unwrap(), Json(self)).into_response()
    }
}
