use time::PrimitiveDateTime;

#[derive(Clone, Debug)]
pub struct Track {
    pub trackid: i64,
    pub title: String,
    pub artist: String,
    pub cover_art: Option<String>,
}

pub struct RecentTrackRecord {
    pub recent_track_id: i64,
    pub trackid: i64,
    pub played_at: PrimitiveDateTime,
    pub now_playing: bool,
    pub was_request: bool,
}

pub struct RecentTrack {
    pub played_at: PrimitiveDateTime,
    pub now_playing: bool,
    pub track: Track,
    pub was_request: bool,
}

pub struct TrackRequestRecord {
    pub track_request_id: Option<i64>,
    pub trackid: Option<i64>,
    pub requested_at: PrimitiveDateTime,
}

pub struct TrackRequestStatRecord {
    pub trackid: Option<i64>,
    pub plays: i64,
    pub last_requested_at: PrimitiveDateTime,
}

pub struct TrackRequestStat {
    pub track: Track,
    pub plays: i64,
    pub last_requested_at: PrimitiveDateTime,
}

pub struct TrackRequest {
    pub requested_at: PrimitiveDateTime,
    pub track: Track,
}
