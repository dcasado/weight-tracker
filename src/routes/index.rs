use axum::{extract::State, response::Html, routing::get, Router};
use serde::Serialize;
use serde_json::json;

use crate::{app_state::AppState, error::ApiError, repositories};

pub fn index(state: AppState) -> Router {
    Router::new()
        .route("/", get(render_index))
        .with_state(state)
}

async fn render_index(State(state): State<AppState>) -> Result<Html<String>, ApiError> {
    #[derive(Serialize)]
    struct UserResponse {
        id: i64,
        name: String,
    }

    let users: Vec<UserResponse> = repositories::users::find_users(&state.pool)
        .await?
        .into_iter()
        .map(|u| UserResponse {
            id: u.id.into(),
            name: u.name.into(),
        })
        .collect();

    let data = json!({
        "title": "Index",
        "users": users
    });

    let template = state
        .handlebars
        .render("index", &data)
        .map_err(|_| ApiError::Unknown)?;

    Ok(Html(template))
}
