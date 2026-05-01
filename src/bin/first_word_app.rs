fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        println!("{i} -- {item}");
        if item == b' ' {
            println!("The string: {}", s.len());
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
}