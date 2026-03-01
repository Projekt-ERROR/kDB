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
    models::{
        CreateRecipe, CreateRecipeIngredient, CreateRecipeStep, Recipe, RecipeDetail,
        RecipeIngredient, RecipeStep, UpdateRecipe,
    },
    routes::ingredients,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/recipes", get(get_all).post(create))
        .route(
            "/api/recipes/{id}",
            get(get_one).delete(delete_one).put(update_one),
        )
        .route("/api/recipes/{id}/ingredients", put(replace_ingredients))
        .route("/api/recipes/{id}/steps", put(replace_steps))
        .route("/api/recipes/{id}/tags", put(replace_tags))
}

async fn replace_tags(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(tags): Json<Vec<String>>,
) -> impl IntoResponse {
    let mut tx = match state.db.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };

    if let Err(e) = sqlx::query!("DELETE FROM recipe_tags WHERE recipe_id = $1", id)
        .execute(&mut *tx)
        .await
    {
        let _ = tx.rollback().await;
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response();
    }

    for tag in tags {
        if let Err(e) = sqlx::query!(
            "INSERT INTO recipe_tags (recipe_id, tag) VALUES ($1, $2)",
            id,
            tag
        )
        .execute(&mut *tx)
        .await
        {
            let _ = tx.rollback().await;
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    }

    if let Err(e) = tx.commit().await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response();
    }

    StatusCode::NO_CONTENT.into_response()
}

async fn replace_steps(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(steps): Json<Vec<CreateRecipeStep>>,
) -> impl IntoResponse {
    let mut tx = match state.db.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };

    let step_numbers: Vec<i32> = steps.iter().filter_map(|i| i.step_number).collect();

    if let Err(e) = sqlx::query!(
        "DELETE FROM recipe_steps WHERE recipe_id = $1 AND NOT (step_number = ANY($2))",
        id,
        &step_numbers,
    )
    .execute(&mut *tx)
    .await
    {
        let _ = tx.rollback().await;
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response();
    }

    for step in steps {
        if let Err(e) = sqlx::query!(
            "INSERT INTO recipe_steps (recipe_id, step_number, instruction, timer_mins)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (recipe_id, step_number)
        DO UPDATE SET
            instruction = EXCLUDED.instruction,
            timer_mins = EXCLUDED.timer_mins",
            id,
            step.step_number,
            step.instruction,
            step.timer_mins,
        )
        .execute(&mut *tx)
        .await
        {
            let _ = tx.rollback().await;
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    }

    let _ = tx.commit().await;
    StatusCode::NO_CONTENT.into_response()
}

async fn replace_ingredients(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(ingredients): Json<Vec<CreateRecipeIngredient>>,
) -> impl IntoResponse {
    let mut tx = match state.db.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };
    let sort_orders: Vec<i32> = ingredients.iter().filter_map(|i| i.sort_order).collect();

    if let Err(e) = sqlx::query!(
        "DELETE FROM recipe_ingredients WHERE recipe_id = $1 AND NOT (sort_order = ANY($2))",
        id,
        &sort_orders,
    )
    .execute(&mut *tx)
    .await
    {
        let _ = tx.rollback().await;
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response();
    }

    for ing in ingredients {
        if let Err(e) = sqlx::query!(
            "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, custom_name, quantity, unit, preparation, optional, sort_order)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT (recipe_id, sort_order)
             DO UPDATE SET
                ingredient_id = EXCLUDED.ingredient_id,
                custom_name = EXCLUDED.custom_name,
                quantity = EXCLUDED.quantity,
                unit = EXCLUDED.unit,
                preparation = EXCLUDED.preparation,
                optional = EXCLUDED.optional",
            id,
            ing.ingredient_id,
            ing.custom_name,
            ing.quantity,
            ing.unit,
            ing.preparation,
            ing.optional.unwrap_or(false),
            ing.sort_order,
        )
        .execute(&mut *tx)
        .await
        {
            let _ = tx.rollback().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response();
        }
    }

    let _ = tx.commit().await;
    StatusCode::NO_CONTENT.into_response()
}

