use serde::{de::value::Error, Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize="camelCase"))]
struct User {
    id:usize,
    user_name:String,
    user_email:String
}

pub fn ser_user() {
    let user = User {
        id:1,
        user_name:"dipak".to_string(),
        user_email:"dipakhade214@gmail.com".to_string()
    };

    let user_json = to_string_pretty(&user).unwrap();
    // println!("{}", user_json);

    let json_str = r#"
        {
        "id": 1,
        "user_name": "dipak",
        "user_email": "dipakhade214@gmail.com"
        }
    "#;

    let user_struct = from_str::<User>(json_str);
    match user_struct {
        Ok(x) => println!("{:#?}", x),
        Err(e) => println!("{:#?}", e)
    }
    
}