use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};

use uuid::Uuid;

use crate::{
    AppState,
    models::{CreateKitchenEntry, Ingredient, KitchenEntry, UpdateKitchenEntry},
};

// router

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/kitchen", get(get_all).post(create))
        .route(
            "/api/kitchen/{id}",
            get(get_one).delete(delete_one).put(update_one),
        )
}

async fn update_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateKitchenEntry>,
) -> impl IntoResponse {
    let result: Result<Option<KitchenEntry>, sqlx::Error> = sqlx::query_as!(
        KitchenEntry,
        "UPDATE kitchen
        SET
            quantity = COALESCE($1, quantity),
            unit = COALESCE($2, unit),
            purchased_on = COALESCE($3, purchased_on),
            expires_on = COALESCE($4, expires_on),
            opened = COALESCE($5, opened)
        WHERE id = $6
        RETURNING *",
        body.quantity,
        body.unit,
        body.purchased_on,
        body.expires_on,
        body.opened,
        id
    )
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(entry)) => (StatusCode::OK, Json(entry)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Ingredient not found"})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn delete_one(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM kitchen WHERE id = ($1) RETURNING id", id)
        .fetch_optional(&state.db)
        .await;

    match result {
        Ok(Some(_)) => StatusCode::NO_CONTENT.into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Entry not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateKitchenEntry>,
) -> impl IntoResponse {
    let purchased_on = body
        .purchased_on
        .unwrap_or_else(|| chrono::Local::now().date_naive());
    let opened = body.opened.unwrap_or(false);
    let expires_on = match body.expires_on {
        Some(date) => Some(date),
        None => {
            let ingredient = sqlx::query_as!(
                Ingredient,
                "SELECT * FROM ingredients WHERE id = ($1)",
                body.ingredient_id
            )
            .fetch_optional(&state.db)
            .await;

            match ingredient {
                Ok(Some(ing)) => ing
                    .shelf_life_days
                    .map(|days| purchased_on + chrono::Duration::days(days as i64)),
                _ => None,
            }
        }
    };

    let result: Result<KitchenEntry, sqlx::Error> = sqlx::query_as!(
        KitchenEntry,
        "INSERT INTO kitchen (ingredient_id, quantity, unit, purchased_on, expires_on, opened)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *",
        body.ingredient_id,
        body.quantity,
        body.unit,
        purchased_on,
        expires_on,
        opened,
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(entry) => (StatusCode::CREATED, Json(entry)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn get_one(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = sqlx::query_as!(KitchenEntry, "SELECT * FROM kitchen WHERE id = ($1)", id)
        .fetch_optional(&state.db)
        .await;

    match result {
        Ok(Some(entry)) => (StatusCode::OK, Json(entry)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Entry not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn get_all(State(state): State<AppState>) -> impl IntoResponse {
    let result: Result<Vec<KitchenEntry>, sqlx::Error> = sqlx::query_as!(
        KitchenEntry,
        "SELECT * FROM kitchen ORDER BY created_at ASC"
    )
    .fetch_all(&state.db)
    .await;

    match result {
        Ok(entries) => (StatusCode::OK, Json(entries)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
