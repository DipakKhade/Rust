use packages_and_crates::{login, UserCrentional};

fn main() {
    let user_credentionals = UserCrentional {
        email: String::from("dipakkhade214@gmail.com"),
        password: String::from("asd123"),
    };
    login(user_credentionals);
}
