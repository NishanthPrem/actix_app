use crate::models::{auth, state};
use actix_web::{post, web, HttpResponse, Responder};

#[post("/login")]
async fn login(
    app_data: web::Data<state::AppState>,
    req: web::Json<auth::LoginReq>,
) -> impl Responder {
    let mut state = app_data.state.lock().unwrap();
    println!("current state: {}", state);
    *state = String::from("login");
    println!("updated state: {}", state);
    println!("Youre credentials are {}: {}", req.username, req.password);
    HttpResponse::Ok().body("Login")
}
