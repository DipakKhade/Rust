use std::fs::read_to_string;

fn main() {
    let result = read_to_string(path:"read.txt");

    match result {
        Ok(data)=>println!("This is a file data :{}",data),
        Err(err)=>println!("some error occured")
    }

}


