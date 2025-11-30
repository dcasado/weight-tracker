use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    UserNotFound,
    MandatoryUserId,
    MandatoryStartDate,
    MandatoryEndDate,
    InvalidUserId,
    InvalidDateTime,
    NegativeWeight,
    WeightNotFound,
    ImpedanceNotFound,
    UnsupportedMediaType,

    Unexpected(Box<dyn std::error::Error>),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            Self::MandatoryUserId => (
                StatusCode::BAD_REQUEST,
                "Query parameter user_id is mandatory".to_string(),
            ),
            Self::MandatoryStartDate => (
                StatusCode::BAD_REQUEST,
                "Query parameter start_date is mandatory".to_string(),
            ),
            Self::MandatoryEndDate => (
                StatusCode::BAD_REQUEST,
                "Query parameter end_date is mandatory".to_string(),
            ),
            Self::InvalidUserId => (StatusCode::BAD_REQUEST, "user_id must be valid".to_string()),
            Self::InvalidDateTime => (
                StatusCode::BAD_REQUEST,
                "date_time must be a valid date".to_string(),
            ),
            Self::NegativeWeight => (
                StatusCode::BAD_REQUEST,
                "Weight cannot be negative".to_string(),
            ),
            Self::WeightNotFound => (StatusCode::NOT_FOUND, "Weight not found".to_string()),
            Self::ImpedanceNotFound => (StatusCode::NOT_FOUND, "Impedance not found".to_string()),
            Self::UnsupportedMediaType => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "media type not supported".to_string(),
            ),
            Self::Unexpected(error) => {
                println!("Unexpected error ocurred. {}", error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };
        (status, Json(json!({"message": err_msg }))).into_response()
    }
}
