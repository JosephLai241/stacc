//! The API for the `stacc`.

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod errors;
mod models;
mod routes;
mod utils;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(web::scope("api").service(hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
