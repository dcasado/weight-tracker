use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
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
    Query(params): Query<HashMap<String, String>>,
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

    let years = repositories::measurements::find_years(&state.pool, &user_id).await?;
    let year: &str = params
        .get("year")
        .map(String::as_str)
        .unwrap_or(years.first().map(String::as_str).unwrap_or_default());

    let months =
        repositories::measurements::find_months_by_year(&state.pool, &user_id, year).await?;
    let month: &str = params
        .get("month")
        .map(String::as_str)
        .unwrap_or(months.first().map(String::as_str).unwrap_or_default());

    let measurements: Vec<MeasurementResponse> =
        repositories::measurements::find_measurements_by_year_month(
            &state.pool,
            &user_id,
            year,
            month,
        )
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
        "years": years,
        "year": year,
        "months": months,
        "month": month,
        "measurements": measurements,
        "user_id": user_id
    });

    let template = state
        .handlebars
        .render("table", &data)
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Html(template))
}
