use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

#[post("/login")]
async fn login(req_body: String) -> impl Responder {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret")?;
    let mut claims = BTreeMap::new();
    claims.insert("sub", "someone");
    let token_str = claims.sign_with_key(&key);
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(login))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
