use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use thiserror::Error;

#[derive(Error, Debug)]

pub enum AppError {
    #[error("database error")]
    Database(#[from] sqlx::Error),

    #[error("internal error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
