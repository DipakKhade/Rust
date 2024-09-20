struct User<'a> {
    name: &'a str,
}
fn main() {
    let user_name = String::from("dipak");
    let user = User { name: &user_name };

    println!("{}", user.name)
}
