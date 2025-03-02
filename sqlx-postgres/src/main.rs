use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

#[derive(Debug)]
struct User{
    id: u32,
    name:String
}


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL should be present");
    let pool = PgPoolOptions::new().max_connections(5).connect(&db_url).await?;

    let row = sqlx::query!("SELECT * FROM users WHERE id=1").fetch_one(&pool).await?;

    println!("{:?}",row);

    Ok(())
}

