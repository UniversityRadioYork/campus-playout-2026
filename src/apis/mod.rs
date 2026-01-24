use std::fmt::Debug;

use miette::{Context, IntoDiagnostic};
use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::apis::{lastfm::{GetTrackInfoResponse, LastFmResponse}, myradio::{MyRadioPlaylist, MyRadioResponse, MyRadioTrack}};

mod lastfm;
pub mod myradio;

const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (+https://ury.org.uk)"
);

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    last_fm_api_key: String,
    myradio_api_base: String,
    myradio_api_key: String,
}

impl ApiClient {
    pub fn new(last_fm_api_key: String, myradio_api_base: String, myradio_api_key: String) -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent(USER_AGENT)
                .build()
                .unwrap(),
            last_fm_api_key,
            myradio_api_base,
            myradio_api_key,
        }
    }

    pub fn myradio_api_key(&self) -> &str {
        &self.myradio_api_key
    }

    async fn myradio_get(
        &self,
        base: &str,
        path: String,
    ) -> miette::Result<reqwest::Response> {
        let url = format!("{}{path}", base);

        let resp = self
            .client
            .get(url)
            .query(&[("api_key", &self.myradio_api_key)])
            .send()
            .await
            .into_diagnostic()
            .with_context(|| "sending request to myradio")?;

        Ok(resp)
    }

    async fn myradio_api_get<T: Debug + DeserializeOwned>(
        &self,
        path: impl Into<String>,
    ) -> miette::Result<T> {
        let resp = self.myradio_get(&self.myradio_api_base, path.into()).await?;
        let status = resp.status();
        let body: MyRadioResponse<T> = resp.json().await.into_diagnostic().with_context(|| "parsing myradio response body")?;
        match body {
            MyRadioResponse::Ok { payload } if status.is_success() => Ok(payload),
            MyRadioResponse::Fail { payload } if !status.is_success() => {
                miette::bail!("myradio error: {} (status code: {})", payload, status.as_u16());
            }
            body => {
                unreachable!(
                    "myradio should not give a response like this {status:?} (body {body:?})"
                )
            }
        }
    }

    pub async fn get_track_info(&self, track_id: i64) -> miette::Result<MyRadioTrack> {
        self.myradio_api_get(format!("/track/{track_id}")).await
    }

    pub async fn get_all_playlists(&self) -> miette::Result<Vec<MyRadioPlaylist>> {
        self.myradio_api_get("/playlist/allitonesplaylists").await
    }

    pub async fn get_playlist_tracks(&self, playlist_id: &str) -> miette::Result<Vec<MyRadioTrack>> {
        self.myradio_api_get(format!("/playlist/{playlist_id}/tracks")).await
    }

    pub async fn get_cover_art_for_track(
        &self,
        title: &str,
        artist: &str,
    ) -> miette::Result<Option<String>> {
        let resp: LastFmResponse<GetTrackInfoResponse> = self
            .client
            .get("http://ws.audioscrobbler.com/2.0/")
            .query(&[
                ("method", "track.getInfo"),
                ("track", title),
                ("artist", artist),
                ("api_key", &self.last_fm_api_key),
                ("format", "json"),
            ])
            .send()
            .await
            .and_then(|r| r.error_for_status())
            .into_diagnostic()
            .with_context(|| format!("fetching {title} - {artist} from last.fm"))?
            .json()
            .await
            .into_diagnostic()
            .with_context(|| "parsing last.fm response")?;

        let resp = match resp {
            LastFmResponse::Ok(resp) => resp,
            LastFmResponse::Error { error, message } => miette::bail!("last.fm returned error {error}: {message}"),
        };

        if let Some(album) = resp.track.album {
            for image in album.image {
                if image.size == "extralarge" {
                    return Ok(Some(image.url));
                }
            }
        }

        Ok(None)
    }
}
