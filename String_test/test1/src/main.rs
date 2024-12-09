fn main() {

    let mut s = String::from("hello");
    println!("s: {}",s);
    s = String::from("test");
    println!("s: {}",s);

    let mut s2= String::from("hello");
    s2 = s2.replace("hello", "test");
    println!("s2: {}",s2);
}
