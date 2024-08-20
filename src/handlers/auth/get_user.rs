use crate::models::auth;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/user")]
async fn get_user(path: web::Query<auth::PathParameters>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Hello {}, Your ID is {} and email is {}",
        &path.name, &path.id, &path.email
    ))
}
