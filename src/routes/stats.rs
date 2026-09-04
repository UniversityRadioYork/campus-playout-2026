use axum::{Router, extract::State, routing::get};
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

use crate::state::AppState;

async fn requests_stats(State(state): State<AppState>) -> crate::Result<maud::Markup> {
    let now = OffsetDateTime::now_utc();
    let now = PrimitiveDateTime::new(now.date(), now.time());
    let one_week = now - Duration::days(7);
    let stats = state.database.get_request_stats(one_week, now).await?;
    let mut stats = state.track_cache.resolve_request_stats(stats).await?;
    stats.sort_by_key(|s| -s.plays);
    Ok(state.template_renderer.request_stats(stats))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/requests", get(requests_stats))
}
