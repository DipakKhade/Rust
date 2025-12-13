use std::sync::{Arc, Mutex};
use poem::{EndpointExt, IntoResponse, Route, Server, get, handler, listener::TcpListener, post, web::{Json, Path}};
use dotenv::dotenv;
use crate::{auth::login::user_login, types::auth::DefaultMessage};
use db::mongo::Db;

pub mod auth;
pub mod types;
pub mod db;

#[handler]
fn entry_route() -> Json<DefaultMessage> {
    Json(DefaultMessage {
        message: "Welcome!, Login to procced".to_string()
    })
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let connection_string = std::env::var("DB_URL").unwrap();
    let db_var = Db::get_db_connection_pool(&connection_string).await;

    let db = Arc::new(Mutex::new(db_var.db));
    
    let app = Route::new().at("/", get(entry_route))
                                .at("/login", post(user_login))
                                .data(db.clone());

    Server::new(TcpListener::bind("0.0.0.0:3008"))
        .run(app)
        .await
}