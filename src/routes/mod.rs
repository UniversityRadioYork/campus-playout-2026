use axum::Router;

use crate::state::AppState;

mod admin;
mod internal;
mod stats;
mod status;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .merge(admin::routes())
        .nest("/status", status::routes())
        .nest("/stats", stats::routes())
        .nest("/__internal", internal::routes())
        .with_state(state)
}
