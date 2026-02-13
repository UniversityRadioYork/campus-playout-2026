use std::path::PathBuf;

use campus_playout_2026::{
    apis::ApiClient, database::AppDatabase, liquidsoap, playlist::PlaylistGenerator, routes,
    state::AppState, templates::TemplateRenderer, tracks::TrackCache,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;

macro_rules! get_env {
    ($name:expr) => {
        std::env::var($name).expect(concat!("environment variable ", $name, " to be set"))
    };
}

#[tokio::main]
async fn main() -> campus_playout_2026::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("info,{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let api_token = get_env!("API_TOKEN");

    let database = AppDatabase::new(&get_env!("DATABASE_URL")).await?;

    let client = ApiClient::new(
        get_env!("LAST_FM_API_KEY"),
        get_env!("MYRADIO_API_BASE"),
        get_env!("MYRADIO_API_KEY"),
        get_env!("PLAYLIST_CATEGORY_ID"),
    );

    let track_cache = TrackCache::new(client.clone());

    let stream_base = get_env!("HLS_BASE_URL");
    let stream_id = get_env!("SRT_STREAM_ID");

    let template_renderer = TemplateRenderer::new(
        get_env!("INSTANCE_NAME"),
        format!("{stream_base}/{stream_id}/index.m3u8"),
    );

    let playlist_generator =
        PlaylistGenerator::new(client.clone(), PathBuf::from(get_env!("PLAYLIST_FILE")));

    database.stop_all_tracks().await?;

    // TODO: prepare playlist file
    let playlist_id = if let Some(playlist_id) = database.get_current_playlist().await? {
        playlist_id
    } else {
        let default_playlist_id = get_env!("DEFAULT_PLAYLIST_ID");
        database.set_current_playlist(&default_playlist_id).await?;
        default_playlist_id
    };

    playlist_generator.update_playlist(&playlist_id).await?;

    let liquidsoap = liquidsoap::unix(get_env!("UNIX_SOCKET_PATH"));

    let state = AppState::new(
        client,
        api_token,
        database,
        playlist_generator,
        template_renderer,
        track_cache,
        liquidsoap,
    );

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
