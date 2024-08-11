use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct PathParameters {
    name: String,
    id: String,
    email: String,
}
#[derive(Deserialize, Debug)]
struct LoginReq {
    username: String,
    password: String
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Home Page")
}

#[post("/login")]
async fn login(req: web::Json<LoginReq>) -> impl Responder {
    println!("Youre credentials are {}: {}", req.username, req.password);
    HttpResponse::Ok().body("Login")
}

#[get("/logout/{name}")]
async fn logout(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}, you have been logged out", &name))
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
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/auth")
                    .service(login)
                    .service(logout)
                    .service(fetch_user)
                    .service(get_user)
            )
            .service(home)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
