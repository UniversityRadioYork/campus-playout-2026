use std::fmt::Write;
use std::path::PathBuf;

use miette::{Context, IntoDiagnostic};

use crate::apis::ApiClient;

#[derive(Clone)]
pub struct PlaylistGenerator {
    client: ApiClient,
    playlist_file_path: PathBuf,
}

impl PlaylistGenerator {
    pub fn new(client: ApiClient, playlist_file_path: PathBuf) -> Self {
        Self {
            client,
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
                "annotate:trackid=\"{track_id}\":https://ury.org.uk/myradio/NIPSWeb/secure_play?trackid={track_id}&api_key={api_key}",
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
}
