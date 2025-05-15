use std::collections::BTreeMap;

fn main() {
    let mut v = std::collections::VecDeque::with_capacity(10);
    v.push_back(2);
    println!(" vec q queue {:?}", v);
    // v.append(other);
    // v.split_off(at)
    // v.as_mut_slices()
    // v.back()
    // v.back_mut()
    // v.binary_search(x)

    let mut  h = std::collections::HashMap::new();
    h.insert("name", "dipak");
    let insert_return = h.insert("name", "gaurav");
    println!("hash map : {:?}", insert_return);
    println!("hash map : {:?}", h);
    
    let entry_return = h.entry("name ");
    println!("entry api return {:?}", entry_return); 

    match entry_return {
        std::collections::hash_map::Entry::Occupied(key) => {
            println!("key already present --{:?}", key)
        }
        std::collections::hash_map::Entry::Vacant(key) => {
            println!("new key -- {:?}", key)
        }
    }

    // BTREE
    let b: BTreeMap<String, String> = std::collections::BTreeMap::new();


}