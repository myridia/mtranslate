use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};

use libs::test::*;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    //let token = encode_token().await;
    //let is_auth = auth_token(&token.unwrap()).await.unwrap();
    //println!("{}", is_auth);

    let cors = CorsLayer::new()
        .allow_origin("http://127.0.0.1".parse::<HeaderValue>().unwrap())
        .allow_origin(
            "https://translate.myridia.com"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_origin("https://lookup.myridia.com".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::OPTIONS, Method::POST]);

    let app = Router::new()
        .route("/test", get(test))
        .layer(cors)
        .layer(CorsLayer::permissive());

    println!("Server started successfully");
    let host = "0.0.0.0:8089";
    println!("http://{}/test?s=en&t=th&v=hello", host);

    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap()
}
