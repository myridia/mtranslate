use axum::{extract::Request, http::header::HeaderMap, response::IntoResponse, Json};
use deeptrans::{Engine, Translator};

//use axum_client_ip::XRealIp as ClientIp;
//use std::net::SocketAddr;


pub async fn test(headers: HeaderMap, req: Request) -> impl IntoResponse {
    // http://127.0.0.1:8889/test
    //println!("{:?}", ip);

    let xip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or("").trim())
        .filter(|s| !s.is_empty())
        .unwrap_or("unknown");

    let name: &str = &env!("name");
    let host: &str = headers.get("host").unwrap().to_str().unwrap();
    let user_agent: &str = headers.get("user-agent").unwrap().to_str().unwrap();
    let _origin = headers.get("origin");
    let mut origin = "".to_string();
    if _origin.is_some() {
        origin = _origin.unwrap().to_str().unwrap().to_string();
    }


    let translator = Translator::with_engine("es", "en", Engine::Google);

   let translation_result = translator.translate("Saludos a todo el mundo").await;

   println!("{:?}", translation_result);

    
    
    let r = serde_json::json!([
        {
            "name": name,
            "host": host,
            "origin": &origin,
            "user_agent": user_agent,
            "xip": xip,
        }
    ]);

    Json(r)
}
