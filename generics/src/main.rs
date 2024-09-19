fn main() {
    let n = vec!["asd", "asd2", "asd3"];
    let ans = get_first_element(&n);
    match ans {
        Some(data) => println!("{}", data),
        None => println!("unable to find the first element"),
    }
}

fn get_first_element<T: std::cmp::PartialOrd>(a: &Vec<T>) -> Option<&T> {
    a.get(0)
}
