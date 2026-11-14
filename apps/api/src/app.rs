mod routes;

use axum::Router;
use sqlx::PgPool;

pub fn create_app(pool: PgPool) -> Router {
    Router::new().merge(routes::router()).with_state(pool)
}
