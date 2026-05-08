pub mod admin;
pub mod partials;
pub mod player;

#[derive(Clone)]
pub struct TemplateRenderer {
    instance_name: String,
    stream_url: String,
}

impl TemplateRenderer {
    pub fn new(instance_name: String, stream_url: String) -> Self {
        Self {
            instance_name,
            stream_url,
        }
    }

    pub fn page(&self, title: Option<&str>, body: impl maud::Render) -> maud::Markup {
        maud::html! {
            (maud::DOCTYPE)

            html lang="en" {
                head {
                    meta charset="UTF-8";
                    meta name="viewport" content="width=device-width; initial-scale=1.0";

                    title {
                        @if let Some(title) = title {
                            (format!("{} - {}", title, self.instance_name))
                        } @else {
                            (format!("Campus Playout Manager - {}", self.instance_name))
                        }
                    }

                    link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.8/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-sRIl4kxILFvY47J16cr9ZwB07vP4J8+LH7qKQnuqkuIAvNWLzeN8tE5YBujZqJLB" crossorigin="anonymous";
                    style { (maud::PreEscaped(include_str!("../assets/style.css"))) }
                }

                body hx-ext="morph" {
                    nav.navbar.navbar-expand-lg.bg-body-tertiary {
                        .container-fluid {
                            a.navbar-brand href="https://ury.org.uk" {
                                "(( URY ))"
                            }
                            button.navbar-toggler type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation" {
                                span.navbar-toggler-icon;
                            }
                            .collapse.navbar-collapse #navbarSupportedContent {
                                ul.navbar-nav.me-auto.mb-2.mb-lg-0 {
                                    li.nav-item {
                                        a.nav-link href="/" {
                                            "Home"
                                        }
                                    }
                                    li.nav-item {
                                        a.nav-link href="https://ury.org.uk/live" {
                                            "URY Live"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    .container.mt-4 {
                        h1 {
                            "Campus Playout - " (self.instance_name)
                        }

                        (body)
                    }

                    script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.8/dist/js/bootstrap.bundle.min.js" integrity="sha384-FKyoEForCGlyvwx9Hj09JcYn3nv7wiPVlz7YYwJrWVcXK/BmnVDxM+D2scQbITxI" crossorigin="anonymous" {}
                    script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.8/dist/htmx.min.js" integrity="sha256-Iig+9oy3VFkU8KiKG97cclanA9HVgMHSVSF9ClDTExM=" crossorigin="anonymous" {}
                    script src="https://cdn.jsdelivr.net/npm/idiomorph@0.7.4/dist/idiomorph-ext.min.js" integrity="sha256-pkN+VbG2oHvEIfDSMCZqOTmbaCbG7Rng7ZxjtwdESl8=" crossorigin="anonymous" {}
                }
            }
        }
    }
}

pub fn error_page(title: &str, body: &[impl maud::Render]) -> maud::Markup {
    // TODO: a better way than this?
    let renderer = TemplateRenderer::new("".to_string(), "".to_string());
    renderer.page(
        Some("Error - Campus Playout Manager"),
        maud::html! {
            h1.govuk-heading-l {
                (title)
            }
            @for para in body {
                p {
                    (para)
                }
            }
        },
    )
}
