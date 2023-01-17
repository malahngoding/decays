use rand::prelude::*;

pub async fn route_info() -> axum::Json<serde_json::Value> {
    let mut rng = rand::thread_rng();
    let value: u64 = rng.gen_range(1..10);
    axum::Json(serde_json::json!({ "random": value }))
}
