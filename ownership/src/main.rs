fn main() {
    // println!("Hello, world!");
    // test();
    // test_function();
    // test3();
    // test_tuple();
    // test4();
    // test7();
    slice();
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

// 引用借用
#[allow(dead_code)]
fn calculate_length(s: String) -> (String,usize) { // 引用
    let length = s.len();
    (s, length)

}

#[allow(dead_code)]
fn test4() {
    let s = String::from("hello");
    let length = calculate_length2(&s);
    println!("the length of '{}' is {}.", s, length);
}

// 借用
#[allow(dead_code)]
fn calculate_length2(s: &String) -> usize {
    s.len()
}

#[allow(dead_code)]
fn test5() {
    // let s = String::from("hello");

    let mut s = String::from("hello");
    change(&mut s);
    println!("s: {}", s);
}

#[allow(dead_code)]
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

#[allow(dead_code)]
fn test6() {
    let mut s = String::from("hello");
    let s1 = &mut s;
    // let s2 = &mut s; // 不支持多个可变引用
    println!("s1: {}", s1);

    let s3 = &s;
    let s4 = &s;// 可支持多个不可变引用

    println!("s3: {}, s4: {}", s3, s4);
    // println!("s1: {}", s1); 不能这样干：s3,s4 已经借走变量，s1 生命周期结束

    // println!("s1: {}, s2: {}", s1, s2);
}

#[allow(dead_code)]
fn test7() {
    let s = dangle();
    println!("s: {}", s);
}

// #[allow(dead_code)]
// fn dangle() -> &String { 此引用会指向一个无效引用，因为生命周期已结束
//
//     let s = String::from("hello");
//     &s
//
// }

#[allow(dead_code)]
fn dangle() -> String { // 所有权被转移
    let s = String::from("hello");
    s
}

#[allow(dead_code)]
fn slice() {
    // 另一个没有所有权的数据类型是 slice。slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。
    // 字符串 slice（string slice）是 String 中一部分值的引用
    let s = String::from("hello world");
    let hello = &s[0..5];
    println!("hello: {}", hello);

    let world = &s[6..11];
    println!("world: {}", world);


    let s = String::from("hello world");
    let slice = &s[0..2];
    println!("slice: {}", slice);
    let slice = &s[..2];
    println!("slice: {}", slice);

    let len = s.len();
    let slice = &s[3..len];
    println!("slice: {}", slice);
    let slice = &s[3..];
    println!("slice: {}", slice);
}