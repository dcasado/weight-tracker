use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::{delete, get};
use axum::Router;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::app_state::AppState;
use crate::domain::user::{UserId, UserName};
use crate::error::ApiError;
use crate::repositories;

#[derive(Deserialize)]
struct PostUser {
    name: String,
}

pub fn users(state: AppState) -> Router {
    Router::new()
        .route("/users", get(get_users).post(add_user))
        .route("/users/:id", delete(delete_user))
        .with_state(state)
}

async fn get_users(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
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

    Ok(Json(json!(users)))
}

async fn add_user(
    State(state): State<AppState>,
    Json(body): Json<PostUser>,
) -> Result<StatusCode, ApiError> {
    let name: UserName = UserName::new(body.name);

    repositories::users::insert_user(&state.pool, &name).await?;

    Ok(StatusCode::CREATED)
}

async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    let id: UserId = UserId::new(id);

    repositories::users::delete_user(&state.pool, &id).await?;

    Ok(StatusCode::NO_CONTENT)
}
