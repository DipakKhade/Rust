use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();
    users.insert(String::from("Dipak"), 12);

    let first_user = users.get("asd");
    match first_user {
        Some(age) => println!("The age is {}", age),
        None => println!("No user with the provided  name"),
    };

    let v = vec![(String::from("gaurav"), 21), (String::from("omkar"), 22)];
    let ans = genrate_hashmap(&v);
    print!("{:?}", ans)
}

fn genrate_hashmap(v: &Vec<(String, i32)>) -> HashMap<&String, &i32> {
    let mut new_hash = HashMap::new();
    for (key, value) in v {
        new_hash.insert(key, value);
    }
    return new_hash;
}
