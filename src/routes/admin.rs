use std::time::Duration;

use axum::{
    Form, Router,
    extract::{Query, State},
    routing::{get, post},
};
use miette::{Context, IntoDiagnostic};
use serde::Deserialize;

use crate::{apis::myradio::MyRadioPlaylist, liquidsoap::LiquidsoapBackend, state::AppState};

#[derive(Deserialize)]
struct SetPlaylistBody {
    playlist_id: String,
}

#[derive(Deserialize)]
struct TrackSearchQuery {
    query: String,
}

#[derive(Deserialize)]
struct TrackRequestBody {
    track_id: i64,
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
    Ok(state.template_renderer.admin_index(
        track,
        recent_tracks,
        current_playlist.as_deref(),
        available_playlists,
    ))
}

async fn set_playlist(
    State(state): State<AppState>,
    Form(body): Form<SetPlaylistBody>,
) -> crate::Result<maud::Markup> {
    tokio::time::sleep(Duration::from_secs(3)).await;

    state
        .playlist_generator
        .update_playlist(&body.playlist_id)
        .await?;
    state
        .database
        .set_current_playlist(&body.playlist_id)
        .await?;

    let (current_playlist, available_playlists) = get_playlist_info(&state).await?;
    Ok(state
        .template_renderer
        .selected_playlist(current_playlist.as_deref(), available_playlists))
}

async fn skip_track(State(state): State<AppState>) -> crate::Result<()> {
    state
        .liquidsoap
        .skip_track()
        .await
        .into_diagnostic()
        .with_context(|| "Failed to contact Liquidsoap instance")?;

    Ok(())
}

async fn track_search(
    State(state): State<AppState>,
    Query(query): Query<TrackSearchQuery>,
) -> crate::Result<maud::Markup> {
    let mut tracks = Vec::with_capacity(50);
    tracks.append(
        &mut state
            .api_client
            .search_track(Some(&query.query), None)
            .await
            .with_context(|| "searching for track by name")?,
    );
    tracks.append(
        &mut state
            .api_client
            .search_track(None, Some(&query.query))
            .await
            .with_context(|| "searching for track by artist")?,
    );

    Ok(state.template_renderer.track_search_results(&tracks))
}

async fn track_request(
    State(state): State<AppState>,
    Form(body): Form<TrackRequestBody>,
) -> crate::Result<()> {
    let track = state
        .api_client
        .get_track_info(body.track_id)
        .await
        .with_context(|| format!("fetching track information for {}", body.track_id))?;

    let track_url = track.url(state.api_client.myradio_api_key(), true);

    tracing::info!(track_id = ?body.track_id, ?track_url, "track requested");

    state
        .liquidsoap
        .request_track(&track_url)
        .await
        .into_diagnostic()
        .with_context(|| "sending request to liquidsoap")?;

    state.database.track_requested(track.track_id).await?;

    Ok(())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_page))
        .route("/playlist", post(set_playlist))
        .route("/skip", post(skip_track))
        .route("/track/search", get(track_search))
        .route("/track/request", post(track_request))
}
