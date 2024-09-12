fn main() {
    let n: i32 = 11;
    iseven(n);

    println!("{}", fib(4));

    print!(
        "The length of the dipak as a string is {}",
        get_str_length("dipak")
    )
}

// even odd numbers
fn iseven(n: i32) {
    if n % 2 == 0 {
        print!("{}", "even")
    } else {
        print!("{}", "odd")
    }
}

// fibonachi sequence

fn fib(n: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if n == 0 {
        return first;
    }
    if n == second {
        return second;
    }

    for _ in 0..(n - 1) {
        let _temp = first;
        second = first + second;
        first = second
    }

    return second;
}

//length of string

fn get_str_length(s: &str) -> usize {
    s.chars().count()
}
