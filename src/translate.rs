use axum::{Json, extract::Query, response::IntoResponse};
use deeptrans::{Engine, Translator};
use mysql::prelude::*;
use mysql::*;
use random_number::random;
use sanitize_html::rules::predefined::DEFAULT;
use sanitize_html::sanitize_str;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use tokio::time::{Duration, sleep};
use toml::Value;

use crate::config::AppConfig;

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

pub async fn translate(
    config: AppConfig,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    println!("{:?}", config.db_host);
    let wait: u64 = random!(2000, 7000);
    let mut source_lang = "";
    let mut source_hash = "".to_string();
    let mut source_value = "";
    let mut source_id = 0;

    let mut target_lang = "";
    let mut target_hash = "".to_string();
    let mut target_value = "".to_string();
    let mut target_id = 0;

    let mut request_hash = "".to_string();
    let mut return_hash = "".to_string();

    let mut msg = "".to_string();

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
        println!("{:?}", database_url);
        let pool = Pool::new(database_url).expect("Failed to create a connection pool");
        let sanitize: &str = &sanitize_str(&DEFAULT, &params["v"]).unwrap();
        source_value = sanitize;
        source_hash = hash8(source_value).await;
        source_lang = &params["s"];
        target_lang = &params["t"];

        if codes.contains(&source_lang) && codes.contains(&target_lang) && source_value.len() < 1000
        {
            request_hash = hash8(&format!(
                "{0}_{1}_{2}",
                source_lang, target_lang, source_value
            ))
            .await;

            let already_translated = get_target(&pool, &target_lang, &request_hash).await;

            if already_translated.is_some() == true {
                println!("...translated already");
                target_value = already_translated.clone().unwrap()[0].clone();
                target_hash = already_translated.clone().unwrap()[1].clone();
            } else {
                println!("wait: {0}", wait);

                let sr = get_id(&pool, &source_lang, &source_hash).await;
                if sr.is_some() {
                    source_id = sr.unwrap()[0].parse().unwrap();
                }

                let gr = google_translate(source_lang, target_lang, source_value, wait).await;
                if gr.is_some() {
                    //println!("{:?}", gr);
                    //println!("{0}", target_hash);
                    target_value = gr.clone().unwrap()[0].to_string();
                    target_hash = gr.unwrap()[1].to_string();
                    if source_hash != target_hash {
                        println!("source_id: {}", source_id);
                        println!("source_hash: {}", source_hash);
                        println!("target_hash: {}", target_hash);

                        let tr = get_id(&pool, &target_lang, &target_hash).await;
                        if tr.is_some() {
                            target_id = tr.unwrap()[0].parse().unwrap();
                        }
                        if source_id == 0 {
                            let r =
                                insert_lang(&pool, source_lang, source_value, &source_hash).await;
                            if r.is_some() {
                                source_id = r.unwrap();
                            }
                        }

                        if target_id == 0 {
                            let r =
                                insert_lang(&pool, target_lang, &target_value, &target_hash).await;
                            if r.is_some() {
                                target_id = r.unwrap();
                            }
                        }

                        if source_id != 0 && target_id != 0 {
                            let id = insert_linking(
                                &pool,
                                &request_hash,
                                source_lang,
                                target_lang,
                                source_id,
                                target_id,
                            )
                            .await;
                        }
                    }
                } else {
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
    let r = serde_json::json!(
        {
            "target_value": target_value,
            "target_hash": target_hash,
            "target_lang": target_lang,
            "source_lang": source_lang,
            "request_hash": request_hash,
            "msg": msg,
        }
    );
    Json(r)
}

pub async fn hash8(s: &str) -> String {
    let result = Sha256::digest(s);
    let x = format!("{:x}", result).to_string();
    let _hash = &x.get(x.len() - 8..);
    let hash = _hash.unwrap_or_default().to_string();
    return hash;
}

pub async fn get_id(pool: &Pool, name: &str, hash: &str) -> Option<Vec<String>> {
    println!("...fn get_id for {0} - {1}", name, hash);
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

pub async fn google_translate(
    source_lang: &str,
    target_lang: &str,
    source_value: &str,
    wait: u64,
) -> Option<Vec<String>> {
    println!("...fn google_translate");
    let mut v: Vec<String> = vec![];
    sleep(Duration::from_millis(wait)).await;
    let trans = Translator::with_engine(source_lang, target_lang, Engine::Google);
    let r = trans.translate(source_value).await;
    if r.is_ok() {
        let value = r.unwrap().as_str().unwrap_or_default().to_string();
        let hash = hash8(&value).await;
        v.push(value);
        v.push(hash);
        return Some(v);
    }
    return None;
}

pub async fn insert_lang(pool: &Pool, lang: &str, value: &str, hash: &str) -> Option<u64> {
    println!("...fn insert_lang  {0} | {1} | {2}", lang, value, hash);
    let mut conn = pool
        .get_conn()
        .expect("Failed to get a connection from the pool");
    let sql = format!("INSERT IGNORE INTO {0} (hash,text) VALUES (?,?)", lang);
    conn.exec_drop(sql, (&hash, &value))
        .expect("Failed to insert data");
    let id: u64 = conn.last_insert_id();
    if id > 0 {
        return Some(id);
    }
    return None;
}

pub async fn insert_linking(
    pool: &Pool,
    request_hash: &str,
    source_lang: &str,
    target_lang: &str,
    source_id: u64,
    target_id: u64,
) -> Option<u64> {
    println!("...fn insert_linking");
    let mut conn = pool
        .get_conn()
        .expect("Failed to get a connection from the pool");

    let sql = format!(
        "INSERT IGNORE INTO a_source_target (hash, source_name, target_name, source_id, target_id) VALUES (?,?,?,?,?)"
    );
    //println!("{}", sql);
    let mut conn = pool
        .get_conn()
        .expect("Failed to get a connection from the pool");
    conn.exec_drop(
        sql,
        (
            &request_hash,
            &source_lang,
            &target_lang,
            &source_id,
            &target_id,
        ),
    )
    .expect("Failed to insert data");

    let id: u64 = conn.last_insert_id();
    if id > 0 {
        return Some(id);
    }
    return None;
}
