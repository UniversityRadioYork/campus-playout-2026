use std::fmt::Debug;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
pub enum MyRadioResponse<T: Debug> {
    #[serde(rename = "OK")]
    Ok { payload: T },
    #[serde(rename = "FAIL")]
    Fail { payload: String },
}

#[derive(Debug, Deserialize)]
pub struct MyRadioTrack {
    pub title: String,
    pub artist: String,
    #[serde(rename = "trackid")]
    pub track_id: i64,
}

impl MyRadioTrack {
    pub fn url(&self, api_key: &str, is_request: bool) -> String {
        let extra_meta = if is_request {
            ",is_request=\"true\""
        } else {
            ""
        };
        // TODO: unhardcode myradio URL
        format!(
            "lufs_track_gain:annotate:trackid=\"{track_id}\"{extra_meta}:https://ury.org.uk/myradio/NIPSWeb/secure_play?trackid={track_id}&api_key={api_key}",
            track_id = self.track_id
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct MyRadioManagedItem {
    pub title: String,
    #[serde(rename = "managedid")]
    pub managed_id: i64,
    pub expired: bool,
}

#[derive(Debug, Deserialize)]
pub struct MyRadioPlaylist {
    pub title: String,
    #[serde(rename = "playlistid")]
    pub playlist_id: String,
    pub category: MyRadioPlaylistCategory,
}

#[derive(Debug, Deserialize)]
pub struct MyRadioPlaylistCategory {
    pub id: String,
}
