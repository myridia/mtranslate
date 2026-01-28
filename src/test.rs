use axum::{Json, response::IntoResponse};

use kuchiki::parse_html;
use kuchiki::traits::*;

pub async fn test(x: u8) -> impl IntoResponse {
    let html = r#"<div><p>hello</p></div>"#;
    let document = parse_html().one(html);

    // Loop transversely  and change all text nodes
    for text_node in document.descendants().text_nodes() {
        let _old_text = text_node.borrow().to_uppercase();
        let new_text = "xxxxxx".to_string();
        text_node.replace(new_text);
    }

    // Serialize back to HTML
    let mut output = Vec::new();
    document.serialize(&mut output).unwrap();
    println!("{}", String::from_utf8(output).unwrap());

    let r = serde_json::json!([
        {
            "test": x,
        }
    ]);
    Json(r)
}
