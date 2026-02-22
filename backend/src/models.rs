use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

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
