use crate::config::AppConfig;
use axum::{Json, extract, extract::Query, response::IntoResponse};
use deeptrans::{Engine, Translator};
use kuchiki::traits::*;
use kuchiki::{NodeRef, parse_html};
use mysql::prelude::*;
use mysql::*;
use random_number::random;
use sanitize_html::rules::predefined::DEFAULT;
use sanitize_html::sanitize_str;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use tokio::time::{Duration, sleep};
use toml::Value;

#[derive(Deserialize, Debug)]
pub struct Payload {
    //xemail: String,
    html: String,
    s: String,
    t: String,
}

#[derive(Debug, Serialize)]
struct Translated {
    target_value: String,
    target_hash: String,
    target_lang: String,
    source_lang: String,
    source_hash: String,
    request_hash: String,
    source_value: String,
    msg: String,
}

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

pub async fn translate_html(
    config: AppConfig,
    extract::Json(payload): extract::Json<Payload>,
) -> impl IntoResponse {
    println!("...translate_html fn");

    // curl -X POST  http://0.0.0.0:8089/translate_html -H 'Content-Type:application/json'  -d '{"html":"<div class="\hello should not be translated\" >hello</div>","t":"ru","s":"en"}'
    // curl -X POST  https://mtranslate.myridia.com/translate_html -H 'Content-Type:application/json'  -d '{"html":"<div class="\hello should not be translated\" >hello</div>","t":"ru","s":"en"}'

    let wait: u64 = random!(config.wait_min, config.wait_max);

    let mut t = Translated {
        target_value: "".to_string(),
        target_hash: "".to_string(),
        target_lang: "".to_string(),
        source_lang: "".to_string(),
        source_hash: "".to_string(),
        request_hash: "".to_string(),
        source_value: "".to_string(),
        msg: "".to_string(),
    };

    if payload.html != "" && payload.s != "" && payload.t != "" {
        let database_url: &str = &format!(
            "mysql://{0}:{1}@{2}:{3}/{4}",
            config.db_user, config.db_pass, config.db_host, config.db_port, config.db_name,
        );
        //let mut html = r#"<div><p>hello</p></div>"#;
        let html = payload.html.to_string().clone();
        let mut document = parse_html().one(html);

        // Loop transversely  and change all text nodes
        let pool = Pool::new(database_url).expect("Failed to create a connection pool");
        for text_node in document.descendants().text_nodes() {
            let old_text = text_node.borrow().to_uppercase();
            //t = xtrans(&pool, &payload.s, &payload.t, &old_text, wait).await;
            let new_text = "xxxxxx".to_string();
            text_node.replace(new_text);
        }

        // Serialize back to HTML
        let mut output = Vec::new();
        document.serialize(&mut output).unwrap();
        println!("{}", String::from_utf8(output).unwrap());

        println!("{:?}", t);
    } else {
        t.msg =
            "missing v,s or t parameter, example: https://mtranslate.myridia.com?s=en&t=th&v=hello"
                .to_string();
    }

    Json(t)
}

pub async fn xtrans(
    pool: &Pool,
    source_lang: &str,
    target_lang: &str,
    v: &str,
    wait: u64,
) -> Translated {
    let mut t = Translated {
        target_value: "".to_string(),
        target_hash: "".to_string(),
        target_lang: "".to_string(),
        source_lang: "".to_string(),
        source_hash: "".to_string(),
        request_hash: "".to_string(),
        source_value: "".to_string(),
        msg: "".to_string(),
    };

    let codes: Vec<&str> = env!("codes").split(',').collect();

    let sanitize: &str = &sanitize_str(&DEFAULT, &v).unwrap();
    t.source_value = sanitize.to_string();

    t.source_hash = hash8(v).await;
    t.source_lang = source_lang.to_string();
    t.target_lang = target_lang.to_string();

    t.request_hash = hash8(&format!(
        "{0}_{1}_{2}",
        t.source_lang, t.target_lang, t.source_value
    ))
    .await;

    let at = get_target(&pool, &t.target_lang, &t.request_hash).await;

    if at.is_some() == true {
        println!("...already translated");
        t.target_value = at.clone().unwrap()[0].clone();
        t.target_hash = at.clone().unwrap()[1].clone();
        t.msg = "mtranslated".to_string();
    } else {
        println!("wait: {0}", wait);
        let mut source_id = 0;
        let mut target_id = 0;
        let sr = get_id(&pool, &t.source_lang, &t.source_hash).await;
        if sr.is_some() {
            source_id = sr.unwrap()[0].parse().unwrap();
        }

        let gr = google_translate(&t.source_lang, &t.target_lang, &t.source_value, wait).await;
        if gr.is_some() {
            t.target_value = gr.clone().unwrap()[0].to_string();
            t.target_hash = gr.unwrap()[1].to_string();
            if t.source_hash != t.target_hash {
                t.msg = "gtranslated".to_string();
                let tr = get_id(&pool, &t.target_lang, &t.target_hash).await;
                if tr.is_some() {
                    target_id = tr.unwrap()[0].parse().unwrap();
                }
                if source_id == 0 {
                    let r =
                        insert_lang(&pool, &t.source_lang, &t.source_value, &t.source_hash).await;
                    if r.is_some() {
                        source_id = r.unwrap();
                    }
                }

                if target_id == 0 {
                    let r =
                        insert_lang(&pool, &t.target_lang, &t.target_value, &t.target_hash).await;
                    if r.is_some() {
                        target_id = r.unwrap();
                    }
                }

                if source_id != 0 && target_id != 0 {
                    let id = insert_linking(
                        &pool,
                        &t.request_hash,
                        &t.source_lang,
                        &t.target_lang,
                        source_id,
                        target_id,
                    )
                    .await;
                }
            }
        } else {
            t.msg = "source cannot be translated".to_string();
        }
    }

    //    println!("{:?}", t);

    return t;
}

pub async fn translate(
    config: AppConfig,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let wait: u64 = random!(config.wait_min, config.wait_max);

    let mut t = Translated {
        target_value: "".to_string(),
        target_hash: "".to_string(),
        target_lang: "".to_string(),
        source_lang: "".to_string(),
        source_hash: "".to_string(),
        request_hash: "".to_string(),
        source_value: "".to_string(),
        msg: "".to_string(),
    };

    if params.contains_key("t") && params.contains_key("s") && params.contains_key("v") {
        let database_url: &str = &format!(
            "mysql://{0}:{1}@{2}:{3}/{4}",
            config.db_user, config.db_pass, config.db_host, config.db_port, config.db_name,
        );

        let pool = Pool::new(database_url).expect("Failed to create a connection pool");
        t = xtrans(&pool, &params["s"], &params["t"], &params["v"], wait).await;
    } else {
        t.msg =
            "missing v,s or t parameter, example: https://mtranslate.myridia.com?s=en&t=th&v=hello"
                .to_string();
    }

    Json(t)
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

pub async fn trans_html(html: &str) -> String {
    let mut r = "".to_string();
    //let mut html = r#"<div><p>hello</p></div>"#;
    let mut document = parse_html().one(html);

    // Loop transversely  and change all text nodes
    for text_node in document.descendants().text_nodes() {
        let old_text = text_node.borrow().to_uppercase();
        let new_text = "xxxxxx".to_string();
        text_node.replace(new_text);
    }

    // Serialize back to HTML
    let mut output = Vec::new();
    document.serialize(&mut output).unwrap();
    println!("{}", String::from_utf8(output).unwrap());

    return r;
}
