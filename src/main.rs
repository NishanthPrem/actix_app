use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Home Page")
}

#[post("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login")
}

#[get("/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().body("Logout")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/auth").service(login).service(logout))
            .service(home)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
