mod health;

use axum::Router;
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new().nest("/api/v1", health::router())
}
