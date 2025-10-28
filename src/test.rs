use axum::{extract::Query, response::IntoResponse, Json};

pub async fn test() -> impl IntoResponse {
    let r = serde_json::json!([
        {
            "test": "OK",
        }
    ]);
    Json(r)
}
