use axum::{
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};

use libs::email::email;
use libs::nslookup::*;
use libs::test::*;
use libs::token::*;
//use std::net::SocketAddr;
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
            "https://domain-swapper.myridia.com"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_origin("https://lookup.myridia.com".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::OPTIONS, Method::POST]);

    let app = Router::new()
        .route("/nslookup", get(nslookup))
        .route("/test", get(test))
        .route("/token", get(get_token))
        .route("/email", post(email))
        .layer(cors)
        .layer(CorsLayer::permissive());

    println!("Server started successfully");
    let host = "0.0.0.0:8089";
    println!("http://{}/test", host);
    println!("http://{}/token", host);
    println!("http://{}/nslookup?domain=baeckerei-katz.de", host);
    println!("http://127.0.0.1:8089/test");
    println!("https://email.local/test");
    println!("https://api.grallator.com/test");
    println!("https://api.grallator.com/token?page=domain-swapper&client_host=domain-swapper.myridia.com&r=55");
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap()
}
