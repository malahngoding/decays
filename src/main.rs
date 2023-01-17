use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod error;
mod models;
mod utils;

static KEYS: Lazy<models::auth::Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "Your secret here".to_owned());
    models::auth::Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    // let durl = std::env::var("DATABASE_URL").expect("set DATABASE_URL env variable");
    let durl = "postgresql://postgres:@localhost:5432/instead";
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::new().allow_origin(Any);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await
        .expect("unable to connect to database");

    let app = Router::new()
        // Public Route
        .route("/echo", get(controllers::info::route_info))
        .route("/auth/handshake", post(controllers::auth::login))
        .route("/auth/connect", post(controllers::auth::register))
        // Protected
        .route("/user/profile", post(controllers::user::user_profile))
        .layer(cors)
        .layer(Extension(pool));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 5000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
