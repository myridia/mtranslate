use axum::{extract::Query, response::IntoResponse, Json};
use std::collections::HashMap;
use std::process::Command;

pub async fn nslookup(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    // http://127.0.0.1:8089/nslookup?domain=myridia.com
    let mut v: Vec<&str> = vec![];
    let mut r = serde_json::json!(v);

    if params.contains_key("domain") {
        let domain = params["domain"].replace(" ", "");
        let cmd: &str = &format!("nslookup -type=txt {domain}");

        let output = Command::new("sh")
            .arg("-c")
            //.arg("nslookup -type=txt baeckerei-katz.de")
            .arg(cmd)
            .output()
            .expect("Failed to execute command");

        let text = String::from_utf8_lossy(output.stdout.as_slice());

        let parts = text.split("\n");
        for part in parts {
            v.push(part);
        }
        r = serde_json::json!(v);
    }

    Json(r)
}
