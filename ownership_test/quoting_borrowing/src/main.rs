fn main() {
    // println!("Hello, world!");
    let s1 = String::from("hello");

    let len = calculate_lenght(&s1);
    println!("The length of '{s1}' is {len}.");

}

fn calculate_lenght(s: &String)-> usize{
    s.len()
}