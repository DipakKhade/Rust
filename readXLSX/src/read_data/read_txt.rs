use std::fs;

pub fn read_txt(file_path:String)->String{
    let txt = fs::read_to_string(file_path).expect("unable to read a file");
    return txt;
}