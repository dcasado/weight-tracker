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
use crate::domain::user::UserId;
use crate::domain::weight::{Kilograms, Weight, WeightId};
use crate::error::ApiError;
use crate::repositories;

#[derive(Deserialize)]
struct PostWeight {
    user_id: i64,
    measured_at: String,
    kilograms: f64,
}

#[derive(Serialize)]
struct WeightResponse {
    weight_id: i64,
    measured_at: String,
    kilograms: f64,
}

pub fn measurements(state: AppState) -> Router {
    Router::new()
        .route("/measurements/weights", get(get_weights).post(add_weight))
        .route("/measurements/weights/{weight_id}", delete(delete_weight))
        .with_state(state)
}

async fn add_weight(
    State(state): State<AppState>,
    Json(body): Json<PostWeight>,
) -> Result<StatusCode, ApiError> {
    let user_id: UserId = UserId::new(body.user_id);

    let user_id = repositories::users::find_user(&state.pool, &user_id)
        .await?
        .ok_or(ApiError::UserNotFound)?
        .id;

    let measured_at = body
        .measured_at
        .parse::<DateTime<FixedOffset>>()
        .map_err(|_| ApiError::InvalidDateTime)?;

    let kilograms = Kilograms::try_from(body.kilograms)?;

    repositories::measurements::insert_weight(&state.pool, &user_id, &measured_at, &kilograms)
        .await?;

    Ok(StatusCode::CREATED)
}

async fn get_weights(
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

    let weights: Vec<WeightResponse> = repositories::measurements::find_weights_between_dates(
        &state.pool,
        &user_id,
        &start_date,
        &end_date,
    )
    .await?
    .into_iter()
    .map(|w: Weight| WeightResponse {
        weight_id: w.weight_id.into(),
        measured_at: DateTime::<Local>::from(w.measured_at).to_rfc3339(),
        kilograms: w.kilograms.into(),
    })
    .collect();

    if let Some(accept_encoding_header) = headers.get(ACCEPT) {
        match accept_encoding_header.to_str().unwrap() {
            "text/csv" => {
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, "text/csv")
                    .header(CONTENT_DISPOSITION, "attachment; filename=\"weights.csv\"")
                    .body(generate_weights_csv(weights))
                    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

                Ok(response)
            }
            "application/json" => {
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, "application/json")
                    .body(json!(weights).to_string())
                    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

                Ok(response)
            }
            _ => Err(ApiError::UnsupportedMediaType),
        }
    } else {
        Err(ApiError::UnsupportedMediaType)
    }
}

fn generate_weights_csv(weights: Vec<WeightResponse>) -> String {
    weights.iter().fold(
        "weight_id,measured_at,kilograms".to_string(),
        |mut acc, weight| {
            let row = format!(
                "\n{},{},{}",
                weight.weight_id, weight.measured_at, weight.kilograms
            );
            acc.push_str(row.as_str());
            acc
        },
    )
}

async fn delete_weight(
    State(state): State<AppState>,
    Path(weight_id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    let weight_id = WeightId::new(weight_id);

    repositories::measurements::delete_weight(&state.pool, &weight_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
