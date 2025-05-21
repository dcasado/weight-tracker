use std::collections::HashMap;
use std::fmt::Write;

use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::get,
    Router,
};
use chrono::{DateTime, Datelike, Duration, FixedOffset, Local, NaiveDate, NaiveTime, TimeZone};
use serde_json::json;

use crate::{
    app_state::AppState,
    domain::{
        measurement::{Measurement, MeasurementId, Weight},
        user::UserId,
    },
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
                NaiveTime::from_hms_milli_opt(23, 59, 59, 999)
                    .expect("manually set time should be valid"),
            )
            .unwrap()
            .into(),
    };

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
        None => end_date.with_time(NaiveTime::MIN).unwrap() - Duration::days(30),
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

    let min_weight: f64 = measurements
        .iter()
        .map(|m| Into::<f64>::into(m.weight.clone()))
        .fold(f64::MAX, f64::min);

    let max_weight: f64 = measurements
        .iter()
        .map(|m| Into::<f64>::into(m.weight.clone()))
        .fold(f64::MIN, f64::max);

    let last_weight: f64 = measurements
        .last()
        .unwrap_or(&Measurement {
            id: MeasurementId::new(0),
            user_id: UserId::new(0),
            date_time: Local::now().into(),
            weight: Weight::new(0.0).expect("Weight with value 0.0 must be valid"),
        })
        .weight
        .clone()
        .into();

    let slope: f64 = calculate_slope(measurements);
    let trend_emoji: &str = if slope > 0.0 { "↗️" } else { "↘️" };

    let user_id: i64 = user_id.into();
    let data = json!({
        "title": "Chart",
        "user_id": user_id,
        "start_date": start_date.date_naive(),
        "end_date": end_date.date_naive(),
        "dates": serde_json::to_string(&dates).map_err(|e| ApiError::Unexpected(Box::new(e)))?,
        "weights": serde_json::to_string(&weights).map_err(|e| ApiError::Unexpected(Box::new(e)))?,
        "alert_message": alert_message,
        "min_weight": min_weight,
        "max_weight": max_weight,
        "last_weight": last_weight,
        "trend": trend_emoji
    });

    let template = state
        .handlebars
        .render("chart", &data)
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Html(template))
}

fn calculate_slope(measurements: Vec<Measurement>) -> f64 {
    let n = measurements.len() as f64;

    let weights: Vec<f64> = measurements
        .iter()
        .map(|m| m.weight.clone().into())
        .collect();

    let timestamps: Vec<f64> = measurements
        .iter()
        .map(|m| m.date_time.num_days_from_ce() as f64)
        .collect();

    let sum_x: f64 = timestamps.iter().sum();
    let sum_x_square: f64 = timestamps.iter().map(|v| v * v).sum();
    let sum_y: f64 = weights.iter().sum();
    let mut sum_xy: f64 = 0.0;

    for i in 0..measurements.len() {
        sum_xy += weights[i] * timestamps[i];
    }

    (n * sum_xy - sum_x * sum_y) / (n * sum_x_square - (sum_x * sum_x))
}
