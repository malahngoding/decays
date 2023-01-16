use axum::{
    http::StatusCode,
    http::{HeaderValue, Method},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/echo", get(echo))
        .route("/users", post(create_user))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3500".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct TrackNet {
    data: u64,
}

async fn echo() -> impl IntoResponse {
    let res = TrackNet { data: 1337 };
    (StatusCode::OK, Json(res))
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1,
        username: payload.username,
        password: hash_password(payload.password),
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
    password: String,
}

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 16).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}

fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)
}
