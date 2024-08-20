use crate::handlers::public::login;
use actix_web::web;

pub fn get_public_services() -> actix_web::Scope {
    return web::scope("/public").service(login::login);
    // .service(logout)
}
