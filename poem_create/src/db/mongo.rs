use mongodb::{ Client, Database};

pub struct Db {
    pub db: Database
}

impl Db {
    pub async fn get_db_connection_pool(connection_str: &str) -> Self {
        let client = Client::with_uri_str(connection_str).await.unwrap();
        let database: Database = client.database("rust_x"); 

        Self {
            db: database
        }
    }
}
