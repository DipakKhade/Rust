
#[derive(Debug)]
struct User {
    id:u32,
    name:String,
    email:String,
    admin:Admin
}

#[derive(Debug)]
struct Admin {
    admin_id:u32
}

impl Default for User {
    fn default() -> Self {
        User {
            id:1,
            name:"asd".to_string(),
            email:"dipak@gmail.com".to_string(),
            admin: Admin { admin_id: 2 }
        }
    }
}

struct FirstName(String);


fn main() {
    let user1_first_name = FirstName("dipak".to_string());

    let user1 = User {
        id:2,
        name:user1_first_name.0,
        ..Default::default()
    };
    dbg!(&user1);

}