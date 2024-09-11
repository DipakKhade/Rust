fn main() {
    let n: i32 = 11;
    iseven(n);
}

fn iseven(n: i32) {
    if n % 2 == 0 {
        print!("{}", "even")
    } else {
        print!("{}", "odd")
    }
}
