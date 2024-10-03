#[derive(Debug)]
struct Point<T: std::cmp::PartialOrd> {
    x: T,
    y: T,
}

fn main() {
    let point = Point { x: 12, y: 8 };
    println!("{point:?}");
}
