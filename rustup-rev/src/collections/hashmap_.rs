use std::collections::HashMap;

pub fn hashmap_iter(hasmap: HashMap<&str, &str>) {
    for (key, value) in hasmap {
        println!("key : {}", key);
        println!("value : {}", value);
    }
}
