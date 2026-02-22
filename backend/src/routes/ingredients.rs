use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};

use uuid::Uuid;

use crate::{
    AppState,
    models::{CreateIngredient, Ingredient, UpdateIngredient},
    routes::ingredients,
};

// router

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/ingredients", get(get_all).post(create))
        .route(
            "/api/ingredients/{id}",
            get(get_one).delete(delete_one).put(update_one),
        )
}

async fn update_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateIngredient>,
) -> impl IntoResponse {
    let result: Result<Option<Ingredient>, sqlx::Error> = sqlx::query_as!(
        Ingredient,
        "UPDATE ingredients
        SET
            name = COALESCE($1, name),
            category = COALESCE($2, category),
            default_unit = COALESCE($3, default_unit),
            shelf_life_days = COALESCE($4, shelf_life_days),
            storage = COALESCE($5, storage)
        WHERE id = $6
        RETURNING *",
        body.name,
        body.category,
        body.default_unit,
        body.shelf_life_days,
        body.storage,
        id
    )
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(ingredient)) => (StatusCode::OK, Json(ingredient)).into_response(),
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
    let result = sqlx::query!("DELETE FROM ingredients WHERE id = ($1) RETURNING id", id)
        .fetch_optional(&state.db)
        .await;

    match result {
        Ok(Some(_)) => StatusCode::NO_CONTENT.into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Ingredient not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn get_one(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = sqlx::query_as!(Ingredient, "SELECT * FROM ingredients WHERE id = ($1)", id)
        .fetch_optional(&state.db)
        .await;

    match result {
        Ok(Some(ingredient)) => (StatusCode::OK, Json(ingredient)).into_response(),
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

async fn get_all(State(state): State<AppState>) -> impl IntoResponse {
    let result: Result<Vec<Ingredient>, sqlx::Error> =
        sqlx::query_as!(Ingredient, "SELECT * FROM ingredients ORDER BY name ASC")
            .fetch_all(&state.db)
            .await;

    match result {
        Ok(ingredients) => (StatusCode::OK, Json(ingredients)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateIngredient>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Ingredient,
        "INSERT INTO ingredients (name, category, default_unit, shelf_life_days, storage)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *",
        body.name,
        body.category,
        body.default_unit,
        body.shelf_life_days,
        body.storage,
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(ingredient) => (StatusCode::CREATED, Json(ingredient)).into_response(),
        Err(sqlx::Error::Database(e)) if e.constraint() == Some("ingredients_name_key") => (
            StatusCode::CONFLICT,
            Json(serde_json::json!({ "error": "An ingredient with that name already exists" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error" : e.to_string() })),
        )
            .into_response(),
    }
}
