
trait Printable {
    fn print_it(&self) -> bool {
        true
    }
}

trait Clonable: Printable {
    
}

impl Printable for i32 {
    fn print_it(&self) -> bool {
        true
    }
}

impl Printable for String {
    fn print_it(&self) -> bool {
        true
    }
}

impl Clonable for i32 {

}

fn main() {
    let v = std::vec::Vec::new() as Vec<Box<dyn Printable>>;
}
