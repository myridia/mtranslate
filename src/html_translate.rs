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
    println!("...html");
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
    //println!("{}", html);

    let mut new_html = html.to_string();
    let document = parse_html().one(html);

    let mut has_textnode = false;
    for text_node in document.descendants().text_nodes() {
        let _old_text = text_node.borrow().to_string();
        let old_text = _old_text.trim().to_string();
        let mut new_text = old_text.clone();
        //println!("{}", is_numeric_and_symbols(&old_text));
        if old_text.len() > 3 && is_numeric_and_symbols(&old_text) == false {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let x = rt.block_on(xtrans(&pool, source_lang, target_lang, &old_text, wait));
            if _old_text.ends_with(" ") && _old_text.starts_with(" ") {
                new_text = format!(" {0} ", x.target_value);
            } else if _old_text.ends_with(" ") {
                new_text = format!("{0}", x.target_value);
            } else if _old_text.starts_with(" ") {
                new_text = format!(" {0}", x.target_value);
            } else {
                new_text = format!("{0}", x.target_value);
            }
            text_node.replace(new_text);
        } else {
            text_node.replace(" ".to_string());
            //println!("xxxxxxxxxxxxxxxx");
            //println!("{}", old_text.len());
            //println!("aaaa{}bbbb", old_text);
            //println!("xxxxxxxxxxxxxxxx");
        }

        has_textnode = true;
    }

    if has_textnode {
        let mut output = Vec::new();
        document.serialize(&mut output).unwrap();
        new_html = String::from_utf8(output)
            .unwrap()
            .replace("<html><head></head><body>", "")
            .replace("</body></html>", "");
    }
    //println!("{}", new_html);
    return new_html;
}

fn is_numeric_and_symbols(s: &str) -> bool {
    if s.is_empty() {
        return false; // Or true, depending on whether you consider an empty string valid
    }

    for c in s.chars() {
        if !c.is_numeric() && !is_special_symbol(c) {
            return false;
        }
    }

    true
}

fn is_special_symbol(c: char) -> bool {
    match c {
        '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '-' | '_' | '+' | '=' | '['
        | ']' | '{' | '}' | ';' | ':' | '\'' | '"' | '\\' | '|' | ',' | '.' | '/' | '<' | '>'
        | '?' | '`' | '~' => true,
        _ => false,
    }
}
