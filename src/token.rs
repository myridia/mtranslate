//use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use axum::{extract::Query, http::HeaderMap, response::IntoResponse, Json};
//use base64::{decode, encode};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
}

pub async fn get_token(
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // http://127.0.0.1:8889/get_token
    let mut token = "".to_string();
    let mut page = "".to_string();
    let mut client_host = "".to_string();
    let mut random = "".to_string();
    let mut origin = "".to_string();

    //println!("{}", env!("token_secret"));

    if params.contains_key("page") {
        page = params["page"].replace(" ", "");
        println!("{}", page);
    }

    if params.contains_key("client_host") {
        client_host = params["client_host"].replace(" ", "");
        //println!("{}", client_host);
    }
    if params.contains_key("r") {
        random = params["r"].replace(" ", "");
        //println!("{}", client_host);
    }

    let _origin = headers.get("origin");
    if _origin.is_some() {
        origin = _origin
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .replace("https://", "")
            .replace("http://", "");
    }
    let host: &str = &origin;
    let _hosts: &str = &env!("hosts");
    let hosts: Vec<&str> = _hosts.split(",").collect();
    println!("{:?}", hosts);
    println!("{:?}", hosts.contains(&host));
    if hosts.contains(&host)
        && page != ""
        && client_host != ""
        && client_host == host
        && random != ""
    {
        let _token = encode_token(host, &page, &random).await;
        if _token.is_ok() {
            token = _token.unwrap();
        }
    } else {
        token = env!("token_fake").to_string();
    }
    let r = serde_json::json!([
        {
            "name": "email token",
            "host": host,
            "client_host": client_host,
            "page": page,
            "token":token
        }
    ]);

    Json(r)
}

pub async fn encode_token(host: &str, page: &str, random: &str) -> Result<String, Box<dyn Error>> {
    let secret: &[u8] = &env!("token_secret").to_string().into_bytes();
    let token_valid: u64 = env!("token_valid").to_string().parse().unwrap();
    //println!("{:?}", secret);
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret)?;
    //let mut claims = BTreeMap::new();

    let mut claims = BTreeMap::<&str, &str>::new();
    let _unixtime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + token_valid;
    let _unixstamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let unixtime = &_unixtime.to_string();
    let unixstamp = &_unixstamp.to_string();
    claims.insert("h", host);
    claims.insert("p", page);
    claims.insert("e", unixtime);
    claims.insert("u", unixstamp);
    claims.insert("r", random);
    let token = claims.sign_with_key(&key)?;

    Ok(token)
}

pub async fn test_ok() -> String {
    let s = "ok".to_string();
    return s;
}

pub async fn auth_token(token: &str) -> Result<bool, Box<dyn Error>> {
    //println!("{}", token);
    let secret: &[u8] = &env!("token_secret").to_string().into_bytes();
    let mut r = false;
    let unix_now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret).unwrap();
    let _claims: Result<BTreeMap<String, String>, jwt::Error> = token.verify_with_key(&key);
    if _claims.is_ok() {
        let claims = _claims.unwrap();
        let _unix_expire: Result<String, _> = claims["e"].parse();
        let _unix_stamp: Result<String, _> = claims["u"].parse();
        let _host: Result<String, _> = claims["h"].parse();
        let _page: Result<String, _> = claims["p"].parse();
        let _random: Result<String, _> = claims["r"].parse();
        if _unix_expire.is_ok() && _host.is_ok() && _page.is_ok() & _random.is_ok() {
            let unix_expire: u64 = _unix_expire.unwrap().parse().unwrap();
            let unix_stamp: u64 = _unix_stamp.unwrap().parse().unwrap();
            let host: String = _host.unwrap().parse().unwrap();
            //let page: String = _page.unwrap().parse().unwrap();
            let random: i32 = _random.unwrap().parse().unwrap();
            let random_max: i32 = env!("random_max").to_string().parse().unwrap();
            let time_passed: u64 = env!("time_passed").to_string().parse().unwrap();
            let _hosts: &str = &env!("hosts");
            let hosts: Vec<&str> = _hosts.split(",").collect();
            //println!("{:?}", time_passed);
            //println!("{:?}", unix_now - unix_stamp);
            //println!("{:?}", unix_expire - unix_now);
            if unix_expire >= unix_now
                && hosts.contains(&host.as_str())
                && random <= random_max
                && (unix_now - unix_stamp) > time_passed
            {
                r = true;
            }
        }
    }
    Ok(r)
}
