use std::ops::Sub;


fn main() {
    let d = std::time::Duration::from_secs(3);
    println!("{d:?}");

    let sub_d = d.sub(std::time::Duration::from_secs(1));
    println!("{sub_d:?}");

    let start_time = std::time::Instant::now();
    println!("{start_time:?}");

    std::thread::sleep(std::time::Duration::from_secs(1));

    let end_time = start_time.elapsed();
    println!("{end_time:?}");

    let todays_date = chrono::Utc::now();
    println!("date is -- {todays_date}");

}
