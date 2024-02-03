use actix_web::{ web, HttpResponse };
use diesel::{r2d2::ConnectionManager, PgConnection, query_dsl::methods::SelectDsl, SelectableHelper, RunQueryDsl};
use r2d2::Pool;
use sailfish::TemplateOnce;
use serde::Serialize;
use crate::models::InternetServices;
use bigdecimal::ToPrimitive;
#[derive(Debug, Serialize)]
struct ResponseJson {
    success: bool,
    data: Vec<InternetServices>
}

pub struct InternetServiceBinding {
    pub id: i32,
    pub name: Option<String>,
    pub price: Option<f64>
}

type DBPool = Pool<ConnectionManager<PgConnection>>;

#[derive(TemplateOnce)]
#[template(path = "packets.html")]
pub struct PacketsView {
    pub packets: Vec<InternetServiceBinding>
}


pub async fn get_packet(
    conn:web::Data<DBPool>
) -> HttpResponse {
    let mut connection = conn.get().expect("something went wrong");
    use crate::schema::internet_services::dsl::*;
    let results = internet_services.select(InternetServices::as_select())
        .load(&mut connection)
        .expect("something went wrong");
    println!(" result {:?}", results);
    let results: Vec<InternetServiceBinding> = results.into_iter().map(| e|  {
        return InternetServiceBinding {
            id: e.id,
            name: e.name,
            price: e.price.unwrap().to_f64()
        }
    }).collect();
    let ctx = PacketsView {
        packets: results
    };

    HttpResponse::Ok().body(ctx.render_once().unwrap())
}