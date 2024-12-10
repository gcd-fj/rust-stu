fn main() {
    // println!("Hello, world!");
    let s = String::from("hello ,world");

    let s1 = &s;
    let word = first_word(s1);

    println!(" word {}",word)
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

