use axum::extract::State;
use axum::{response::Html, Router};
use serde_json::json;

use crate::app_state::AppState;
use crate::error::ApiError;

pub fn not_found(state: AppState) -> Router {
    Router::new().fallback(render_not_found).with_state(state)
}

async fn render_not_found(State(state): State<AppState>) -> Result<Html<String>, ApiError> {
    let data = json!({
        "title": "Not found",
    });

    let template = state
        .handlebars
        .render("not_found", &data)
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Html(template))
}
