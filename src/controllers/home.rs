use actix_web::HttpResponse;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template( path = "index.html")]
struct HomeView {}


pub async fn index() -> HttpResponse {
    let ctx = HomeView {};
    HttpResponse::Ok().body(ctx.render_once().unwrap())
}