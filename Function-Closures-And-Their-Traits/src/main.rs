fn main() {
    let x = bar;

    let y = |x: i32, y: i32| x + y;
    let sum = y(2, 4);
    println!("sum is :{}", sum);
}

fn bar() {}

fn baz(f: fn(u32) -> u32) {}
