use axum::{extract::Query, http::header::HeaderMap, response::IntoResponse, Json};
use deeptrans::{Engine, Translator};
use hex_literal::hex;
use mysql::prelude::*;
use mysql::*;
use sha2::{Digest, Sha256};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Atrans {
    target_value: String,
}
pub async fn hash8(s: &str) -> String {
    let result = Sha256::digest(s);
    let x = format!("{:x}", result).to_string();
    let hash = &x.get(x.len() - 8..);
    return hash.unwrap_or_default().to_string();
}

pub async fn test(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    // http://127.0.0.1:8889/test
    let mut target_value = "".to_string();

    if params.contains_key("t") && params.contains_key("s") && params.contains_key("v") {
        let database_url = "mysql://dbsql1:passpass@localhost:3306/dbsql1";
        let pool = Pool::new(database_url).expect("Failed to create a connection pool");
        let source_value = &params["v"];
        let source_hash = hash8(source_value).await;
        let mut target_hash = "";
        let source_name = &params["s"];
        let target_name = &params["t"];
        let request_hash = hash8(&format!(
            "{0}_{1}_{2}",
            source_name, target_name, source_value
        ))
        .await;

        let mut conn = pool
            .get_conn()
            .expect("Failed to get a connection from the pool");

        let sql0 = format!(
            "SELECT
         t.text AS target_value
         FROM a_source_target  as a
         LEFT JOIN {}  AS t
         ON a.target_id = t.id
         where a.hash = '{1}'
         LIMIT 1",
            &target_name, request_hash
        );

        let atrans: Vec<Atrans> = conn
            .query_map(sql0, |(target_value,)| Atrans { target_value })
            .expect("Failed to fetch data");

        if atrans.is_empty() == false {
            target_value = atrans[0].target_value.to_string();
        } else {
            let trans = Translator::with_engine(source_name, target_name, Engine::Google);
            let _target_value = trans.translate(source_value).await.unwrap();
            let target_value: &str = _target_value.as_str().unwrap_or_default();
            let target_hash = hash8(target_value).await;

            let sql = format!(
                "INSERT IGNORE INTO {0} (hash,text) VALUES (?,?)",
                source_name
            );
            conn.exec_drop(sql, (&source_hash, &source_value))
                .expect("Failed to insert data");
            let last_source_id = conn.last_insert_id();

            let sql = format!(
                "INSERT IGNORE INTO {0} (hash,text) VALUES (?,?)",
                target_name
            );

            conn.exec_drop(sql, (&target_hash, &target_value))
                .expect("Failed to insert data");
            let last_target_id = conn.last_insert_id();
            //        println!("{}", last_source_id);
            //        println!("{}", last_target_id);

            if last_source_id != 0 && last_target_id != 0 {
                let sql = format!("INSERT IGNORE INTO a_source_target (hash, source_name, target_name, source_id, target_id) VALUES (?,?,?,?,?)");
                conn.exec_drop(
                    sql,
                    (
                        &request_hash,
                        &source_name,
                        &target_name,
                        &last_source_id,
                        &last_target_id,
                    ),
                )
                .expect("Failed to insert data");
            }
        }
    }
    let r = serde_json::json!([
        {
            "source": "en",
            "target": "th",
        }
    ]);
    Json(r)
}
