use std::collections::HashMap;

fn main() {
    let mut user = HashMap::new();
    user.insert(String::from("name"), String::from("gaurav"));
    println!("{:?}", user);

    let default = String::from("no value corresponding to this key");
    let name = user.get(&String::from("name")).unwrap_or(&default);
    print!("{name}");

    let value = user.get(&String::from("nameasd"));
    match value {
        Some(v) => println!(" {v}"),
        None => println!(" no value is found"),
    };

    for (key, value) in &user {
        println!("key is =>{}", key);
        println!("value is ==>{}", value)
    }
}
