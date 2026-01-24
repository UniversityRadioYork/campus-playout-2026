use std::time::Duration;

use axum::{
    Form, Router,
    extract::State,
    routing::{get, post},
};
use serde::Deserialize;

use crate::{apis::myradio::MyRadioPlaylist, state::AppState};

#[derive(Deserialize)]
struct SetPlaylistBody {
    playlist_id: String,
}

async fn get_playlist_info(
    state: &AppState,
) -> crate::Result<(Option<String>, Vec<MyRadioPlaylist>)> {
    let current_playlist = state.database.get_current_playlist().await?;
    let available_playlists = state.api_client.get_all_playlists().await?;
    Ok((current_playlist, available_playlists))
}

async fn admin_page(State(state): State<AppState>) -> crate::Result<maud::Markup> {
    let track_id = state.database.get_now_playing().await?;

    let track = if let Some(trackid) = track_id {
        Some(state.track_cache.get_track(trackid).await?)
    } else {
        None
    };

    let tracks = state.database.get_recent_tracks().await?;
    let recent_tracks = state.track_cache.resolve_recent_tracks(tracks).await?;

    let (current_playlist, available_playlists) = get_playlist_info(&state).await?;
    Ok(state
        .template_renderer
        .admin_index(track, recent_tracks, current_playlist.as_deref(), available_playlists))
}

async fn set_playlist(
    State(state): State<AppState>,
    Form(body): Form<SetPlaylistBody>,
) -> crate::Result<maud::Markup> {
    tokio::time::sleep(Duration::from_secs(3)).await;

    state.playlist_generator.update_playlist(&body.playlist_id).await?;
    state.database.set_current_playlist(&body.playlist_id).await?;

    let (current_playlist, available_playlists) = get_playlist_info(&state).await?;
    Ok(state
        .template_renderer
        .selected_playlist(current_playlist.as_deref(), available_playlists))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_page))
        .route("/playlist", post(set_playlist))
}
