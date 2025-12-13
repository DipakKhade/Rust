use std::sync::{Arc, Mutex};
use mongodb::{Database, bson::{Document, doc}, Collection};
use poem::{error::BadRequest, handler, web::{Data, Json}};

use crate::types::auth::UserLoginPayload;

#[handler]
#[allow(unused_variables)]
pub async fn user_login(db: Data<&Arc<Mutex<Database>>>){    

    let c = Arc::try_unwrap(db.0.clone()).unwrap();

    let db = Mutex::into_inner(c).unwrap();

    let users: Collection<Document> = db.collection("users");

    let new_user = users.insert_one(doc! {
        "user_id": "dipak",
        "password": "dipak1234"
    }).await.unwrap();

}