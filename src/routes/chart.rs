use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::get,
    Router,
};
use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use serde_json::json;

use crate::{
    app_state::AppState,
    domain::{measurement::Measurement, user::UserId},
    error::ApiError,
    repositories,
};

pub fn chart(state: AppState) -> Router {
    Router::new()
        .route("/chart/:user_id", get(render_chart))
        .with_state(state)
}

async fn render_chart(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Html<String>, ApiError> {
    let user_id: UserId = UserId::new(user_id);

    let user_id = repositories::users::find_user(&state.pool, &user_id)
        .await?
        .ok_or(ApiError::UserNotFound)?
        .id;

    let start_date: DateTime<FixedOffset> = match params.get("start-date") {
        Some(d) => {
            DateTime::<FixedOffset>::parse_from_rfc3339(d).map_err(|_| ApiError::InvalidDateTime)?
        }
        None => return Err(ApiError::MandatoryStartDate),
    };

    let end_date: DateTime<FixedOffset> = match params.get("end-date") {
        Some(d) => {
            DateTime::<FixedOffset>::parse_from_rfc3339(d).map_err(|_| ApiError::InvalidDateTime)?
        }
        None => return Err(ApiError::MandatoryEndDate),
    };

    #[derive(Serialize)]
    struct MeasurementResponse {
        id: i64,
        date_time: String,
        weight: f64,
    }

    let measurements: Vec<MeasurementResponse> =
        repositories::measurements::find_measurements_between_dates(
            &state.pool,
            &user_id,
            &start_date,
            &end_date,
        )
        .await?
        .into_iter()
        .map(|m: Measurement| MeasurementResponse {
            id: m.id.into(),
            date_time: m.date_time.to_rfc3339(),
            weight: m.weight.into(),
        })
        .collect();

    let user_id: i64 = user_id.into();
    let data = json!({
        "title": "Chart",
        "user_id": user_id,
        "start_date": start_date,
        "end_date": end_date,
        "measurements": serde_json::to_string(&measurements).map_err(|_| ApiError::Unknown)?
    });

    let template = state
        .handlebars
        .render("chart", &data)
        .map_err(|_| ApiError::Unknown)?;

    Ok(Html(template))
}
