use axum::Router;
use axum::http::Method;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod models;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any)
        .allow_origin(Any);

    Router::new()
        .merge(routes::health::router())
        .merge(routes::ingredients::router())
        .merge(routes::kitchen::router())
        .layer(cors)
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("[X] - failed to connect to postgres");

    // start up ping
    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .expect("[X] - database unreachable");

    println!("[*] - connected to database");

    let state = AppState { db: pool };

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("kDB backend listening on {addr}");

    axum::serve(listener, build_router(state)).await.unwrap();
}
