use actix_web::{get, post, web::service, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_user))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Serialize)]
struct MyObj {
    name: String,
}

#[get("/")]
async fn get_user() -> impl Responder {
    let user_details = MyObj {
        name: String::from("Diapk"),
    };
    HttpResponse::Ok().json(user_details)
}
