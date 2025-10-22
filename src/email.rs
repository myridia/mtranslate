use crate::token;
use axum::{extract, response::IntoResponse, Json};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;
use urldecode::decode;

#[derive(Deserialize)]
pub struct Messagex {
    //xemail: String,
    xname: String,
    xmessage: String,
    token: String,
    //random: String,
    page: String,
}

pub async fn email(extract::Json(payload): extract::Json<Messagex>) -> impl IntoResponse {
    let mut ret = serde_json::json!({"ok": false,"msg":"Could not send email"});
    //let r = "Could not send email";
    let page = decode(payload.page);
    let name = decode(payload.xname);
    //let email = decode(payload.xemail);
    let message = decode(payload.xmessage);
    let token = decode(payload.token);
    //let random = decode(payload.random);
    let v_message: Vec<String> = serde_json::from_str(&message).unwrap();
    //let x = token::test_ok().await;
    let a = token::auth_token(&token).await;
    //println!("{:?}", a);
    if a.unwrap() == true {
        //println!("ok");
        let smtp_host: &str = &env!("smtp_host");
        let smtp_user: &str = &env!("smtp_user");
        let smtp_pass: &str = &env!("smtp_pass");
        //let smtp_port: u16 = env!("smtp_port").parse().unwrap();
        let email_send_from: &str = &env!("email_send_from");
        let email_send_to: &str = &env!("email_send_to");
        let name_send_to: &str = &env!("name_send_to");
        let mut body = "".to_string();
        body.push_str(&format!("Page: {page}\n"));
        body.push_str(&format!("Subject: {name}\n"));
        body.push_str(&v_message.join("\n"));
        let email = Message::builder()
            .from(format!("{page} <{email_send_from}>").parse().unwrap())
            .to(format!("{name_send_to} <{email_send_to}>").parse().unwrap())
            .subject(format!("Sending email from {page}"))
            .body(body)
            .unwrap();
        let creds = Credentials::new(smtp_user.to_string(), smtp_pass.to_string());

        let mailer = SmtpTransport::relay(smtp_host)
            .unwrap()
            .credentials(creds)
            .build();
        // Send the email
        match mailer.send(&email) {
            Ok(_) => ret = serde_json::json!({"ok": true,"msg":"Successfully send email,"}),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
    Json(ret)
}
