use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::templates;

const INTERNAL_SERVER_ERROR_TITLE: &str = "Internal server error";
#[cfg_attr(debug_assertions, allow(unused))]
const INTERNAL_SERVER_ERROR_BODY: &[&str] = &[
    "Something went wrong, please try again later.",
    "If you are still experiencing issues, contact the Computing Team",
];

const NOT_FOUND_TITLE: &str = "Page not found";
const NOT_FOUND_BODY: &[&str] = &[
    "If you typed the web address, check it is correct.",
    "If you pasted the web address, check you copied the entire address.",
];

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal server error: {0}")]
    Miette(miette::Report),
    #[error("not found")]
    NotFound,
    #[error("unauthorized")]
    Unauthorized,
}

impl From<miette::Report> for Error {
    fn from(value: miette::Report) -> Self {
        Self::Miette(value)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Miette(e) => {
                let s: String = format!("{e:?}");
                let formatted = ansi_to_html::convert(&s).unwrap();
                tracing::error!("internal server error: {e}");
                #[cfg(not(debug_assertions))]
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    templates::error_page(INTERNAL_SERVER_ERROR_TITLE, INTERNAL_SERVER_ERROR_BODY),
                )
                    .into_response();
                #[cfg(debug_assertions)]
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    templates::error_page(
                        INTERNAL_SERVER_ERROR_TITLE,
                        &[maud::PreEscaped(format!("<pre>{formatted}</pre>"))],
                    ),
                )
                    .into_response();
            }
            Error::NotFound => (
                StatusCode::NOT_FOUND,
                templates::error_page(NOT_FOUND_TITLE, NOT_FOUND_BODY),
            )
                .into_response(),
            Error::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": true,
                    "message": "unauthorized",
                })),
            ).into_response(),
        }
    }
}
