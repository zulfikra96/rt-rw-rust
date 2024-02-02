// use std::fmt::format;
// use actix_remote_ip::RemoteIP;
use crate::models::GetUserDetail;
// use crate::models::User;
use crate::DBPool;
use actix_web::{post, web, HttpResponse};
use diesel::query_dsl::methods::FilterDsl;
use diesel::{query_dsl::methods::SelectDsl, SelectableHelper};
use diesel::{ExpressionMethods, RunQueryDsl};
use reqwest::Client;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct RequestJson {
    phone: String,
}
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct ResponseJson {
    pub message: String,
    pub status_code: u32,
}

#[post("/buy-voucher")]
pub async fn buy_voucher(conn: web::Data<DBPool>, body: web::Json<RequestJson>) -> HttpResponse {
    use crate::schema::users::dsl::*;
    let mut connection = conn.get().expect("something went wring");
    println!("{}", body.phone);
    let results = users
        .select(GetUserDetail::as_select())
        .filter(phone.eq(body.phone.to_string()))
        .load(&mut connection);

    let res = match results {
        Ok(res) => res,
        Err(err) => {
            println!("Error: {}", err);
            return HttpResponse::BadRequest().json(ResponseJson {
                message: String::from("Something went wrong"),
                status_code: 422,
            });
        }
    };
    if res.len() == 0 {
        return HttpResponse::NotFound().json(ResponseJson {
            message: String::from("User is not found"),
            status_code: 422,
        });
    }
    println!("Call here {:?}", res);

    if let None = res[0].mikrotik_id {
        return HttpResponse::NotFound().json(ResponseJson {
            message: String::from("User is not found"),
            status_code: 422,
        });
    }

    let url = std::env::var("MIKROTIK_URL").expect("something went wrong");
    let url_format = format!("{url}/rest/ip/hotspot/user");
    let client = Client::new();
    let res = client
        .get(&url_format)
        .basic_auth("admin", Some("Billgates1996"))
        .send()
        .await.unwrap();
    let res_text = res.text().await.unwrap();
    let json: Value = serde_json::from_str(res_text.as_str()).unwrap();

    HttpResponse::Ok().body("success")
}
