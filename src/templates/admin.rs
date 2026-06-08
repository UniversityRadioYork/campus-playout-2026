use crate::{
    apis::myradio::MyRadioPlaylist,
    model::{RecentTrack, Track},
};

impl super::TemplateRenderer {
    pub fn admin_index(
        &self,
        track: Option<Track>,
        recent_tracks: Vec<RecentTrack>,
        current_playlist_id: Option<&str>,
        available_playlists: Vec<MyRadioPlaylist>,
    ) -> maud::Markup {
        self.page(None, maud::html! {
            .row {
                .col-md-auto {
                    div hx-get="/status/playing" hx-swap="morph:innerHTML" hx-trigger="every 2s" {
                        (self.now_playing(track))
                    }

                    (self.player())
                }

                .col {
                    div hx-get="/status/recent" hx-swap="morph:innerHTML" hx-trigger="every 2s" {
                        (self.recent_tracks(recent_tracks))
                    }

                    div {
                        (self.selected_playlist(current_playlist_id, available_playlists))
                    }

                    div {
                        (self.track_request())
                    }
                }
            }
        })
    }
}
