//Lifetimes in Rust   :------- For Validating References
fn main() {
    let x;
    {
        let y = 21;
        x = &y;
        println!("value of x: {x}")
    }
}

// There are 3 Rules for lifetimes which borrow checker knows

// 1. compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

fn some_thing<'a>(x: &'a str) -> &'a str {
    x
}

fn another_thing<'a, 'b>(x: &'a i32, y: &'b i32) -> &'b i32 {
    y
}

// 2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters

fn any_fn<'a>(a: &'a f64) -> &'a f64 {
    a
}

// 3. if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
