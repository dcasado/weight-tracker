use std::collections::HashMap;
use std::fmt::Write;

use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::get,
    Router,
};
use chrono::{DateTime, Duration, FixedOffset, Local, NaiveDate, NaiveTime, TimeZone};
use serde_json::json;

use crate::{
    app_state::AppState,
    domain::{measurement::Measurement, user::UserId},
    error::ApiError,
    repositories,
};

pub fn chart(state: AppState) -> Router {
    Router::new()
        .route("/chart/{user_id}", get(render_chart))
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
        Some(d) => Local::now()
            .timezone()
            .from_local_datetime(
                &NaiveDate::parse_from_str(d, "%Y-%m-%d")
                    .map_err(|_| ApiError::InvalidDateTime)?
                    .and_hms_opt(0, 0, 0)
                    .expect("manually set time should be valid"),
            )
            .unwrap()
            .into(),
        None => (Local::now()
            .with_time(NaiveTime::from_hms_opt(0, 0, 0).expect("manually set time should be valid"))
            .unwrap()
            - Duration::days(30))
        .into(),
    };

    let end_date: DateTime<FixedOffset> = match params.get("end-date") {
        Some(d) => Local::now()
            .timezone()
            .from_local_datetime(
                &NaiveDate::parse_from_str(d, "%Y-%m-%d")
                    .map_err(|_| ApiError::InvalidDateTime)?
                    .and_hms_milli_opt(23, 59, 59, 999)
                    .expect("manually set time should be valid"),
            )
            .unwrap()
            .into(),
        None => Local::now()
            .with_time(
                NaiveTime::from_hms_milli_opt(23, 59, 59, 999).expect("time should be valid"),
            )
            .unwrap()
            .into(),
    };

    let measurements: Vec<Measurement> =
        repositories::measurements::find_measurements_between_dates(
            &state.pool,
            &user_id,
            &start_date,
            &end_date,
        )
        .await?
        .into_iter()
        .collect();

    let mut dates: Vec<NaiveDate> = vec![];
    let mut weights: Vec<Option<f64>> = vec![];

    let mut current_date = start_date;
    let mut i = 0;
    while current_date < end_date {
        dates.push(current_date.date_naive());
        if i < measurements.len() {
            let measurement = measurements.get(i).unwrap();
            if current_date.date_naive() == measurement.date_time.date_naive() {
                let weight = &measurement.weight;
                weights.push(Some(weight.into()));
                i += 1;
            } else {
                weights.push(None);
            }
        } else {
            weights.push(None);
        }
        current_date += Duration::days(1);
    }

    let duplicate_measurements =
        repositories::measurements::find_duplicate_measurements(&state.pool, &user_id).await?;
    let mut alert_message = "".to_string();
    if !duplicate_measurements.is_empty() {
        alert_message = format!(
            "<p>There are duplicate measurements on the follwing dates.</p> <ul>{}</ul>",
            duplicate_measurements
                .into_iter()
                .fold(String::new(), |mut output, d| {
                    let _ = write!(output, "<li>{}</li>", d.0);
                    output
                })
        )
        .to_string();
    }

    let user_id: i64 = user_id.into();
    let data = json!({
        "title": "Chart",
        "user_id": user_id,
        "start_date": start_date.date_naive(),
        "end_date": end_date.date_naive(),
        "dates": serde_json::to_string(&dates).map_err(|e| ApiError::Unexpected(Box::new(e)))?,
        "weights": serde_json::to_string(&weights).map_err(|e| ApiError::Unexpected(Box::new(e)))?,
        "alert_message": alert_message
    });

    let template = state
        .handlebars
        .render("chart", &data)
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Html(template))
}
