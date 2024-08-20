use crate::models::auth;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/fetch/{name}/{id}/{email}")]
async fn fetch_user(path: web::Path<auth::PathParameters>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Hello {}, Your ID is {} and email is {}",
        &path.name, &path.id, &path.email
    ))
}
