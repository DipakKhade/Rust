pub trait Summary {
    fn summarise(&self) -> String;
}

struct User {
    name: String,
    age: i32,
}

impl Summary for User {
    fn summarise(&self) -> String {
        return format!("The user {} has a age {}", self.name, self.age);
    }
}

fn main() {
    let user = User {
        name: String::from("Dipak"),
        age: 12,
    };

    println!("{}", user.summarise());
}
