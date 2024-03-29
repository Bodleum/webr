use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Nav(#[from] crate::utils::Error),
    #[error(transparent)]
    Markdown(#[from] crate::markdown::Error),
    #[error(transparent)]
    Lectionary(#[from] crate::lectionary::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),

    #[error(transparent)]
    Axum(#[from] axum::http::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::NOT_FOUND, format!("{self:?}")).into_response()
    }
}
