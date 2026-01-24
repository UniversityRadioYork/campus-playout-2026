use std::path::PathBuf;

use campus_playout_2026::{
    apis::ApiClient, database::AppDatabase, playlist::PlaylistGenerator, routes, state::AppState,
    templates::TemplateRenderer, tracks::TrackCache,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> campus_playout_2026::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("info,{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let api_token = std::env::var("API_TOKEN").expect("environment variable API_TOKEN to be set");

    let database = AppDatabase::new(
        &std::env::var("DATABASE_URL").expect("environment variable DATABASE_URL to be set"),
    )
    .await?;

    let client = ApiClient::new(
        std::env::var("LAST_FM_API_KEY").expect("environment variable LAST_FM_API_KEY to be set"),
        std::env::var("MYRADIO_API_BASE").expect("environment variable MYRADIO_API_BASE to be set"),
        std::env::var("MYRADIO_API_KEY").expect("environment variable MYRADIO_API_KEY to be set"),
    );

    let track_cache = TrackCache::new(client.clone());

    let template_renderer = TemplateRenderer::new(
        std::env::var("INSTANCE_NAME").expect("environment variable INSTANCE_NAME to be set"),
    );

    let playlist_generator = PlaylistGenerator::new(
        client.clone(),
        PathBuf::from(std::env::var("PLAYLIST_FILE").expect("environment variable PLAYLIST_FILE to be set")),
    );

    database.stop_all_tracks().await?;

    // TODO: prepare playlist file
    let playlist_id = if let Some(playlist_id) = database.get_current_playlist().await? {
        playlist_id
    } else {
        let default_playlist_id = std::env::var("DEFAULT_PLAYLIST_ID")
            .expect("environment variable DEFAULT_PLAYLIST_ID to be set");
        database.set_current_playlist(&default_playlist_id).await?;
        default_playlist_id
    };

    playlist_generator.update_playlist(&playlist_id).await?;

    let state = AppState::new(client, api_token, database, playlist_generator, template_renderer, track_cache);

    let app = routes::routes(state).layer(TraceLayer::new_for_http());

    let host = std::env::var("HOST").unwrap_or_else(|_| "[::1]".to_string());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("valid port number");

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
