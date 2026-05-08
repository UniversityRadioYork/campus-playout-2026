use axum::{
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};

pub struct M3U8Playlist(pub String);

impl IntoResponse for M3U8Playlist {
    fn into_response(self) -> axum::response::Response {
        let mut headers = HeaderMap::new();
        // i would like to do this typed_insert, but some methods from ContentDisposition only has a method for creating
        // Content-Disposition: inline
        headers.insert(
            "Content-Disposition",
            HeaderValue::from_static(r#"attachment;filename="playlist.m3u8""#),
        );

        (headers, self.0).into_response()
    }
}
