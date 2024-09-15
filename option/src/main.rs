fn main() {
    println!(
        "The 'a' occurs at this index: {:?}",
        get_index(String::from("Dipak"))
    );
    print!("{:?}", name(String::from("Dipak")));
    print!("{:?}", find_first_word(String::from("This is a sentence")))
}

// where ever you want to return a null from rust function use a Option enum

fn get_index(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }

    return None;
}

fn name(s: String) -> Option<bool> {
    if s == "Dipak" {
        return Some(true as bool);
    }
    return None;
}

fn find_first_word(s: String) -> Option<String> {
    if s.len() > 0 {
        return s.split(' ').collect();
    };

    return None;
}
