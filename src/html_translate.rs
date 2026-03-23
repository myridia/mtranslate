use crate::config::AppConfig;
use crate::translate::xtrans;

use axum::{Json, extract, response::IntoResponse};
//use kuchiki::parse_html;
use kuchiki::traits::*;
use kuchiki::*;
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
fn get_class_names(n: NodeRef) -> Option<String> {
    n.as_element()
        .and_then(|elem| elem.attributes.borrow().get("class").map(|s| s.to_string()))
}

fn has_notranslate(n: NodeRef) -> bool {
    // check first node
    let _c = get_class_names(n.clone());
    if !_c.is_none() {
        let c = _c.unwrap();
        if c.contains("notranslate") == true {
            return true;
        }
    }

    // check ancestors;
    let a = n.ancestors();
    for i in a {
        let _class = get_class_names(i.clone());
        if !_class.is_none() {
            let class = _class.unwrap();
            if class.contains("notranslate") == true {
                return true;
            }
        }
    }
    return false;
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

        let is_nontranslate = has_notranslate(text_node.as_node().parent().unwrap().clone());

        if is_nontranslate == false {
            if old_text.len() > 2 && is_numeric_and_symbols(&old_text) == false {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let x = rt.block_on(xtrans(&pool, source_lang, target_lang, &old_text, wait));
                if _old_text.ends_with(" ") && _old_text.starts_with(" ") {
                    new_text = format!(" {0} ", x.target_value);
                } else if _old_text.ends_with(" ") {
                    new_text = format!("{0} ", x.target_value);
                } else if _old_text.starts_with(" ") {
                    new_text = format!(" {0}", x.target_value);
                } else {
                    new_text = format!("{0}", x.target_value);
                }
            } else {
                if old_text.len() == 0 {
                    new_text = " ".to_string();
                }
            }
        }
        text_node.replace(new_text);
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
