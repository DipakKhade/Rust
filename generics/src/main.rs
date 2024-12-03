fn main() {
    let n = vec!["asd", "asd2", "asd3"];
    let ans = get_first_element(&n);
    match ans {
        Some(data) => println!("{}", data),
        None => println!("unable to find the first element"),
    }

    let rollno = vec![1, 2, 43, 230];
    let last_roll_no = get_last_element(&rollno);
    match last_roll_no {
        Some(data) => println!("The last element of vector is {}", data),
        None => println!("unable to find the last element"),
    }
}

fn get_first_element<T: std::cmp::PartialOrd>(a: &Vec<T>) -> Option<&T> {
    a.get(0)
}

fn get_last_element<T: std::cmp::PartialOrd>(a: &Vec<T>) -> Option<&T> {
    a.get(a.len() - 1)
}
