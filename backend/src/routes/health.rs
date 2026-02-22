use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
    database: bool,
}

async fn health_handler(State(state): State<AppState>) -> impl IntoResponse {
    let db_ok = sqlx::query("SELECT 1").execute(&state.db).await.is_ok();

    let body = HealthResponse {
        status: "Ok",
        version: env!("CARGO_PKG_VERSION"),
        database: db_ok,
    };

    (StatusCode::OK, Json(body))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_handler))
}
