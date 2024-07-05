use axum::{
    extract::{Path, State},
    response::Html,
    routing::get,
    Router,
};
use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use serde_json::json;

use crate::{app_state::AppState, domain::measurement::Measurement, error::ApiError, repositories};

pub fn table(state: AppState) -> Router {
    Router::new()
        .route("/table/:user_id", get(render_table))
        .with_state(state)
}

async fn render_table(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Html<String>, ApiError> {
    let user = repositories::users::find_user(&state.pool, user_id)
        .await?
        .ok_or(ApiError::UserNotFound)?;

    #[derive(Serialize)]
    struct MeasurementResponse {
        id: i32,
        date_time: DateTime<FixedOffset>,
        weight: f64,
    }

    let measurements: Vec<MeasurementResponse> =
        repositories::measurements::find_measurements(&state.pool, user.id.as_ref())
            .await?
            .into_iter()
            .map(|m: Measurement| MeasurementResponse {
                id: m.id.into(),
                date_time: m.date_time,
                weight: m.weight.into(),
            })
            .collect();

    let data = json!({
        "title": "Table",
        "measurements": measurements
    });

    let template = state
        .handlebars
        .render("table", &data)
        .map_err(|_| ApiError::Unknown)?;

    Ok(Html(template))
}
