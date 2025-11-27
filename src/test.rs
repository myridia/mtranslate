use axum::{Json, response::IntoResponse};

pub async fn test(x: u8) -> impl IntoResponse {
    println!("{:?}", x);
    let r = serde_json::json!([
        {
            "test": "OK",
        }
    ]);
    Json(r)
}
