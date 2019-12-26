fn main() {
    let s = String::from("Hello world!");
    let w = first_word(&s);
    println!("{}", &w);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
