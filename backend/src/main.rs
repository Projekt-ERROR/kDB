use axum::Router;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod models;
mod routes;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(routes::health::router())
        .merge(routes::ingredients::router())
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

    let state = AppState { db: pool };

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));

    let listener = TcpListener::bind(addr).await.unwrap();

    println!("kDB backend listening on {addr}");

    axum::serve(listener, build_router(state)).await.unwrap();
}
