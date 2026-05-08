use std::path::PathBuf;
use std::{fmt::Write, path::Path};

use miette::{Context, IntoDiagnostic};

use crate::{apis::ApiClient, config::JinglesConfig};

#[derive(Clone)]
pub struct PlaylistGenerator {
    client: ApiClient,
    jingles_config: JinglesConfig,
    playlist_file_path: PathBuf,
}

impl PlaylistGenerator {
    pub fn new(
        client: ApiClient,
        jingles_config: JinglesConfig,
        playlist_file_path: PathBuf,
    ) -> Self {
        Self {
            client,
            jingles_config,
            playlist_file_path,
        }
    }

    pub async fn update_playlist(&self, playlist_id: &str) -> miette::Result<()> {
        let tracks = self.client.get_playlist_tracks(playlist_id).await?;
        let mut s = String::new();
        for track in tracks {
            let _ = writeln!(
                s,
                // TODO: unhardcode
                "lufs_track_gain:annotate:trackid=\"{track_id}\":https://ury.org.uk/myradio/NIPSWeb/secure_play?trackid={track_id}&api_key={api_key}",
                track_id = track.track_id,
                api_key = self.client.myradio_api_key(),
            );
        }

        tokio::fs::write(&self.playlist_file_path, s)
            .await
            .into_diagnostic()
            .with_context(|| "writing playlist file")?;

        Ok(())
    }

    async fn read_playlist(&self, path: &Path) -> miette::Result<String> {
        let s = tokio::fs::read_to_string(path)
            .await
            .into_diagnostic()
            .with_context(|| format!("reading playlist from {path:?}"))?;
        Ok(s)
    }

    async fn push_playlist(
        &self,
        playlist: &mut String,
        label: &str,
        path: &Path,
    ) -> miette::Result<()> {
        playlist.push('#');
        playlist.push_str(label);
        playlist.push('\n');
        for line in self.read_playlist(path).await?.lines() {
            playlist.push_str("lufs_track_gain:");
            playlist.push_str(line);
            playlist.push('\n');
        }
        Ok(())
    }

    pub async fn get_jingles_playlist(&self) -> miette::Result<String> {
        let mut playlist = String::new();

        let promos = self
            .client
            .get_managed_playlist_items(&self.jingles_config.promos_playlist_id)
            .await?;

        for promo in promos {
            let _ = writeln!(
                playlist,
                // TODO: unhardcode
                "lufs_track_gain:https://ury.org.uk/myradio/NIPSWeb/managed_play?managedid={managed_id}&api_key={api_key}",
                managed_id = promo.managed_id,
                api_key = self.client.myradio_api_key(),
            );
        }

        self.push_playlist(
            &mut playlist,
            "main jingles",
            &self.jingles_config.main_playlist,
        )
        .await?;
        let now = time::OffsetDateTime::now_local()
            .into_diagnostic()
            .with_context(|| "getting the current local time")?;

        let hour = now.time().hour();
        let timed_playlist = match hour {
            6..13 => &self.jingles_config.morning_playlist,
            13..18 => &self.jingles_config.afternoon_playlist,
            _ => &self.jingles_config.evening_playlist,
        };

        self.push_playlist(&mut playlist, "timed", timed_playlist)
            .await?;

        Ok(playlist)
    }
}
