use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 2, 3];

    // for i in v {
    //     println!("value : {}", i)
    // }

    // let v_iter = v.iter();
    // for val in v_iter {
    //     println!("{}", val)
    // }

    // let v_iter_mut = v.iter_mut();
    // for i in v_iter_mut {
    //     println!("{}", i);
    // }

    // let mut v_mut_iter = v.iter_mut();
    // while let Some(val) = v_mut_iter.next() {
    //     println!(" : {}", val)
    // }

    let v_into_iter = v.into_iter();
    for i in v_into_iter {
        println!("{}", i)
    }

    //adapters

    //consuming adapters
    let v2 = vec![1, 2, 3, 4, 5];
    let v2_iter = v2.iter();
    let total: i32 = v2_iter.sum();
    println!("{}", total);

    //Iterator adaptors
    //adaptor that returns a new iterator
    //map
    let v3 = vec![11, 12, 13];
    let iter = v3.iter();
    let iter2 = iter.map(|x| x + 1);
    for i in iter2 {
        println!("{}", i)
    }

    //filter
    let v4 = vec![10, 20, 30, 31, 32, 42];
    let v4_iter = v4.iter();
    let v4_iter_2 = v4_iter.filter(|x| *x % 2 == 0);
    for i in v4_iter_2 {
        print!("{} ", i)
    }

    //
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let values_iter = values.iter();
    let values_iter2 = values_iter.filter(|x| **x % 2 == 0);
    let mut double_values = Vec::new();
    for i in values_iter2 {
        double_values.push(i * 2);
    }
    // or converting a itor back to vector
    // let double_values: Vec<i32> = values_iter2.collect();
    print!(
        "The vector with filtered odd values and doubled values is {:?}",
        double_values
    );

    let users = vec![("dipak", 12), ("gaurav", 13)];
    let mut users_hashmap: HashMap<String, i32> = HashMap::new();
    let users_iter = users.into_iter();
    for (name, age) in users_iter {
        users_hashmap.insert(name.to_owned(), age);
    }
    println!("{:?}", users_hashmap)
}
