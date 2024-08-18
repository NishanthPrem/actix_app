use crate::models::state;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/logout/{name}")]
async fn logout(app_data: web::Data<state::AppState>, name: web::Path<String>) -> impl Responder {
    let mut state = app_data.state.lock().unwrap();
    println!("current state: {}", state);
    *state = String::from("logout");
    println!("updated state: {}", state);
    HttpResponse::Ok().body(format!("Hello {}, you have been logged out", &name))
}
