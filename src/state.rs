use axum::extract::FromRef;

use crate::{apis::ApiClient, database::AppDatabase, playlist::PlaylistGenerator, templates::TemplateRenderer, tracks::TrackCache};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub api_client: ApiClient,
    pub api_token: String,
    pub database: AppDatabase,
    pub playlist_generator: PlaylistGenerator,
    pub template_renderer: TemplateRenderer,
    pub track_cache: TrackCache,
}

impl AppState {
    pub fn new(api_client: ApiClient, api_token: String, database: AppDatabase, playlist_generator: PlaylistGenerator, template_renderer: TemplateRenderer, track_cache: TrackCache) -> Self {
        Self {
            api_client,
            api_token,
            database,
            playlist_generator,
            template_renderer,
            track_cache,
        }
    }
}
