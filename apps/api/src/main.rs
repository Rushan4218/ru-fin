mod app;
mod db;

use std::path::Path;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");

    if env_path.exists() {
        dotenvy::from_path(env_path).expect("failed to load environment variables");
    }

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::create_pool(&database_url)
        .await
        .expect("failed to connect to database");

    println!("connected to PostgreSQL");

    let app = app::create_app(pool);

    let port = std::env::var("PORT")
        .expect("PORT must be set")
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    let address = format!("127.0.0.1:{port}");

    let listener = TcpListener::bind(&address)
        .await
        .expect("failed to bind address");

    println!("server running at http://{address}");

    axum::serve(listener, app).await.expect("server failed");
}
