use collections::hashmap_::hashmap_iter;
use std::collections::HashMap;
pub mod collections;

#[derive(Debug)]
struct Doc {
    title: String,
    desc: String,
    author: String,
    is_public: bool,
}

fn main() {
    let mut docs = vec![
        Doc {
            title: String::from("ETH"),
            desc: String::from("impermanent loss in ethreum"),
            author: String::from("dipak"),
            is_public: true,
        },
        Doc {
            title: String::from("SOL"),
            desc: String::from("impermanent loss in ethreum"),
            author: String::from("dipak"),
            is_public: true,
        },
        Doc {
            title: String::from("BTC"),
            desc: String::from("impermanent loss in ethreum"),
            author: String::from("dipak"),
            is_public: true,
        },
    ];

    let updated_docs: Vec<String> = docs.iter_mut().map(|i| i.title.clone()).collect();
    // println!("{:?}",updated_docs);

    // let desc_collection: Vec<Vec<&str>> = docs
    //    .iter()
    //  .map(|i| i.desc.clone().split(" ").collect())
    //.collect();
    let desc_collection: Vec<Vec<String>> = docs
        .iter()
        .map(|i| i.desc.split(" ").map(|j| String::from(j)).collect())
        .collect();
    // println!("{:?}",desc_collection);

    let mut docs_iter = docs.iter_mut();

    for doc in docs_iter {
        doc.author = String::from("gaurav");
        println!("{:?}", doc)
    }
    let mut user_keys = HashMap::new();
    user_keys.insert("user1", "asa78721kniudaukad089");
    hashmap_iter(user_keys);

    let mut v = vec![1, 2, 3, 10, 12, 13];
    let mut v_iter = v.iter_mut();
    while let Some(val) = v_iter.next() {
        println!("{}", val);
    }

    // consumable adapter
    //
    // adapter are something that we use on the Iterrators
    let mut even = vec![0, 2, 4, 6, 8, 10];
    let even_sum: i32 = even.iter().sum();
    println!("sum is this :{}", even_sum);

    // Iterrators adapter

    let odd = vec![1, 3, 5, 7, 9, 11];
    let odd_plus_one = odd.iter().map(|x| x + 1);
    let asd_iter_collector: Vec<i32> = odd_plus_one.collect();
    println!("{:?}", asd_iter_collector);

    // filter
    //
    let nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let final_vec: Vec<i32> = nums
        .iter()
        .filter(|i| **i % 2 != 0)
        .map(|j| j * 2)
        .collect();
    println!("Final vec : {:?}", final_vec);

    let mut hash: HashMap<String, i32> = HashMap::new();
}
