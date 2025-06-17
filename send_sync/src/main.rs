
/*
                Send and Sync
A type is Send if it is safe to send it to another thread.
A type is Sync if it is safe to share between threads (T is Sync if and only if &T is Send).
*/


fn main() {

    // Send and Sync with primitive types
    let x: std::sync::Arc<String> = std::sync::Arc::new(String::from("dipak")); 
    let y = 7;

    std::thread::spawn(move || {
        // println!("value x {:?}", x);
    }).join().expect("thread failed");

    let handlers = (1..5).map(|_| {
        let shared_x = std::sync::Arc::clone(&x);
        std::thread::spawn(move || {
            println!("shared value {:?}", shared_x);
        })
    });

    for h in handlers {
        h.join().expect("thread failed");
    }
}