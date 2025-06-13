
use mongodb::{ 
	bson::{doc, Document},
	Client,
	Collection, Database 
};

#[tokio::main]
pub async fn connect_to_db(connection_str:String) -> mongodb::error::Result<Database> {
    let client = Client::with_uri_str(connection_str).await?;
    let db = client.database("test");
    Ok(db)
}