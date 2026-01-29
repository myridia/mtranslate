use crate::config::AppConfig;
use crate::translate::xtrans;

use axum::{Json, extract, response::IntoResponse};
use kuchiki::parse_html;
use kuchiki::traits::*;
use mysql::*;
use random_number::random;
use serde::{Deserialize, Serialize};
use tokio::task;

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

#[derive(Deserialize, Debug)]
pub struct Payload {
    //xemail: String,
    html: String,
    s: String,
    t: String,
}

//#[axum_macros::debug_handler]
pub async fn html(
    config: AppConfig,
    extract::Json(p): extract::Json<Payload>,
) -> impl IntoResponse {
    let wait: u64 = random!(config.wait_min, config.wait_max);

    let mut t = Translated {
        target_value: "".to_string(),
        target_hash: "".to_string(),
        target_lang: p.t.to_string(),
        source_lang: p.s.to_string(),
        source_hash: "".to_string(),
        request_hash: "".to_string(),
        source_value: p.html.to_string(),
        msg: "".to_string(),
    };

    let database_url: &str = &format!(
        "mysql://{0}:{1}@{2}:{3}/{4}",
        config.db_user, config.db_pass, config.db_host, config.db_port, config.db_name,
    );
    let pool = Pool::new(database_url).expect("Failed to create a connection pool");

    let target_value =
        match task::spawn_blocking(move || translatex(&pool, &p.s, &p.t, &p.html, wait)).await {
            Ok(v) => v, // the function returned successfully
            Err(e) => e.to_string(),
        };
    t.target_value = target_value;
    Json(t)
}

fn translatex(pool: &Pool, source_lang: &str, target_lang: &str, html: &str, wait: u64) -> String {
    let document = parse_html().one(html);

    for text_node in document.descendants().text_nodes() {
        let old_text = text_node.borrow().to_string();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let x = rt.block_on(xtrans(&pool, source_lang, target_lang, &old_text, wait));
        let new_text = x.target_value.to_string();
        text_node.replace(new_text);
    }

    let mut output = Vec::new();
    document.serialize(&mut output).unwrap();
    let new_html = String::from_utf8(output)
        .unwrap()
        .replace("<html><head></head><body>", "")
        .replace("</body></html>", "");
    return new_html;
}
