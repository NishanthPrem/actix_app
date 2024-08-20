use crate::handlers::auth::{fetch_user, get_user, logout};
use actix_web::web;
pub fn get_auth_services() -> actix_web::Scope {
    return web::scope("/auth")
        .service(get_user::get_user)
        .service(fetch_user::fetch_user)
        .service(logout::logout);
}
