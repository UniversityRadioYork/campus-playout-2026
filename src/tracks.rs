use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    apis::ApiClient,
    model::{RecentTrack, RecentTrackRecord, Track},
};

#[derive(Clone)]
pub struct TrackCache {
    inner: Arc<Mutex<HashMap<i64, Track>>>,
    client: ApiClient,
}

impl TrackCache {
    pub fn new(client: ApiClient) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            client,
        }
    }

    async fn fetch_track(&self, track_id: i64) -> miette::Result<Track> {
        let track = self.client.get_track_info(track_id).await?;
        let cover_art = self
            .client
            .get_cover_art_for_track(&track.title, &track.artist)
            .await?;
        Ok(Track {
            trackid: track.track_id,
            title: track.title,
            artist: track.artist,
            cover_art,
        })
    }

    pub async fn get_track(&self, track_id: i64) -> miette::Result<Track> {
        let mut inner = self.inner.lock().await;
        if let Some(track) = inner.get(&track_id) {
            Ok(track.clone())
        } else {
            let track = self.fetch_track(track_id).await?;
            inner.insert(track_id, track.clone());
            Ok(track)
        }
    }

    pub async fn resolve_recent_tracks(
        &self,
        tracks: Vec<RecentTrackRecord>,
    ) -> miette::Result<Vec<RecentTrack>> {
        let mut recent_tracks = Vec::with_capacity(tracks.len());
        for recent_track in tracks {
            let track = self.get_track(recent_track.trackid).await?;
            recent_tracks.push(RecentTrack {
                track,
                now_playing: recent_track.now_playing,
                played_at: recent_track.played_at,
                was_request: recent_track.was_request,
            });
        }
        Ok(recent_tracks)
    }
}
