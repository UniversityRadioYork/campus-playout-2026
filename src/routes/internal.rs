use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::Deserialize;

use crate::{auth::ValidApiToken, responses::M3U8Playlist, state::AppState};

#[derive(Debug, Deserialize)]
struct MetadataPayload {
    trackid: Option<i64>,
    was_request: Option<bool>,
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
        state
            .database
            .track_played(trackid, metadata.was_request.unwrap_or(false))
            .await?;
    } else {
        tracing::info!("track ended");
    }
    Ok(())
}

async fn jingles_playlist(
    _token: ValidApiToken,
    State(state): State<AppState>,
) -> crate::Result<M3U8Playlist> {
    let playlist = state.playlist_generator.get_jingles_playlist().await?;
    Ok(M3U8Playlist(playlist))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/metadata", post(new_metadata))
        .route("/jingles.m3u8", get(jingles_playlist))
}
