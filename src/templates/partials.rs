use crate::{
    apis::myradio::MyRadioPlaylist,
    model::{RecentTrack, Track},
};

impl super::TemplateRenderer {
    pub fn now_playing(&self, track: Option<Track>) -> maud::Markup {
        maud::html! {
            h2 { "Now Playing" }

            .card style="width: 300px" {
                @if let Some(track) = track {
                    @if let Some(cover_art) = track.cover_art {
                        img.card-img-top loading="lazy" alt="" src=(cover_art);
                    } @else {
                        // TODO: per-instance art
                        img.card-img-top loading="lazy" alt="" src="https://ury.org.uk/images/default_show_profile.png";
                    }

                    .card-body {
                        h5.card-title { (track.title) }
                        p.card-text { (track.artist) }
                    }
                } @else {
                    // TODO: per-instance art
                    img.card-img-top loading="lazy" alt="" src="https://ury.org.uk/images/default_show_profile.png";

                    .card-body {
                        h5.card-title { (self.instance_name) }
                        p.card-text { "No song is playing right now!" }
                    }
                }
            }
        }
    }

    pub fn recent_tracks(&self, tracks: Vec<RecentTrack>) -> maud::Markup {
        maud::html! {
            h2 { "Recent Tracks" }

            table.table style="max-width: 720px" {
                thead {
                    tr {
                        th scope="col" { "Played at" }
                        th scope="col" { "Track" }
                    }
                }

                tbody {
                    @for track in tracks {
                        tr {
                            td {
                                @if track.now_playing {
                                    span.playing-bars {
                                        span.bar-1.bg-secondary {}
                                        span.bar-2.bg-secondary {}
                                        span.bar-3.bg-secondary {}
                                    }
                                } @else {
                                    (track.played_at)
                                }
                            }
                            th scope="row" {
                                (format!("{} - {}", track.track.title, track.track.artist))
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn selected_playlist(
        &self,
        current_playlist_id: Option<&str>,
        available_playlists: Vec<MyRadioPlaylist>,
    ) -> maud::Markup {
        let mut current_playlist = "";

        if let Some(current_playlist_id) = current_playlist_id {
            for playlist in &available_playlists {
                if playlist.playlist_id == current_playlist_id {
                    current_playlist = &playlist.title;
                }
            }
        }

        maud::html! {
            #change_playlist {
                p {
                    @if current_playlist != "" {
                        "Current playlist: " (current_playlist)
                    } @else {
                        "No playlist selected!?"
                    }
                }

                form.row.px-2.pb-2 hx-post="/playlist" hx-target="#change_playlist" hx-swap="outerHTML" hx-disabled-elt="find button" {
                    select.form-select.col-sm-10 aria-label="Select current playlist" style="max-width: 360px" name="playlist_id" {
                        @for playlist in available_playlists {
                            option selected[Some(&*playlist.playlist_id) == current_playlist_id] value=(playlist.playlist_id) { (playlist.title) }
                        }
                    }

                    .col-auto {
                        button.btn.btn-primary type="submit" {
                            span.htmx-indicator.spinner-border.spinner-border-sm aria-hidden="true" {}
                            (" ")
                            span { "Change playlist" }
                        }
                    }
                }

                p {
                    "The next song will be from the selected playlist"
                }
            }
        }
    }
}
