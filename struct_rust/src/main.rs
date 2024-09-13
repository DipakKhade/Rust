fn main() {
    let room = Rect {
        width: 30,
        height: 20,
    };

    println!(" the area of the reactangle is {}", room.area());
}

struct Rect {
    height: u32,
    width: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}
