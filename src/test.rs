use axum::{Json, response::IntoResponse};
use kuchiki::parse_html;
use kuchiki::traits::*;
use scraper::Html;
use serde::{Deserialize, Serialize};
use std::io;
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

#[axum_macros::debug_handler]
pub async fn test() -> impl IntoResponse {
    let html = r#"<div><p>hello</p></div>"#;

    let document = match task::spawn_blocking(move || heavy_sync_transform(html.to_string())).await
    {
        Ok(v) => v, // the function returned successfully
        Err(join_err) => "".to_string(),
    };

    println!("{:?}", document);

    /*
        let document = parse_html().one(html);

        // Loop transversely  and change all text nodes
        for text_node in document.descendants().text_nodes() {
            let _old_text = text_node.borrow().to_uppercase();
            let new_text = "xxxxxx".to_string();
            text_node.replace(new_text);
    }
        */
    /*
    // Serialize back to HTML
    let mut output = Vec::new();
    document.serialize(&mut output).unwrap();
    println!("{}", String::from_utf8(output).unwrap());
    */
    let mut t = Translated {
        target_value: "".to_string(),
        target_hash: "".to_string(),
        target_lang: "".to_string(),
        source_lang: "".to_string(),
        source_hash: "".to_string(),
        request_hash: "".to_string(),
        source_value: "".to_string(),
        msg: "".to_string(),
    };

    t = a("hello").await;

    //let message = helper("World").await;
    Json(t)
}

async fn helper(name: &str) -> String {
    format!("Hello, {name}!")
}

fn heavy_sync_transform(html: String) -> String {
    // simulate CPU / blocking work
    //std::thread::sleep(std::time::Duration::from_millis(200));

    let document = parse_html().one(html.clone());

    // Loop transversely  and change all text nodes
    for text_node in document.descendants().text_nodes() {
        let _old_text = text_node.borrow().to_uppercase();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(a("hello"));
        println!("{:?}", result);

        let new_text = "xxxxxx".to_string();
        text_node.replace(new_text);
    }

    let mut output = Vec::new();
    document.serialize(&mut output).unwrap();
    let new_html = String::from_utf8(output).unwrap();

    // simple transform for demo
    //html.replace("hello", "FOO_MODIFIED")

    return new_html;
}

fn get_nodes(html: String) -> kuchiki::NodeRef {
    let d = parse_html().one(html.clone());

    return d;
}

pub async fn a(x: &str) -> Translated {
    let mut t = Translated {
        target_value: "".to_string(),
        target_hash: "".to_string(),
        target_lang: "".to_string(),
        source_lang: "".to_string(),
        source_hash: "".to_string(),
        request_hash: "".to_string(),
        source_value: "".to_string(),
        msg: "fffffffffffffff".to_string(),
    };

    t
}
