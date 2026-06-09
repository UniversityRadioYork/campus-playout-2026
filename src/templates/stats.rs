use crate::model::TrackRequestStat;

impl super::TemplateRenderer {
    pub fn request_stats(&self, stats: Vec<TrackRequestStat>) -> maud::Markup {
        self.page(Some("Track request stats"), maud::html! {
            h2 { "Track request stats" }

            table.table {
                thead {
                    tr {
                        th scope="col" { "Track" }
                        th scope="col" { "Plays" }
                        th scope="col" { "Last requested" }
                    }
                }

                tbody {
                    @for stat in stats {
                        tr {
                            th scope="row" { (format!("{} - {}", stat.track.title, stat.track.artist)) }
                            td { (stat.plays) }
                            td { (stat.last_requested_at) }
                        }
                    }
                }
            }
        })
    }
}
