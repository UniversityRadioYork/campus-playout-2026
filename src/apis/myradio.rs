use std::fmt::Debug;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
pub enum MyRadioResponse<T: Debug> {
    #[serde(rename = "OK")]
    Ok {
        payload: T,
    },
    #[serde(rename = "FAIL")]
    Fail {
        payload: String,
    }
}

#[derive(Debug, Deserialize)]
pub struct MyRadioTrack {
    pub title: String,
    pub artist: String,
    #[serde(rename = "trackid")]
    pub track_id: i64,
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
