fn main() {
    let a1 = String::from("dipak");
    let a2 = a1;
    println!("The value is a1", a2);
    println!("The value of a2", a1); // the a1 is  invalid here since the is only 1 owner of data in heap at a time which is in this case is a2
}
