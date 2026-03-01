use std::result;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
};

use uuid::Uuid;

use crate::{
    AppState,
    models::{CreateMealPlan, MealPlan, UpdateMealPlan},
    routes::meal_plan,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/meal-plan", get(get_all).post(create))
        .route(
            "/api/meal-plan/{id}",
            get(get_one).delete(delete_one).put(update_one),
        )
}

async fn update_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateMealPlan>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        MealPlan,
        "UPDATE meal_plan
        SET
            plan_date = COALESCE($1, plan_date),
            meal_slot = COALESCE($2, meal_slot),
            recipe_id = COALESCE($3, recipe_id),
            custom_meal = COALESCE($4, custom_meal),
            notes = COALESCE($5, notes)
        WHERE id = $6
        RETURNING *",
        body.plan_date,
        body.meal_slot,
        body.recipe_id,
        body.custom_meal,
        body.notes,
        id
    )
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(meal_plan)) => (StatusCode::OK, Json(meal_plan)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Meal plan not found"})),
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
    let result = sqlx::query!("DELETE FROM meal_plan WHERE id = ($1) RETURNING id", id)
        .fetch_optional(&state.db)
        .await;

    match result {
        Ok(Some(_)) => StatusCode::NO_CONTENT.into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Meal plan not found" })),
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
    Json(body): Json<CreateMealPlan>,
) -> impl IntoResponse {
    let result: Result<MealPlan, sqlx::Error> = sqlx::query_as!(
        MealPlan,
        "INSERT INTO meal_plan (plan_date, meal_slot, recipe_id, custom_meal, notes)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *",
        body.plan_date,
        body.meal_slot,
        body.recipe_id,
        body.custom_meal,
        body.notes,
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(meal_plan) => (StatusCode::CREATED, Json(meal_plan)).0.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
async fn get_one(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = sqlx::query_as!(MealPlan, "SELECT * FROM meal_plan WHERE id = $1", id,)
        .fetch_optional(&state.db)
        .await;

    match result {
        Ok(Some(meal_plan)) => (StatusCode::OK, Json(meal_plan)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Meal plan not found" })),
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
    let result: Result<Vec<MealPlan>, sqlx::Error> =
        sqlx::query_as!(MealPlan, "SELECT * FROM meal_plan ORDER BY plan_date")
            .fetch_all(&state.db)
            .await;

    match result {
        Ok(meal_plan) => (StatusCode::OK, Json(meal_plan)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
