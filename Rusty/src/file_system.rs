use std::io::Write;



pub fn file_system() {
    // let new_file = std::fs::File::create_new("app.py").unwrap();

    // let dir = std::fs::create_dir("asd");

    let dirs = std::fs::read_dir("/Users/dipakkhade/projects/Rust/Rusty").unwrap();
    for f in dirs.into_iter() {
        match f {
            Ok(x) => {
                println!("dir location : {:?}", x);
                let is_file_exists = std::path::Path::new(&x.path()).exists();
                println!("is_file_exists-- {}", is_file_exists);
            },
            Err(e) => println!("error while reading dirs")
        }
    }





}