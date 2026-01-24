-- Add migration script here
CREATE TABLE recent_tracks
(
    recent_track_id INTEGER PRIMARY KEY NOT NULL,
    trackid INTEGER NOT NULL,
    played_at TEXT NOT NULL,
    now_playing BOOLEAN NOT NULL
);
