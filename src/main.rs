use axum::{
    Router,
    http::{HeaderValue, Method},
    routing::get,
};

use libs::config::get_config;
use libs::help::help;
use libs::test::test;
use libs::translate::translate;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let config = get_config();
    //println!("{:?}", config.db_user);
    //    std::env::set_var("host", "123"); // Sets AAA to 123
    unsafe {
        std::env::set_var("db_host", config.db_host);
        std::env::set_var("db_name", config.db_name);
        std::env::set_var("db_user", config.db_user);
        std::env::set_var("db_pass", config.db_pass);
        std::env::set_var("db_port", config.db_port);
    };

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
        .route("/", get(translate))
        .route("/help", get(help))
        .route("/test", get(test))
        .layer(cors)
        .layer(CorsLayer::permissive());

    println!("Server started successfully");
    let host = "0.0.0.0:8089";
    println!("http://{}?s=en&t=th&v=hello", host);

    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap()
}
