macro_rules! avec {
    ($x:ident) => {
        $x += 1
    };
}

#[warn(dead_code)]
fn foo() {
    let mut x = 2;
    avec!(x);
}

// micros in rust library
//

macro_rules! four {
    () => {
        1 + 3
    };
}

macro_rules! capture_then_match_tokens {
    ($e:expr) => {
        match_tokens!($e)
    };
}

macro_rules! match_tokens {
    ($a:tt + $b:tt) => {
        "got an addition"
    };
    (($i:ident)) => {
        "got an identifier"
    };
    ($($other:tt)*) => {
        "got something else"
    };
}

fn main() {
    println!(
        "{}\n{}\n{}\n",
        match_tokens!((caravan)),
        match_tokens!(3 + 6),
        match_tokens!(5),
    );
    println!(
        "{}\n{}\n{}",
        capture_then_match_tokens!((caravan)),
        capture_then_match_tokens!(3 + 6),
        capture_then_match_tokens!(5),
    );
}
