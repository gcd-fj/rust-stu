fn main() {
    // println!("Hello, world!");
    // test();
    // test_function();
    // test3();
    test_tuple();
}


#[allow(dead_code)]
fn test() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("s: {}", s);
}

#[allow(dead_code)]
fn test1() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1: {}, s2: {}", s1, s2); s1 所有权已转移给 s2
    println!("s2: {}", s2);
}

#[allow(dead_code)]
fn test2() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s2 拥有 s1 的独立副本
    println!("s1: {}, s2: {}", s1, s2);
}

#[allow(dead_code)]
fn test_function(){

    let s = String::from("hello");
    takes_ownership(s);
    // println!("s: {}", s); 所有权已经转移

    let x = 5;
    makes_copy(x);
    println!("x: {}", x); // 所有权未转移
}

#[allow(dead_code)]
fn takes_ownership(some_string: String){ // some_string 所有权转移给函数
    println!("some_string: {}", some_string);
}// some_string 所有权在函数结束时被释放

#[allow(dead_code)]
fn makes_copy(some_integer: i32){ // some_integer 所有权不转移
    println!("some_integer: {}", some_integer);
} // some_integer 移出作用域，不会有特殊操作


#[allow(dead_code)]
fn test3() {
    let s1 = String::from("hello");
    let s2 = gives_ownership();
    println!("s1: {}, s2: {}", s1, s2);
    let s3 = takes_and_gives_back(s2);
    // println!("s1: {}, s2: {}", s1, s2); s2 所有权已转移
    println!("s3: {}", s3);
}

#[allow(dead_code)]
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}


#[allow(dead_code)]
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

#[allow(dead_code)]
fn test_tuple() {
    let s = String::from("hello");
    let (s_length, s_length2) = calculate_length(s);
    println!("s_length: {}, s_length2: {}", s_length, s_length2);
}

#[allow(dead_code)]
fn calculate_length(s: String) -> (String,usize) {
    let length = s.len();
    (s, length)

}