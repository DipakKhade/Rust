fn main() {
    let x;
    {
        let y = 21;
        x = &y;
    }
    println!("value of x: {x}")
}
