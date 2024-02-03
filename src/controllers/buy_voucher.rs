// use std::fmt::format;
// use actix_remote_ip::RemoteIP;
use crate::models::{GetUserDetail, InternetServices};
// use crate::models::User;
use crate::DBPool;
use actix_web::{post, web, HttpResponse};
use back_end::schema::users::mikrotik_id;
use diesel::query_dsl::methods::FilterDsl;
use diesel::r2d2::ConnectionManager;
use diesel::sql_types::{Integer, VarChar};
use diesel::{query_dsl::methods::SelectDsl, SelectableHelper};
use diesel::{sql_query, ExpressionMethods, PgConnection, RunQueryDsl};
use r2d2::PooledConnection;
use reqwest::Client;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct RequestJson {
    phone: String,
    packet_id: i32,
}
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct ResponseJson {
    pub message: String,
    pub status_code: u32,
}

async fn update_user_packet(
    mut conn: PooledConnection<ConnectionManager<PgConnection>>,
    _mikrotik_id: String,
    packet_id: i32,
) -> Result<usize, String> {
    let statement = r"
        UPDATE users 
        SET internet_service_id = $1
        WHERE mikrotik_id = $2
    ";
    let sql = sql_query(statement);
    let sql = sql
        .bind::<Integer, _>(packet_id)
        .bind::<VarChar, _>(_mikrotik_id)
        .execute(&mut conn);

    match sql {
        Ok(res) => {
            println!(" {}", res);
            return Ok(1);
        }
        Err(err) => {
            println!("Error update : {:?}", err);
            return Err("Something went wrong".to_string());
        }
    }
}

#[post("/buy-voucher")]
pub async fn buy_voucher(conn: web::Data<DBPool>, body: web::Json<RequestJson>) -> HttpResponse {
    use crate::schema::internet_services::dsl::{id as internet_service_id, internet_services};
    use crate::schema::users::dsl::*;
    let mut connection: PooledConnection<ConnectionManager<PgConnection>> = conn.get().expect("something went wring");
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
        println!("user  is not found");
        return HttpResponse::NotFound().json(ResponseJson {
            message: String::from("User is not found"),
            status_code: 422,
        });
    }

    if let None = res[0].mikrotik_id {
        println!("Mikrotik id is not found");
        return HttpResponse::NotFound().json(ResponseJson {
            message: String::from("User is not found"),
            status_code: 422,
        });
    }
    let mut _mikrotik_id: String = String::from("");
    if let Some(some_mikrotik_id) = res[0].mikrotik_id.clone() {
        _mikrotik_id = some_mikrotik_id;
    }
    println!("Mikrotik id {}", _mikrotik_id);
    let url = std::env::var("MIKROTIK_URL").expect("something went wrong");
    let url_format = format!("{url}/rest/ip/hotspot/user/{_mikrotik_id}");
    let client = Client::new();
    let res = client
        .get(&url_format)
        .basic_auth("admin", Some("Billgates1996"))
        .send()
        .await
        .unwrap();
    let res_text: Value = res.json().await.unwrap();
    println!("{} ", res_text["disabled"]);
    if res_text["disabled"] == String::from("false") {
        return HttpResponse::BadRequest().json(ResponseJson {
            message: String::from("Internet anda masih / telah aktif silahkan konfirmasi langsung kepada admin untuk pembelian voucher internet"),
            status_code: 422,
        });
    }
    // check is packet exists
    let packet_service_detail = internet_services
        .select(InternetServices::as_select())
        .filter(internet_service_id.eq(body.packet_id))
        .load(&mut connection);
    let not_found_packet_msg = String::from("Paket tidak ditemukan");
    match packet_service_detail {
        Ok(res) => {
            if res.len() == 0 {
                return HttpResponse::NotFound().json(ResponseJson {
                    message: not_found_packet_msg,
                    status_code: 404,
                });
            }
        }
        Err(_) => {
            return HttpResponse::NotFound().json(ResponseJson {
                message: not_found_packet_msg,
                status_code: 404,
            });
        }
    }
    match update_user_packet(connection, _mikrotik_id, body.packet_id).await {
        Ok(_) => {}
        Err(err) => {
            println!("{:?} ", err);
            return HttpResponse::BadRequest().json(ResponseJson {
                message: String::from("Something went wrong"),
                status_code: 422,
            });
        }
    }
    // let json: Value = serde_json::from_str(res_text.as_str()).unwrap();
    // let packet_detail =
    HttpResponse::Ok().body("success")
}
