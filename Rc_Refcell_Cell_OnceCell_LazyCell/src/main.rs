use std::rc::Rc;

#[derive(Debug)]
struct Node {
    pub id: i32,
    pub points: Option<Rc<Node>>,
}

fn main() {
    //RC
    //https://doc.rust-lang.org/rust-by-example/std/rc.html
    let node1 = Rc::new(Node {
        id: 1,
        points: None,
    });

    let node2 = Rc::new(Node {
        id: 2,
        points: Some(Rc::clone(&node1)),
    });

    let node3 = Rc::new(Node {
        id: 3,
        points: Some(Rc::clone(&node1)),
    });

    dbg!(Rc::strong_count(&node1));

    //Cell
    //https://doc.rust-lang.org/std/cell/

}
