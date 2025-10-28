use axum::{extract::Query, response::IntoResponse, Json};
use deeptrans::{Engine, Translator};
use mysql::prelude::*;
use mysql::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug)]
struct Atrans {
    target_value: String,
    target_hash: String,
}
pub async fn hash8(s: &str) -> String {
    let result = Sha256::digest(s);
    let x = format!("{:x}", result).to_string();
    let hash = &x.get(x.len() - 8..);
    return hash.unwrap_or_default().to_string();
}

pub async fn help(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    // http://127.0.0.1:8889/help
    let codes: Vec<&str> = env!("codes").split(',').collect();
    let r = serde_json::json!([
        {
            "api": "help",
            "s": env!("codes"),
            "t": env!("codes"),
            "example" : "https://translate.myridia.com?s=en&t=de&v=hello"
        }
    ]);
    Json(r)
}
