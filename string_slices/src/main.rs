fn main() {
    let name = String::from("Dipak Khade");
    let first_name = get_first_word(&name);

    println!("{}", first_name);
}

fn get_first_word(s: &String) -> &str {
    let mut index = 0;
    for (_, i) in s.chars().enumerate() {
        if i == ' ' {
            break;
        };
        index = index + 1
    }

    return &s[0..index];
}
