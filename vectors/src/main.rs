fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{:?}", vec);
    let even_v = even_values(vec);
    println!("{:?}", even_v)
}

fn even_values(v: Vec<i32>) -> Vec<i32> {
    let mut even_vec = Vec::new();
    for i in v {
        if i % 2 == 0 {
            even_vec.push(i);
        }
    }
    return even_vec;
}
