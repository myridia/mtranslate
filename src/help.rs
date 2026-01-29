use axum::{Json, response::IntoResponse};

pub async fn help() -> impl IntoResponse {
    // http://127.0.0.1:8889/help
    let h: String = hostname::get().unwrap().into_string().unwrap();
    let r = serde_json::json!([
        {
            "api": "help",
            "s": env!("codes"),
            "t": env!("codes"),
            "example" : "https://translate.myridia.com?s=en&t=de&v=hello",
            "hostname": h
        }
    ]);
    Json(r)
}
