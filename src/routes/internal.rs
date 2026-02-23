use axum::{Json, Router, extract::State, routing::post};
use serde::Deserialize;

use crate::{auth::ValidApiToken, state::AppState};

#[derive(Debug, Deserialize)]
struct MetadataPayload {
    trackid: Option<i64>,
}

async fn new_metadata(
    _token: ValidApiToken,
    State(state): State<AppState>,
    Json(metadata): Json<MetadataPayload>,
) -> crate::Result<()> {
    state.database.stop_all_tracks().await?;
    if let Some(trackid) = metadata.trackid {
        tracing::info!(?trackid, "new track started");
        // track
        state.database.track_played(trackid).await?;
    } else {
        tracing::info!("track ended");
    }
    Ok(())
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/metadata", post(new_metadata))
}
