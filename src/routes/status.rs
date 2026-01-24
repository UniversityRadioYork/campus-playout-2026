use axum::{Router, extract::State, routing::get};

use crate::state::AppState;

async fn get_playing(State(state): State<AppState>) -> crate::Result<maud::Markup> {
    let track_id = state.database.get_now_playing().await?;

    let track = if let Some(trackid) = track_id {
        Some(state.track_cache.get_track(trackid).await?)
    } else {
        None
    };

    Ok(state.template_renderer.now_playing(track))
}

async fn get_recent_tracks(State(state): State<AppState>) -> crate::Result<maud::Markup> {
    let tracks = state.database.get_recent_tracks().await?;
    let tracks = state.track_cache.resolve_recent_tracks(tracks).await?;

    Ok(state.template_renderer.recent_tracks(tracks))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/playing", get(get_playing))
        .route("/recent", get(get_recent_tracks))
}
