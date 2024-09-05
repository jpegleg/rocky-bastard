// module for frontend functionality template

use actix_web::{get, web, HttpResponse};
use actix_web::HttpRequest;
use actix_web::http::Error;
use chrono::Utc;
use chrono::DateTime;

#[get("/health")]
async fn healthchecks(req: HttpRequest) -> Result<HttpResponse, Error> {
    let peer = req.peer_addr();
    let requ = req.headers();
    let readi: DateTime<Utc> = Utc::now();
    println!("[{:?} INFO ] - - /health GET (health check) request - from {:?} - {:?}", readi, peer, &requ);
    let hresp = "HEALTHY";
    Ok(HttpResponse::Ok().json(hresp))
}

#[get("/")]
async fn reg(req: HttpRequest) -> Result<HttpResponse, Error> {
    let peer = req.peer_addr();
    let requ = req.headers();
    let readi: DateTime<Utc> = Utc::now();
    println!("[{:?} INFO ] - - / GET (customer flow) request - from {:?} - {:?}", readi, peer, &requ);
    let _hresp = "DO LOGIN AND ACCOUNT FLOW HERE";
    Ok(HttpResponse::Ok())
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(health);
    config.service(reg);
}
