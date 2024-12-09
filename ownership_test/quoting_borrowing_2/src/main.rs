fn main() {
    // println!("Hello, world!");
    let s = String::from("hello");

    quoting_test(&s);
}

fn quoting_test(s: &String){
    // s.push_str(",world"); 不合理，引用的值无法更改，因为没有所有权
    println!("s {}",s)
}
