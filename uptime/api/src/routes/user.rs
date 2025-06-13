use std::{collections::BTreeMap, fmt::format};

use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use mongodb::{bson::{doc, Document}, Collection};
use poem::{handler, http::StatusCode, web::Path, Request, Response, Result};
use serde::{Serialize, Deserialize};
use sha2::Sha256;
use store::connect_to_db;

#[derive(Debug, Serialize, Deserialize)]
struct SignUpStruct {
    user_name:String,
    email:String,
    password:String
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginStruct {
    user_name:String,
    password:String
}

#[handler]
pub async fn signup(req: &Request) -> Result<Response> {
    let body:Result<SignUpStruct> = req.into_body().into_json().await;
    let db = connect_to_db(std::env::var("DB_URL").unwrap()).unwrap();
    let user_collection:Collection<Document> = db.collection("users");
    match body {
        Ok(body) => {
            let _result = user_collection.insert_one(doc!{
                "user_name":body.user_name,
                "password":body.password,
                "email": body.email
            }).await;
            Ok(Response::builder().status(StatusCode::OK).body(format!("signup successfull")))
        },
        Err(_) => Err(poem::Error::from_string("failed to signup", StatusCode::INTERNAL_SERVER_ERROR))
    }
}

#[handler]
pub fn login(req: &Request) -> Result<Response> {
    let params = req.params::<LoginStruct>()?;
    
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"dipak123")
        .map_err(|_| poem::Error::from_string("Invalid HMAC key", StatusCode::INTERNAL_SERVER_ERROR))?;
    
    let mut claims = BTreeMap::new();
    claims.insert("user_name", params.user_name);
    let token_str = claims.sign_with_key(&key).map_err(|_| poem::Error::from_string("falied to generate token", StatusCode::INTERNAL_SERVER_ERROR));
    
    match token_str {
        Ok(token) => return Ok(Response::builder().status(StatusCode::OK).body(serde_json::to_string(&token).unwrap())),
        Err(_) => return Err(poem::Error::from_string("falied to generate token", StatusCode::INTERNAL_SERVER_ERROR))
    }
}

