fn main() {
    println!(
        "The 'a' occurs at this index: {:?}",
        get_index(String::from("Dipak"))
    );
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
