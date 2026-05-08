use axum::extract::{FromRequestParts, Query};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use reqwest::Method;
use serde::Deserialize;

use crate::{error::Error, state::AppState};

pub struct ValidApiToken;

#[derive(Deserialize)]
struct ApiTokenQuery {
    api_key: String,
}

impl FromRequestParts<AppState> for ValidApiToken {
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await;
        match header {
            Ok(authorization) => {
                if authorization.token() == state.api_token {
                    return Ok(Self);
                }
            }
            Err(e) => {
                if e.is_missing() {
                    if parts.method == Method::GET {
                        let query = Query::<ApiTokenQuery>::from_request_parts(parts, state).await;
                        if let Ok(query) = query {
                            if query.api_key == state.api_token {
                                return Ok(Self);
                            }
                        }
                    }
                }
            }
        }
        Err(Error::Unauthorized)
    }
}
