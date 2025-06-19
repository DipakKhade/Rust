use std::{error::Error, fs::File};


fn main() {
    //The Box<T> smart pointer in Rust is used for heap allocation.
    let a = Box::new(5);

}

fn open_file() -> Result<File, Box<dyn Error>> {
    let file = File::open("file_path.txt")?;
    Ok(file)
}