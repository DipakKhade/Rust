///serde, borsh, lifetimes, solana program
use serde::{Serialize, Deserialize};
use borsh::{BorshDeserialize, BorshSerialize};

// serde
#[derive(Serialize, Deserialize, Debug)]
struct  User{
    name:String,
    password:String
}

// borsh
#[derive(BorshDeserialize, BorshSerialize)]
struct Employee{
    name:String,
    dept:String
}

fn main() {
    //serde
    let user2 = User{
        name:String::from("value"),
        password:String::from("pass")
    };
    let ser_user2 = serde_json::to_string(&user2).unwrap();
    println!("{}",ser_user2);
    
    let des_user1:User = serde_json::from_str(&ser_user2).unwrap();
    println!("the desc struct : {:?}",des_user1);


    //borsh
    let emp = Employee{
        name:String::from("asd"),
        dept:String::from("zxc")
    };

    let mut emp_byte_data:Vec<u8> = Vec::new();
    let emp_to_byte = emp.serialize(&mut emp_byte_data).unwrap();
    println!("emp_to_byte : {:?}", emp_to_byte)

}

fn get_longest_str<'a>(s1:&'a str, s2:&'a str) ->&'a str{
    if s1.len() > s2.len() {
         s1;
    }
    s2
}