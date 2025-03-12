#[derive(Debug)]
struct User {
    name: String,
    email: String,
    id: i32,
    is_admin: bool,
}

trait UserTrait {
    fn change_email(&mut self, new_email: String) {}
}

impl UserTrait for User {
    fn change_email(&mut self, new_email: String) {
        self.email = new_email
    }
}

fn main() {
    let mut user1 = User {
        name: String::from("dipak"),
        email: String::from("dipak214@gmail.com"),
        id: 1,
        is_admin: true,
    };

    println!("{:?}", user1);

    user1.change_email(String::from("newmail@gmail.com"));

    println!("{:?}", user1);

    let desc = String::from("hello, good morning");
    println!("{:?}", desc.find("good"));
    println!("{:?}",)
}

fn get_first_ele<'T>(arr: &'T Vec<i32>) -> Option<&'T i32> {
    if arr.len() == 0 {
        None
    } else {
        Some(&arr[0])
    }
}

