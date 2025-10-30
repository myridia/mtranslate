use axum::{extract::Query, response::IntoResponse, Json};
use deeptrans::{Engine, Translator};
use mysql::prelude::*;
use mysql::*;
use random_number::random;
use sanitize_html::rules::predefined::DEFAULT;
use sanitize_html::sanitize_str;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use tokio::time::{sleep, Duration};
use toml::Value;

#[derive(Debug)]
struct Atrans {
    target_value: String,
    target_hash: String,
}

#[derive(Debug)]
struct Xtrans {
    id: i64,
    value: String,
}

pub async fn translate(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    // http://127.0.0.1:8889/test

    let mut source_name = "";
    let mut source_hash = "".to_string();
    let mut source_value = "";
    let mut source_id = "";

    let mut target_name = "";
    let mut target_hash = "".to_string();
    let mut target_value = "".to_string();
    let mut target_id = "";

    let mut return_value = "".to_string();
    let mut return_target = "".to_string();
    let mut return_source = "".to_string();

    let mut req_hash = "".to_string();
    let mut return_hash = "".to_string();
    let mut request_hash = "".to_string();

    let mut msg = "".to_string();

    //let s = fs::read_to_string("config.toml");
    //let v: Value = toml::from_str(&s);
    //println!("{:?}", s);

    if params.contains_key("t") && params.contains_key("s") && params.contains_key("v") {
        let codes: Vec<&str> = env!("codes").split(',').collect();
        let database_url: &str = &format!(
            "mysql://{0}:{1}@{2}:{3}/{4}",
            env!("db_user"),
            env!("db_pass"),
            env!("db_host"),
            env!("db_port"),
            env!("db_name")
        );

        let pool = Pool::new(database_url).expect("Failed to create a connection pool");
        let sanitize: &str = &sanitize_str(&DEFAULT, &params["v"]).unwrap();
        source_value = sanitize;
        source_hash = hash8(source_value).await;
        source_name = &params["s"];
        target_name = &params["t"];

        if codes.contains(&source_name) && codes.contains(&target_name) && source_value.len() < 1000
        {
            request_hash = hash8(&format!(
                "{0}_{1}_{2}",
                source_name, target_name, source_value
            ))
            .await;
            req_hash = request_hash.clone();

            let atrans = get_target(&pool, &target_name, &request_hash).await;

            if atrans.is_some() == true {
                println!("...translated already");
                target_value = atrans.clone().unwrap()[0].clone();
                target_hash = atrans.clone().unwrap()[1].clone();
                //return_value = target_value;
                //return_hash = target_hash;
            } else {
                let wait: u64 = random!(2000, 7000);
                println!("wait: {0}", wait);
                println!("source_name: {0}", source_name);
                println!("source_hash: {0}", source_hash);

                let sr = get_id_hash(pool, &source_name, &source_hash).await;
                if sr.is_some() {
                    source_id = &sr.unwrap()[0];
                }

                sleep(Duration::from_millis(wait)).await;
                let trans = Translator::with_engine(source_name, target_name, Engine::Google);
                let _target_value = trans.translate(source_value).await.unwrap();
                target_value = _target_value.as_str().unwrap_or_default().to_string();
                target_hash = hash8(&target_value).await;

                //println!("{0}", source_hash);
                //println!("{0}", target_hash);
                if source_hash != target_hash {

                    /*
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
                    //println!("{}", last_source_id);
                    println!("{}", last_target_id);

                    if last_source_id != 0 && last_target_id != 0 {
                        let sql = format!("INSERT IGNORE INTO a_source_target (hash, source_name, target_name, source_id, target_id) VALUES (?,?,?,?,?)");
                        println!("{}", sql);
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
                    }  */
                } else {
                    return_value = "".to_string();
                    return_hash = "".to_string();
                    msg = "source cannot be translated".to_string();
                }
            }
        } else {
            msg = "wrong v,s or t parameter, example: https://translate.myridia.com?s=en&t=th&v=hello -  not more than 1000 characters".to_string();
        }
    } else {
        msg =
            "missing v,s or t parameter, example: https://mtranslate.myridia.com?s=en&t=th&v=hello"
                .to_string();
    }
    let r = serde_json::json!([
        {
            "target_value": target_value,
            "target_name": target_name,
            "source_name": source_name,

            //"req_hash": req_hash,
            //"return_hash": return_hash,
            "msg": msg,
        }
    ]);
    Json(r)
}

pub async fn hash8(s: &str) -> String {
    let result = Sha256::digest(s);
    let x = format!("{:x}", result).to_string();
    let _hash = &x.get(x.len() - 8..);
    let hash = _hash.unwrap_or_default().to_string();
    return hash;
}

pub async fn get_id_hash(pool: Pool, name: &str, hash: &str) -> Option<Vec<String>> {
    println!("...fn get_id_hash");
    let mut v: Vec<String> = vec![];
    let mut conn = pool
        .get_conn()
        .expect("Failed to get a connection from the pool");
    let sql = format!(
        "SELECT
         s.id AS id
         ,s.text AS source_value
         FROM {0} as s
         where s.hash = '{1}'
         LIMIT 1",
        &name, &hash
    );

    //println!("{}", sql);

    let r: Vec<Xtrans> = conn
        .query_map(sql, |(id, value)| Xtrans { id, value })
        .expect("Failed to fetch data");

    if !r.is_empty() {
        v.push(r[0].id.to_string());
        v.push(r[0].value.to_string());
        //println!("{:?}", v);
        return Some(v);
    }

    return None;
}

pub async fn get_target(pool: &Pool, target_name: &str, request_hash: &str) -> Option<Vec<String>> {
    println!("...fn get_target");

    //println!("{}", sql0);

    let mut v: Vec<String> = vec![];
    let mut conn = pool
        .get_conn()
        .expect("Failed to get a connection from the pool");
    let sql0 = format!(
        "SELECT
         t.text AS target_value
         ,t.hash AS target_hash
         FROM a_source_target  as a
         LEFT JOIN {0}  AS t
         ON a.target_id = t.id
         where a.hash = '{1}'
         LIMIT 1",
        &target_name, request_hash
    );

    //println!("{}", sql);
    let r: Vec<Atrans> = conn
        .query_map(sql0, |(target_value, target_hash)| Atrans {
            target_value,
            target_hash,
        })
        .expect("Failed to fetch data");

    /*
    let r: Vec<Xtrans> = conn
        .query_map(sql, |(id, value)| Xtrans { id, value })
        .expect("Failed to fetch data");
    */
    if !r.is_empty() {
        v.push(r[0].target_value.to_string());
        v.push(r[0].target_hash.to_string());
        //println!("{:?}", v);
        return Some(v);
    }

    return None;
}
