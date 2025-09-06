use std::string;

use axum::{
    extract::Query, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new().route("/", get(get_user_data)).route("/get_user_name", get(get_user_name));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    id: i32,
}

async fn get_user_data() -> Json<User> {
    Json(User {
        id: 1,
        name: String::from("dipak"),
    })
}

#[derive(Serialize, Deserialize, Debug)]
struct GetUser {
    id: i32
}

impl std::fmt::Display for GetUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self)
    }
    
}

async fn get_user_name(Query(user_id): Query<GetUser>)-> String {
    println!("user_id--{}", user_id);
    let id = user_id.id;
    println!("user id -- {}", id);

    String::from("dipak")
}