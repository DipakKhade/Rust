use actix_web::{post, web, HttpResponse};

struct sign_in_payload {
    name: String,
    email: String,
}

pub fn sign_in(data: web::Data<sign_in_payload>) {
    let name = &data.name;
    let email = &data.email;

    if !name || !email {
        panic!("invalid cred");
    }
    let jwt = String::from("asdasd312");
    HttpResponse::Ok()
        .contant_type(ContantType::json())
        .body(jwt);

    HttpResponse::Ok().body(jwt)
}
