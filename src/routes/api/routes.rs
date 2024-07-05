use axum::Router;

use crate::app_state::AppState;

use super::{measurements, users};

pub fn api(state: AppState) -> Router {
    Router::new()
        .merge(users::users(state.clone()))
        .merge(measurements::measurements(state.clone()))
}