async fn delete_one(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM recipes WHERE id = $1", id)
        .execute(&state.db)
        .await;

    match result {
        Ok(r) if r.rows_affected() == 0 => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Recipe not found" })),
        )
            .into_response(),
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

// TODO
async fn update_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateRecipe>,
) -> impl IntoResponse {
    let result: Result<Option<Recipe>, sqlx::Error> = sqlx::query_as!(
        Recipe,
        "UPDATE recipes
        SET
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            servings = COALESCE($3, servings),
            prep_time_mins = COALESCE($4, prep_time_mins),
            cook_time_mins = COALESCE($5, cook_time_mins),
            difficulty = COALESCE($6, difficulty),
            cuisine = COALESCE($7, cuisine),
            meal_type = COALESCE($8, meal_type),
            source = COALESCE($9, source)
        WHERE id = $10
        RETURNING *",
        body.name,
        body.description,
        body.servings,
        body.prep_time_mins,
        body.cook_time_mins,
        body.difficulty,
        body.cuisine,
        body.meal_type,
        body.source,
        id
    )
    .fetch_optional(&state.db)
    .await;

    match result {
        Ok(Some(recipe)) => (StatusCode::OK, Json(recipe)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Recipe not found"})),
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
    Json(body): Json<CreateRecipe>,
) -> impl IntoResponse {
    let mut tx = match state.db.begin().await {
        Ok(tx) => tx,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let recipe: Recipe = match sqlx::query_as!(
        Recipe,
        "INSERT INTO recipes (name, description, servings, prep_time_mins, cook_time_mins, difficulty, cuisine, meal_type, source)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *",
        body.name,
        body.description,
        body.servings.unwrap_or(4),
        body.prep_time_mins,
        body.cook_time_mins,
        body.difficulty,
        body.cuisine,
        body.meal_type,
        body.source,
    )
    .fetch_one(&mut *tx)
    .await {
        Ok(r) => r,
        Err(e) => {
            let _ = tx.rollback().await;
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    };

    if let Some(ingredients) = body.ingredients {
        for ing in ingredients {
            if let Err(e) = sqlx::query!(
                "INSERT INTO recipe_ingredients (recipe_id, ingredient_id, custom_name, quantity, unit, preparation, optional, sort_order)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                recipe.id,
                ing.ingredient_id,
                ing.custom_name,
                ing.quantity,
                ing.unit,
                ing.preparation,
                ing.optional.unwrap_or(false),
                ing.sort_order,
            )
                .execute(&mut *tx)
                .await {
                    let _ = tx.rollback().await;
                    return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }
    }

    if let Some(steps) = body.steps {
        for step in steps {
            if let Err(e) = sqlx::query!(
                "INSERT INTO recipe_steps (recipe_id, step_number, instruction, timer_mins)
                 VALUES ($1, $2, $3, $4)",
                recipe.id,
                step.step_number,
                step.instruction,
                step.timer_mins,
            )
            .execute(&mut *tx)
            .await
            {
                let _ = tx.rollback().await;
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }
    }

    if let Some(tags) = body.tags {
        for tag in tags {
            if let Err(e) = sqlx::query!(
                "INSERT INTO recipe_tags (recipe_id, tag) VALUES ($1, $2)",
                recipe.id,
                tag,
            )
            .execute(&mut *tx)
            .await
            {
                let _ = tx.rollback().await;
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }
    }

    if let Err(e) = tx.commit().await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    (StatusCode::CREATED, Json(recipe)).0.into_response()
}

async fn get_one(State(state): State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let recipe: Recipe = match sqlx::query_as!(Recipe, "SELECT * FROM recipes WHERE id = $1", id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(r)) => r,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "Recipe not found" })),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };

    let ingredients = match sqlx::query_as!(
        RecipeIngredient,
        "SELECT * FROM recipe_ingredients WHERE recipe_id = $1 ORDER BY sort_order",
        id
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(i) => i,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };

    let steps = match sqlx::query_as!(
        RecipeStep,
        "SELECT * FROM recipe_steps WHERE recipe_id = $1",
        id
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };

    let tags = match sqlx::query_scalar!("SELECT tag FROM recipe_tags WHERE recipe_id = $1", id)
        .fetch_all(&state.db)
        .await
    {
        Ok(t) => t,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        Json(RecipeDetail {
            recipe,
            ingredients,
            steps,
            tags,
        }),
    )
        .into_response()
}

async fn get_all(State(state): State<AppState>) -> impl IntoResponse {
    let result: Result<Vec<Recipe>, sqlx::Error> =
        sqlx::query_as!(Recipe, "SELECT * FROM recipes ORDER BY name ASC")
            .fetch_all(&state.db)
            .await;

    match result {
        Ok(recipes) => (StatusCode::OK, Json(recipes)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
