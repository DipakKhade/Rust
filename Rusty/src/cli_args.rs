use std::{env::{args, Args}, num::ParseIntError};

pub fn cli_args() {
    println!("trying to parse -- {:?}", "7".parse::<i32>());

    let args = args();
    println!("{:?}", args);

    let total_args = args.len();
    println!(" args passed {}", total_args);

    let name = args.skip(1).take(1).next().unwrap();
    println!("arg name is - {}", name);
}