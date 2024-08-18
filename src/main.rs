use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod api;
mod handlers;
mod models;
mod utils;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Home Page")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(models::state::AppState {
        state: Mutex::new(String::from("init-state")),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(api::public::get_public_services())
            .service(
                web::scope("/auth"), // .service(logout)
            )
            .service(home)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
