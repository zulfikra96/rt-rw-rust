pub mod config;
pub mod controllers;
pub mod models;
pub mod schema;

// use std::fs;

use self::models::{NewUser, User};
use actix_cors::Cors;
use actix_remote_ip::*;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, http};
use config::database::establish_connection;
use controllers::{buy_voucher::buy_voucher, get_packets::get_packet, register::register};
use diesel::{
    query_dsl::methods::{FilterDsl, LimitDsl, SelectDsl},
    r2d2::ConnectionManager,
    ExpressionMethods, PgConnection, RunQueryDsl, SelectableHelper,
};
use dotenv::dotenv;
use magic_crypt::new_magic_crypt;
use magic_crypt::MagicCryptTrait;
use r2d2::Pool;
use actix_files as af;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

pub type DBPool = Pool<ConnectionManager<PgConnection>>;

async fn generate_user(pool: web::Data<DBPool>, req: HttpRequest) -> HttpResponse {
    use self::schema::users::dsl::*;
    println!("{:?}", req.peer_addr());
    let mut conn = pool.get().expect("Could not get connection");
    let result: Vec<User> = users
        .filter(phone.eq("085156614335"))
        .limit(1)
        .select(User::as_select())
        .load(&mut conn)
        .expect("something went wrong");
    println!("{:?}", result);
    if result.len() != 0 {
        HttpResponse::Ok().body("");
    }
    let private_key = std::env::var("PRIVATE_KEY").expect("Undefined private key");
    let mc = new_magic_crypt!(&private_key, 256);
    let _password = mc.encrypt_str_to_base64("Billgates1996");
    let data = NewUser {
        name: String::from("Zulfikra"),
        password: _password,
        phone: String::from("085156614335"),
        internet_service_id: 1,
        role:String::from("admin"),
        password_plain:String::from(""),
        mikrotik_id: String::from("1")
    };
    diesel::insert_into(users)
        .values(data)
        .execute(&mut conn)
        .expect("something went wrong");
    HttpResponse::Ok().body("success")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(establish_connection().clone()))
            .app_data(web::Data::new(RemoteIPConfig {
                proxy: Some("IP".to_string()),
            }))
            .service(af::Files::new("/public","src/public").show_files_listing())
            // routes
            .service(buy_voucher)
            .route("/", web::post().to(hello))
            .route("/", web::get().to(controllers::home::index))
            .route("/generate-admin", web::post().to(generate_user))
            // Register
            .route("/register", web::post().to(register))
            .route("/register", web::get().to(controllers::register::index))
            // Packets
            .route("/packets", web::get().to(get_packet))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
