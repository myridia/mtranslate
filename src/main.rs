use axum::{
    Router,
    http::{HeaderValue, Method},
    routing::{get, post},
};

use libs::config::get_config;
use libs::help::help;
use libs::html_translate::*;
use libs::test::test_get;
use libs::translate::translate;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let config = get_config();
    let config2 = get_config();

    let cors = CorsLayer::new()
        .allow_origin("http://127.0.0.1".parse::<HeaderValue>().unwrap())
        .allow_origin(
            "https://translate.myridia.com"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_origin("https://lookup.myridia.com".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::OPTIONS, Method::POST]);
    let x: u8 = 3;

    let app = Router::new()
        .route("/", get(move |p| translate(config, p)))
        .route("/", post(move |p| html(config2, p)))
        .route("/help", get(help))
        .route("/test", get(move || test_get(x)))
        .layer(cors)
        .layer(CorsLayer::permissive());

    println!("Server started successfully");
    let h: String = hostname::get().unwrap().into_string().unwrap();
    let host = format!("0.0.0.0:8089");
    println!("http://{}:8089/test", h);
    println!("http://{}:8089?s=en&t=th&v=hello", host);

    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap()
}
