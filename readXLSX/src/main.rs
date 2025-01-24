pub mod read_data;
use crate::read_data::read_txt::read_txt;

fn main(){
    // read_xlsx();
    let data = read_txt(String::from("txt.txt"));
    println!("{}",data)
    
}


