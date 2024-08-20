use crate::models::{auth, errors};
use actix_web::{get, web, HttpResponse, Responder};

#[get("/user")]
async fn get_user(path: web::Query<auth::PathParameters>) -> Result<impl Responder, errors::HttpError> {
    let user = auth::LoginReq {
        username: "Nishanth".to_string(),
        password: "********".to_string()
    };
    Ok(web::Json(user))
}
