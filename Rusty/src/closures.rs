
struct User {
    name:String,
    email:String
}

pub fn cloures() {
    let add = |a:i32, b:i32| {
        return a+b;
    };

    print!("sum of 3 and 5 is {}", add(3,5));

    let mut user = User {
        name: "dipak".to_string(),
        email:"dipak214@gmail.com".to_string()
    };

    let mut change_user_name = || user.name = "gaurav".to_string();
    change_user_name();

    println!("user name is {}", user.name);

}

pub static y:i32 = 5;
pub const d:i32 = 9;
