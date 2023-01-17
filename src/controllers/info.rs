pub async fn route_info() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "routes": ["/echo", "/auth/connect", "/auth/handshake", "/user/profile"],
        "routes_info": {
            "/echo" : "this route",
            "/auth/connect": "register a user with email and password",
            "/auth/handshake": "login with the credentials used for registering",
            "/user/profile": "view your user profile with the token recieved from /login"
        }
    }))
}
