fn main() {
    // println!("Hello, world!");

    let mut s1 = String::from("hello");

    {
        let r1 = &mut s1;
        change(r1);
        println!(" s1 {}",s1)
    }
    change(&mut s1);

    println!("s1 {}",s1)
}

fn change(s: &mut String){
    s.push_str(",world");
}