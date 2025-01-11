use std::collections::HashMap;

use axum::extract::{Path, Query};
use axum::http::header::{ACCEPT, CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::http::{HeaderMap, Response, StatusCode};
use axum::routing::{delete, get};
use axum::Router;
use axum::{extract::State, Json};
use chrono::{DateTime, FixedOffset, Local};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::app_state::AppState;
use crate::domain::measurement::{Measurement, MeasurementId, Weight};
use crate::domain::user::UserId;
use crate::error::ApiError;
use crate::repositories;

#[derive(Deserialize)]
struct PostMeasurement {
    user_id: i64,
    date_time: String,
    weight: f64,
}

#[derive(Serialize)]
struct MeasurementResponse {
    id: i64,
    date_time: String,
    weight: f64,
}

pub fn measurements(state: AppState) -> Router {
    Router::new()
        .route("/measurements", get(get_measurements).post(add_measurement))
        .route("/measurements/{id}", delete(delete_measurement))
        .with_state(state)
}

async fn add_measurement(
    State(state): State<AppState>,
    Json(body): Json<PostMeasurement>,
) -> Result<StatusCode, ApiError> {
    let user_id: UserId = UserId::new(body.user_id);

    let user_id = repositories::users::find_user(&state.pool, &user_id)
        .await?
        .ok_or(ApiError::UserNotFound)?
        .id;

    let date_time = body
        .date_time
        .parse::<DateTime<FixedOffset>>()
        .map_err(|_| ApiError::InvalidDateTime)?;

    let weight = Weight::try_from(body.weight)?;

    repositories::measurements::insert_measurement(&state.pool, &user_id, &date_time, &weight)
        .await?;

    Ok(StatusCode::CREATED)
}

async fn get_measurements(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Result<Response<String>, ApiError> {
    let user_id: UserId = UserId::new(match params.get("user_id") {
        Some(id) => id.parse().map_err(|_| ApiError::InvalidUserId)?,
        None => return Err(ApiError::MandatoryUserId),
    });

    let start_date: DateTime<FixedOffset> = match params.get("start_date") {
        Some(d) => {
            DateTime::<FixedOffset>::parse_from_rfc3339(d).map_err(|_| ApiError::InvalidDateTime)?
        }
        None => return Err(ApiError::MandatoryStartDate),
    };

    let end_date: DateTime<FixedOffset> = match params.get("end_date") {
        Some(d) => {
            DateTime::<FixedOffset>::parse_from_rfc3339(d).map_err(|_| ApiError::InvalidDateTime)?
        }
        None => return Err(ApiError::MandatoryEndDate),
    };

    let user_id = repositories::users::find_user(&state.pool, &user_id)
        .await?
        .ok_or(ApiError::UserNotFound)?
        .id;

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
            date_time: DateTime::<Local>::from(m.date_time).to_rfc3339(),
            weight: m.weight.into(),
        })
        .collect();

    if let Some(accept_encoding_header) = headers.get(ACCEPT) {
        match accept_encoding_header.to_str().unwrap() {
            "text/csv" => {
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, "text/csv")
                    .header(
                        CONTENT_DISPOSITION,
                        "attachment; filename=\"measurements.csv\"",
                    )
                    .body(generate_csv(measurements))
                    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

                Ok(response)
            }
            "application/json" => {
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, "application/json")
                    .body(json!(measurements).to_string())
                    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

                Ok(response)
            }
            _ => Err(ApiError::UnsupportedMediaType),
        }
    } else {
        Err(ApiError::UnsupportedMediaType)
    }
}

fn generate_csv(measurements: Vec<MeasurementResponse>) -> String {
    measurements
        .iter()
        .fold("id,date_time,weight".to_string(), |mut acc, measurement| {
            let row = format!(
                "\n{},{},{}",
                measurement.id, measurement.date_time, measurement.weight
            );
            acc.push_str(row.as_str());
            acc
        })
}

async fn delete_measurement(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    let id = MeasurementId::new(id);

    repositories::measurements::delete_measurement(&state.pool, &id).await?;

    Ok(StatusCode::NO_CONTENT)
}
