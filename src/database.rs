use miette::{Context, IntoDiagnostic};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

use crate::model::RecentTrackRecord;

const CURRENT_PLAYLIST_KEY: &str = "current_playlist";

#[derive(Clone)]
pub struct AppDatabase {
    pool: SqlitePool,
}

impl AppDatabase {
    pub async fn new(url: &str) -> miette::Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .into_diagnostic()
            .with_context(|| "connecting to database")?;

        sqlx::migrate!()
            .run(&pool)
            .await
            .into_diagnostic()
            .with_context(|| "running migrations")?;

        Ok(Self { pool })
    }

    pub async fn stop_all_tracks(&self) -> crate::Result<()> {
        sqlx::query!("UPDATE recent_tracks SET now_playing = FALSE")
            .execute(&self.pool)
            .await
            .into_diagnostic()
            .with_context(|| "marking tracks as not playing")?;

        Ok(())
    }

    pub async fn track_played(&self, trackid: i64, was_request: bool) -> crate::Result<()> {
        sqlx::query!("INSERT INTO recent_tracks(trackid, played_at, now_playing, was_request) VALUES(?, datetime(), TRUE, ?)", trackid, was_request)
            .execute(&self.pool)
            .await
            .into_diagnostic()
            .with_context(|| "inserting new tracklist entry")?;

        Ok(())
    }

    pub async fn get_now_playing(&self) -> crate::Result<Option<i64>> {
        let resp = sqlx::query!("SELECT trackid FROM recent_tracks WHERE now_playing = TRUE ORDER BY played_at DESC LIMIT 1")
            .fetch_optional(&self.pool)
            .await.into_diagnostic().with_context(|| "getting currently playing track")?;
        Ok(resp.map(|r| r.trackid))
    }

    pub async fn get_recent_tracks(&self) -> crate::Result<Vec<RecentTrackRecord>> {
        let tracks = sqlx::query_as!(RecentTrackRecord, "SELECT recent_track_id, trackid, played_at as \"played_at: time::PrimitiveDateTime\", now_playing, was_request FROM recent_tracks ORDER BY played_at DESC LIMIT 6")
            .fetch_all(&self.pool)
            .await.into_diagnostic().with_context(|| "getting recently played tracks")?;
        Ok(tracks)
    }

    pub async fn track_requested(&self, track_id: i64) -> crate::Result<()> {
        sqlx::query!(
            "INSERT INTO track_requests(trackid, requested_at) VALUES(?, datetime())",
            track_id
        )
        .execute(&self.pool)
        .await
        .into_diagnostic()
        .with_context(|| "inserting request log entry")?;

        Ok(())
    }

    async fn get_key(&self, key: &str) -> crate::Result<Option<String>> {
        let value = sqlx::query!("SELECT v FROM kv WHERE k = ?", key)
            .fetch_optional(&self.pool)
            .await
            .into_diagnostic()
            .with_context(|| format!("getting key {key}"))?;
        Ok(value.map(|v| v.v))
    }

    async fn set_key(&self, key: &str, value: &str) -> crate::Result<()> {
        sqlx::query!(
            "INSERT INTO kv (k, v) VALUES (?1, ?2) ON CONFLICT(k) DO UPDATE SET v = ?2",
            key,
            value
        )
        .execute(&self.pool)
        .await
        .into_diagnostic()
        .with_context(|| format!("failed to set key {key} = {value:?}"))?;
        Ok(())
    }

    pub async fn get_current_playlist(&self) -> crate::Result<Option<String>> {
        self.get_key(CURRENT_PLAYLIST_KEY).await
    }

    pub async fn set_current_playlist(&self, playlist_id: &str) -> crate::Result<()> {
        self.set_key(CURRENT_PLAYLIST_KEY, playlist_id).await
    }
}
