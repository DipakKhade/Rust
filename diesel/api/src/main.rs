use db::{models::User, schema};

#[tokio::main]
async fn main() {
    use schema::users::dsl::*;
    let connection = db::db::Db::new().unwrap();
    let users_data = users.select(User::as_select())
                        .load(connection)
                        .expect("Error loading users");

    for x in users_data {
        println!("{:?}", x.id);
    }
}