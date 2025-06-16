use dotenv::dotenv;
pub struct Config {
    db_url: String
}

impl Default for Config {
    fn default() -> Self {
        dotenv().ok();
        let db_url = std::env::var("DATABASE_URL")
                            .unwrap_or_else(|_| panic!("missing DB_CONNECTION_STR"));
        Self {
            db_url
        }
    }
}