use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use chrono::{DateTime, Local};
use serde::Serialize;
use serde_json::json;

use crate::{
    app_state::AppState,
    domain::{measurement::Measurement, user::UserId},
    error::ApiError,
    repositories,
};

pub fn table(state: AppState) -> Router {
    Router::new()
        .route("/table/:user_id", get(render_table))
        .with_state(state)
}

async fn render_table(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Html<String>, ApiError> {
    let user_id = UserId::new(user_id);

    let user_id = repositories::users::find_user(&state.pool, &user_id)
        .await?
        .ok_or(ApiError::UserNotFound)?
        .id;

    #[derive(Serialize)]
    struct MeasurementResponse {
        id: i64,
        date_time: String,
        weight: f64,
    }

    let measurements: Vec<MeasurementResponse> =
        repositories::measurements::find_measurements(&state.pool, &user_id)
            .await?
            .into_iter()
            .map(|m: Measurement| MeasurementResponse {
                id: m.id.into(),
                date_time: DateTime::<Local>::from(m.date_time)
                    .format("%Y-%m-%d %H:%M")
                    .to_string(),
                weight: m.weight.into(),
            })
            .collect();

    let user_id: i64 = user_id.into();
    let data = json!({
        "title": "Table",
        "measurements": measurements,
        "user_id": user_id
    });

    let template = state
        .handlebars
        .render("table", &data)
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Html(template))
}
