use axum::{extract::Request, http::header::HeaderMap, response::IntoResponse, Json};
use deeptrans::{Engine, Translator};

//use axum_client_ip::XRealIp as ClientIp;
//use std::net::SocketAddr;

pub async fn test(headers: HeaderMap, req: Request) -> impl IntoResponse {
    // http://127.0.0.1:8889/test
    let trans = Translator::with_engine("en", "th", Engine::Google);
    let trans_r = trans.translate("hello").await;
    let r = serde_json::json!([
        {
            "trans_r": trans_r.unwrap(),
        }
    ]);
    Json(r)
}
