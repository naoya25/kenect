use axum::{Extension, Router, routing::get};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL が設定されていません");
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("DB接続に失敗しました");

    let app = Router::new()
        .route("/health", get(health))
        .layer(Extension(pool))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("http://localhost:3000 で起動しました");
    axum::serve(listener, app).await.unwrap();
}

async fn health(Extension(pool): Extension<SqlitePool>) -> String {
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();
    format!("ok (users: {})", row.0)
}
