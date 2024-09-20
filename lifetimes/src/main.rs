fn main() {
    let ans;
    let s1 = String::from("Dipak");
    {
        let s2 = String::from("Gaurav");

        ans = bigger_str(&s1, &s2);
    }
    println!("{}", ans)
}

fn bigger_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a;
    }

    return b;
}
