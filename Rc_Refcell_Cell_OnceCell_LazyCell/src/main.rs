use std::{cell, rc::Rc};

#[derive(Debug)]
struct Node {
    pub id: cell::Cell<i32>,
    pub points: Option<Rc<Node>>,
}

fn main() {
    //RC
    //https://doc.rust-lang.org/rust-by-example/std/rc.html

    //Cell
    //https://doc.rust-lang.org/std/cell/

    let node1 = Rc::new(Node {
        id: cell::Cell::new(1),
        points: None,
    });

    let node2 = Rc::new(Node {
        id: cell::Cell::new(2),
        points: Some(Rc::clone(&node1)),
    });

    let node3 = Rc::new(Node {
        id: cell::Cell::new(3),
        points: Some(Rc::clone(&node1)),
    });

    dbg!(Rc::strong_count(&node1));

    cell::Cell::set(&node1.id, 10);
    
    dbg!(node1);

     let mut a = String::from("value");
    let b = a;

    // println!("{}",a);

    let c = std::rc::Rc::new(String::from("value rc"));

    let d = std::rc::Rc::clone(&c);

    println!("c is {} and d is {}", c, d);
    println!("reference count {}", std::rc::Rc::strong_count(&c));

}

