use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ------------- INGREDIENTS -------------
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
    pub category: Option<String>,
    pub default_unit: Option<String>,
    pub shelf_life_days: Option<i32>,
    pub storage: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateIngredient {
    pub name: String,
    pub category: Option<String>,
    pub default_unit: Option<String>,
    pub shelf_life_days: Option<i32>,
    pub storage: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateIngredient {
    pub name: Option<String>,
    pub category: Option<String>,
    pub default_unit: Option<String>,
    pub shelf_life_days: Option<i32>,
    pub storage: Option<String>,
}

// --------- KITCHEN ---------
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct KitchenEntry {
    pub id: Uuid,
    pub ingredient_id: Uuid,
    pub quantity: f64,
    pub unit: String,
    pub purchased_on: NaiveDate,
    pub expires_on: Option<NaiveDate>,
    pub opened: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateKitchenEntry {
    pub ingredient_id: Uuid,
    pub quantity: f64,
    pub unit: String,
    pub purchased_on: Option<NaiveDate>,
    pub expires_on: Option<NaiveDate>,
    pub opened: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateKitchenEntry {
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub purchased_on: Option<NaiveDate>,
    pub expires_on: Option<NaiveDate>,
    pub opened: Option<bool>,
}

// --------- RECIPES ---------
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub servings: i32,
    pub prep_time_mins: Option<i32>,
    pub cook_time_mins: Option<i32>,
    pub total_time_mins: Option<i32>,
    pub difficulty: Option<String>,
    pub cuisine: Option<String>,
    pub meal_type: Option<String>,
    pub source: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdateRecipe {
    pub name: Option<String>,
    pub description: Option<String>,
    pub servings: Option<i32>,
    pub prep_time_mins: Option<i32>,
    pub cook_time_mins: Option<i32>,
    pub difficulty: Option<String>,
    pub cuisine: Option<String>,
    pub meal_type: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecipeIngredient {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub ingredient_id: Option<Uuid>,
    pub custom_name: Option<String>,
    pub quantity: sqlx::types::Decimal,
    pub unit: Option<String>,
    pub preparation: Option<String>,
    pub optional: Option<bool>,
    pub sort_order: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RecipeStep {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub step_number: i32,
    pub instruction: String,
    pub timer_mins: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct RecipeDetail {
    #[serde(flatten)]
    pub recipe: Recipe,
    pub ingredients: Vec<RecipeIngredient>,
    pub steps: Vec<RecipeStep>,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecipe {
    pub name: String,
    pub description: Option<String>,
    pub servings: Option<i32>,
    pub prep_time_mins: Option<i32>,
    pub cook_time_mins: Option<i32>,
    pub difficulty: Option<String>,
    pub cuisine: Option<String>,
    pub meal_type: Option<String>,
    pub source: Option<String>,
    pub ingredients: Option<Vec<CreateRecipeIngredient>>,
    pub steps: Option<Vec<CreateRecipeStep>>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecipeIngredient {
    pub ingredient_id: Option<Uuid>,
    pub custom_name: Option<String>,
    pub quantity: Option<sqlx::types::Decimal>,
    pub unit: Option<String>,
    pub preparation: Option<String>,
    pub optional: Option<bool>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRecipeStep {
    pub step_number: Option<i32>,
    pub instruction: String,
    pub timer_mins: Option<i32>,
}

// --------- MEAL PLAN ---------
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MealPlan {
    pub id: Uuid,
    pub plan_date: chrono::NaiveDate,
    pub meal_slot: String,
    pub recipe_id: Option<Uuid>,
    pub custom_meal: Option<String>,
    pub notes: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMealPlan {
    pub plan_date: chrono::NaiveDate,
    pub meal_slot: String,
    pub recipe_id: Option<Uuid>,
    pub custom_meal: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMealPlan {
    pub plan_date: Option<chrono::NaiveDate>,
    pub meal_slot: Option<String>,
    pub recipe_id: Option<Uuid>,
    pub custom_meal: Option<String>,
    pub notes: Option<String>,
}
