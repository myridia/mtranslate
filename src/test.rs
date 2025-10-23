use axum::{extract::Request, http::header::HeaderMap, response::IntoResponse, Json};
use deeptrans::{Engine, Translator};
use mysql::prelude::*;
use mysql::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
//use axum_client_ip::XRealIp as ClientIp;
//use std::net::SocketAddr;

pub async fn hash8(s: &str) -> String {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();
    format!("{:016x}", hash)[..8].to_string()
}

pub async fn test(headers: HeaderMap, req: Request) -> impl IntoResponse {
    // http://127.0.0.1:8889/test
    let database_url = "mysql://dbsql1:passpass@localhost:3306/dbsql1";
    let pool = Pool::new(database_url).expect("Failed to create a connection pool");
    let sv = "hello";
    let sh = hash8(sv).await;

    let s = "en";
    let t = "th";
    let mut conn = pool
        .get_conn()
        .expect("Failed to get a connection from the pool");

    let trans = Translator::with_engine(s, t, Engine::Google);
    let _tv = trans.translate(sv).await.unwrap();
    let tv: &str = _tv.as_str().unwrap_or_default();
    let th = hash8(tv).await;

    let sql = format!("INSERT INTO {0} (hash,text) VALUES (?,?)", s);
    conn.exec_drop(sql, (&sh, &sv))
        .expect("Failed to insert data");

    let sql = format!("INSERT INTO {0} (hash,text) VALUES (?,?)", t);
    conn.exec_drop(sql, (&th, &tv))
        .expect("Failed to insert data");

    let r = serde_json::json!([
        {
            "source": "en",
            "target": "th",
            "source_value": sv,
            "target_value": tv,
        }
    ]);
    Json(r)
}
