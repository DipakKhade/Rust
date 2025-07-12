
fn main() {
    let mut p1 = std::process::Command::new("which");
    p1.arg("python3");
    let mut handler = p1.spawn().unwrap();

    println!("doing some work in spawned thred");

    let result = handler.wait().unwrap();

    println!("Output : {}", result.code().unwrap());
}