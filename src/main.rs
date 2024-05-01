use std::net::Ipv4Addr;

use axum::Router;
use sqlx::pool::PoolOptions;
use sqlx::postgres::PgPool;
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

pub async fn get_connection_pool(database_url: &str) -> PgPool {
    PoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
        .expect("Failed to connect to the database")
}

pub fn get_database_url() -> String {
    dotenvy::var("DATABASE_URL").expect("env: DATABASE_URL should be set")
}

#[derive(Clone)]
pub struct AppState {
    pool: Pool<Postgres>,
}

impl AppState {
    fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
}

pub fn get_address() -> String {
    format!("{}:{}", Ipv4Addr::UNSPECIFIED.to_string(), 8000)
}

#[tokio::main]
async fn main() {
    let database_url = get_database_url();
    let pool = get_connection_pool(&database_url).await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    let state = AppState::new(pool);

    let api = routes().with_state(state);

    let address = get_address();

    let tcp_listener = TcpListener::bind(&address).await.unwrap();

    println!("server available at {}", &address);

    axum::serve(tcp_listener, api).await.unwrap();
}
