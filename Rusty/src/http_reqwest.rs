

pub async fn get_todos() {
    let res = reqwest::blocking::get("https://jsonplaceholder.typicode.com/todos").unwrap()
    .text();

    // println!("body = {res:?}");

    let client = reqwest::Client::new();
    let response = client.get("https://jsonplaceholder.typicode.com/todos/1").send();
    
}