use maud::PreEscaped;

use crate::templates::TemplateRenderer;

impl TemplateRenderer {
    pub fn player(&self) -> maud::Markup {
        maud::html! {
            .card.mt-2 style="width: 300px" {
                .card-header {
                    "Player"
                }

                audio #player {}

                .card-body style="display: flex; gap: 10px;" {
                    button.btn.btn-outline-primary #toggle-button type="button" {
                        "Play"
                    }

                    button.btn.btn-primary hx-post="/skip" hx-swap="none" {
                        span.htmx-indicator.spinner-border.spinner-border-sm aria-hidden="true" {}
                        (" ")
                        span { "Skip" }
                    }
                }

                script src="https://cdn.jsdelivr.net/npm/hls.js@1.6.15/dist/hls.min.js" integrity="sha256-QTqD4rsMd+0L8L4QXVOdF+9F39mEoLE+zTsUqQE4OTg=" crossorigin="anonymous" {}
                script defer data-stream=(self.stream_url) { (PreEscaped(include_str!("../assets/player.js"))) }
            }
        }
    }
}
