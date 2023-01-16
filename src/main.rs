use axum::{
    http::StatusCode,
    http::{HeaderValue, Method},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use bcrypt::{hash, verify};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // Routers
    let app = Router::new()
        .route("/echo", get(echo))
        .route("/auth/connect", post(create_user))
        .route("/auth/handshake", post(verify_handshake))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:3500".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        );

    // App
    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/* CONTROLLERS */

async fn echo() -> impl IntoResponse {
    let mut rng = rand::thread_rng();
    let value: u64 = rng.gen_range(1..10);
    let res = TrackNet { data: value };
    return (StatusCode::OK, Json(res));
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1,
        username: payload.username,
        password: hash_password(&payload.password),
    };

    return (StatusCode::CREATED, Json(user));
}

async fn verify_handshake(Json(payload): Json<VerifyUser>) -> impl IntoResponse {
    let hashed = hash_password(&payload.password);
    let status: String;
    if verify_password(&hashed, &payload.password) {
        status = String::from("GOOD");
    } else {
        status = String::from("BAD");
    }

    let user = User {
        id: 1,
        username: payload.username,
        password: String::from(status),
    };

    return (StatusCode::CREATED, Json(user));
}

/* TYPES */

#[derive(Serialize)]
struct TrackNet {
    data: u64,
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct VerifyUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
    password: String,
}

/* UTILS */

fn hash_password(s: &String) -> String {
    return hash(&s, 4).unwrap();
}

fn verify_password(hashed_str: &str, s: &str) -> bool {
    return verify(s, &hashed_str).unwrap();
}
