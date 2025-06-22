use std::sync::Mutex;


pub fn mutal_exclusive() {
    let n = Mutex::new(7);

    println!("value of mutex before mutation {:?}", n);

    let mut d = 10;

    {
        let mut num = n.lock().unwrap();
        *num = 8;

        println!("value of num {}", num);
        drop(num);

        d = 12;

    }

    println!("value of mutex {:?}", n);
    println!("value of d {:?}", d);


    std::thread::spawn(move || {
        let mut asd = n.lock().unwrap();
        *asd = 99;

    }).join();

}
