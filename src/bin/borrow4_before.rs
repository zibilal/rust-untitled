fn main() {
    let mut s = String::from("hello word");
    let word = first_word(&s);
    s.clear();
    let first_word = &s[..word];
    println!("{first_word}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}