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

                .card-body {
                    button.btn.btn-outline-primary #toggle-button type="button" {
                        "Play"
                    }
                }

                script src="https://cdn.jsdelivr.net/npm/hls.js@1.6.15/dist/hls.min.js" integrity="sha256-QTqD4rsMd+0L8L4QXVOdF+9F39mEoLE+zTsUqQE4OTg=" crossorigin="anonymous" {}
                script defer data-stream=(self.stream_url) { (PreEscaped(include_str!("../assets/player.js"))) }
            }
        }
    }
}
