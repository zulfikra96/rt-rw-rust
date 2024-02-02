use std::collections::HashMap;

use crate::controllers::buy_voucher::ResponseJson;
use crate::models::GetUserDetail;
use crate::models::NewUser;
use crate::DBPool;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::ExpressionMethods;
use magic_crypt::new_magic_crypt;
use magic_crypt::MagicCryptTrait;
use rand::Rng;
use reqwest::Client;
use sailfish::TemplateOnce;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    pub name: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize)]
pub struct MikrotikJson {
    pub name: String,
    pub password: String,
}

#[derive(TemplateOnce)]
#[template(path = "register.html")]
pub struct RegisterView {}

pub async fn index() -> impl Responder {
    let ctx = RegisterView {};

    HttpResponse::Ok().body(ctx.render_once().unwrap())
}

pub async fn register(conn: web::Data<DBPool>, body: web::Json<RequestBody>) -> HttpResponse {
    use crate::schema::users::dsl::*;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    const PASSWORD_LEN: usize = 5;
    let _phone = &body.phone;
    let mut rng = rand::thread_rng();
    let mut connection = conn.get().expect("Something went wrong");
    let _password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    let result = users
        .filter(phone.eq(&_phone))
        .select(GetUserDetail::as_select())
        .load(&mut connection)
        .unwrap();
    if result.len() > 0 {
        return HttpResponse::BadRequest().json(ResponseJson {
            message: String::from("Akun telah terpakai, silahkan gunakan nomor telp yang lain"),
            status_code: 400,
        });
    }
    let mikrotik_url = std::env::var("MIKROTIK_URL").expect("mikrotik url env not defined");
    let mikrotik_user = std::env::var("MIKROTIK_USER").expect("mikrotik user env is not definded");
    let mikrotik_password =
        std::env::var("MIKROTIK_PASSWORD").expect("mikrotik password env is not definded");
    let _pwd = &_password;
    let mut map = HashMap::new();
    let profile = &String::from("MEMBER");
    map.insert("name", &_phone);
    map.insert("password", &_pwd);
    map.insert("profile", &profile);
    println!("Phone {:?}", body);
    let mikrotik_client = Client::new()
        .post(format!("{mikrotik_url}/rest/ip/hotspot/user/add"))
        .basic_auth(&mikrotik_user, Some(&mikrotik_password))
        .json(&map)
        .header("content-type", "application/json")
        .send()
        .await;
    let mikrotik_client = match mikrotik_client {
        Ok(e) => e,
        Err(err) => {
            println!("Error : {:?}", err);
            return HttpResponse::BadRequest().json(ResponseJson {
                message: String::from("Ada kesalahan"),
                status_code: 400,
            });
        }
    };
    let mikrotik_client = mikrotik_client.text().await.unwrap();
    let text_to_json: serde_json::Value = serde_json::from_str(mikrotik_client.as_str()).unwrap();
    // println!("error {}", &text_to_json["ret"]);
    println!("response {}", text_to_json["ret"]);
    if text_to_json["ret"].is_null() {
        return HttpResponse::BadRequest().json(ResponseJson {
            message: String::from("Ada kesalahan"),
            status_code: 400,
        });
    }

    let private_key = std::env::var("PRIVATE_KEY").expect("Password is not defined");
    let mc = new_magic_crypt!(&private_key, 256);
    let chipper_password = mc.encrypt_str_to_base64(&_password);
    let params = NewUser {
        internet_service_id: 7,
        name: body.name.clone(),
        password: chipper_password,
        password_plain: _password,
        phone: _phone.to_string(),
        role: String::from("member"),
        mikrotik_id: text_to_json["ret"].to_string().replace("\"", ""),
    };

    diesel::insert_into(users::table())
        .values(&params)
        .execute(&mut connection)
        .expect("Something went wrong");
    HttpResponse::Ok().body("Hello world")
}
