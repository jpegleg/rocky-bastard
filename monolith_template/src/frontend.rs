// module for frontend functionality
HttpServer::new(|| App::new().configure(users::init_routes))
use actix_web::{get, post, web, HttpResponse};
use serde_json::json;
use actix_web::HttpRequest;
use chrono::Utc;
use chrono::DateTime;

mod backend;

#[get("/health")]
async fn healthchecks(req: HttpRequest) -> Result<HttpResponse, CustomError> {
    let peer = req.peer_addr();
    let requ = req.headers();
    let readi: DateTime<Utc> = Utc::now();
    println!("[{:?} INFO ] - - /health GET (health check) request - from {:?} - {:?}", readi, peer, &requ);
    let hresp = "HEALTHY";
    Ok(HttpResponse::Ok().json(hresp))
}

#[get("/")]
async fn reg(req: HttpRequest) -> Result<HttpResponse, CustomError> {
    let peer = req.peer_addr();
    let requ = req.headers();
    let readi: DateTime<Utc> = Utc::now();
    println!("[{:?} INFO ] - - / GET (customer flow) request - from {:?} - {:?}", readi, peer, &requ);
    let _hresp = "DO LOGIN AND ACCOUNT FLOW HERE";
    Ok(HttpResponse::Ok())
}

#[post("/api")]
async fn api(req: HttpRequest) -> Result<HttpResponse, CustomError> {
    let peer = req.peer_addr();
    let requ = req.headers();
    let readi: DateTime<Utc> = Utc::now();
    println!("[{:?} INFO ] - - API POST request - from {:?} - {:?}", readi, peer, &requ);
    let hresp = "DO BACKEND MODULE FUNCTIONS HERE";
    Ok(HttpResponse::Ok().json(hresp))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(health);
    config.service(api);
    config.service(reg);
}
