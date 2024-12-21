use reqwest::Error; // Add reqwest = { version = "0.11", features = ["blocking", "json"] } in Cargo.toml
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let todos = get_todos().await;
    println!("{:?}", todos);
    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct Todo {
    userId: i32,
    id: i32,
    title: String,
    completed: bool,
}

async fn get_todos() -> Result<Vec<Todo>, Error> {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/todos").await?;

    let todos: Vec<Todo> = response.json().await?;
    Ok(todos)
}

async fn complete_todo() ->Result<T,E> {
    let response = reqwest::get('').await?;

    let res = response::json().await?;
    Ok(res)
}
