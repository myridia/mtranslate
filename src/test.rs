use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Translated {
    target_value: String,
    target_hash: String,
    target_lang: String,
    source_lang: String,
    source_hash: String,
    request_hash: String,
    source_value: String,
    msg: String,
}

//#[axum_macros::debug_handler]
pub async fn test_get(x: u8) -> impl IntoResponse {
    let t = Translated {
        target_value: "".to_string(),
        target_hash: "".to_string(),
        target_lang: "".to_string(),
        source_lang: "".to_string(),
        source_hash: "".to_string(),
        request_hash: "".to_string(),
        source_value: "".to_string(),
        msg: x.to_string(),
    };

    Json(t)
}
