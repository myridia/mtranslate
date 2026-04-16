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

pub async fn ftl() -> impl IntoResponse {
    // http://127.0.0.1:8889/ftl
    let ftl: Vec<&str> = env!("ftl").split(',').collect();
    let r = serde_json::json!(
        {
            "api": "ftl",
            "ftl": ftl,
        }
    );
    Json(r)
}

pub async fn codes() -> impl IntoResponse {
    // http://127.0.0.1:8889/codes
    let codes: Vec<&str> = env!("codes").split(',').collect();
    let r = serde_json::json!(
        {
            "api": "codes",
            "codes": codes,

        }
    );
    Json(r)
}
