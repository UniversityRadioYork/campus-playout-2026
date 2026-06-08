-- Add migration script here
CREATE TABLE track_requests
(
    track_request_id INTEGER PRIMARY KEY NOT NULL,
    trackid INTEGER NOT NULL,
    requested_at BOOLEAN NOT NULL
);

ALTER TABLE recent_tracks ADD COLUMN was_request BOOLEAN NOT NULL DEFAULT TRUE;
