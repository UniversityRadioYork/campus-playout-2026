use axum::extract::FromRequestParts;
use axum_extra::{TypedHeader, headers::{Authorization, authorization::Bearer}};

use crate::{error::Error, state::AppState};

pub struct ValidApiToken;

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
                    Ok(Self)
                } else {
                    Err(Error::Unauthorized)
                }
            },
            Err(_) => Err(Error::Unauthorized),
        }
    }
}
