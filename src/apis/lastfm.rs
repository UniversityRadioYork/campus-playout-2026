use std::fmt::Debug;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum LastFmResponse<T: Debug> {
    Ok(T),
    Error { error: i32, message: String },
}

#[derive(Debug, Deserialize)]
pub struct GetTrackInfoResponse {
    pub track: Track,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub album: Option<Album>,
}

#[derive(Debug, Deserialize)]
pub struct Album {
    pub image: Vec<Image>,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub size: String,
    #[serde(rename = "#text")]
    pub url: String,
}
