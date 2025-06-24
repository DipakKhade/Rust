use std::sync::mpsc;

pub fn mpsc_test() {
    let (tx, rx) = mpsc::channel::<i32>();

    std::thread::spawn( move|| {
        let id = tx.send(2);
    }).join().unwrap();

    for i in rx.iter() {
        println!("{}", i);
    }
}