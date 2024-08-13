use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

mod api;
mod handlers;
mod models;
mod utils;

#[derive(Deserialize)]
struct PathParameters {
    name: String,
    id: String,
    email: String,
}


#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Home Page")
}


#[get("/fetch/{name}/{id}/{email}")]
async fn fetch_user(path: web::Path<PathParameters>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Hello {}, Your ID is {} and email is {}",
        &path.name, &path.id, &path.email
    ))
}

#[get("/user")]
async fn get_user(path: web::Query<PathParameters>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Hello {}, Your ID is {} and email is {}",
        &path.name, &path.id, &path.email
    ))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        state: Mutex::new(String::from("init-state")),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(
                web::scope("/auth")
                    .service(login)
                    .service(logout)
                    .service(fetch_user)
                    .service(get_user),
            )
            .service(home)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
