use actix_web::web;
use crate::handlers::auth::{get_user, fetch_user, logout};
pub fn get_auth_services() -> actix_web::Scope {
    return web::scope("/auth")
    .service(get_user::get_user)
    .service(fetch_user::fetch_user)
    .service(logout::logout)
}
