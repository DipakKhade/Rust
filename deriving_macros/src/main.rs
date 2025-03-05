trait Shape {
    fn area(&self) -> u32;
}

struct Rect {
    height: u32,
    width: u32,
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn get_area_for_any_shape(s: impl Shape) -> u32 {
    s.area()
}

// declerative macros
macro_rules! print_time {
    () => {
        println!("{:?}", std::time::SystemTime::now());
    };
}

// procedural macros
//
#[derive(Debug)]
struct Admin {
    name: String,
    id: u32,
}

pub macro proce_macro() {}

fn main() {
    let vec = vec![1, 2, 3];
    println!("vev :{:?}", vec);
    print_time!();
    let admin = Admin {
        id: 1,
        name: String::from("dipak"),
    };

    println!("{:?}", admin);
}
