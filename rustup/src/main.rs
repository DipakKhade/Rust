fn main() {
    println!("Hello, world!");
    let rev_arr = foo();
    println!("{:?}", rev_arr);
}

fn foo() {
    let arr = vec![1, 2, 3];
    for i in &arr {
        println!("{}", i)
    }
    let mut rev_arr = vec![];
    for i in 0..arr.len() {
        let l = arr.len();
        let curr_index = l - i;
        println!("{}", curr_index);
        rev_arr.push(arr[l - i]);
    }
    println!("{:?}", rev_arr)
    //return rev_arr;
}
