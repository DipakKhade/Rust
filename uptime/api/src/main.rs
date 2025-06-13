use poem::{ get,post, listener::TcpListener, Route, Server};

use crate::routes::user::{login, signup};
use dotenv::dotenv;



pub mod routes;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let app = Route::new()
    .at("/login", get(login))
    .at("/signup", post(signup) );
    Server::new(TcpListener::bind("0.0.0.0:3000"))
    .run(app)
    .await
}