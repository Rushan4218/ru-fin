use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{Value, json};
use sqlx::PgPool;

async fn health_check(State(pool): State<PgPool>) -> Result<Json<Value>, StatusCode> {
    let result = sqlx::query("SELECT 1").execute(&pool).await;

    match result {
        Ok(_) => Ok(Json(json!({
            "status": "ok",
            "message": "ru-fin API is running",
            "database": "connected"
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn router() -> Router<PgPool> {
    Router::new().route("/health", get(health_check))
}
