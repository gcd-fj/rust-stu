// use std::alloc;

fn main() {
    // println!("Hello, world!");
    // another_function();
    // another_another_2(5);
    // another_another_3(5, 'h');
    // test1();
    // test2();
    test3();
}

#[allow(dead_code)]
fn another_function() {
    println!("Another function!");
}

#[allow(dead_code)]
fn another_another_2(x: i32) {
    println!("the value x is: {}", x);
}

#[allow(dead_code)]
fn another_another_3(x: i32, unint_label: char) {
    println!(
        "the value x is: {} and the unint_label is: {}",
        x, unint_label
    )
}

#[allow(dead_code)]
fn test1() {
    let y = {
        let x = 5;
        x * 2 // 返回值不能带分号
    };
    println!("y is: {}", y);
}

#[allow(dead_code)]
fn five() -> i32 {
    5
}

#[allow(dead_code)]
fn test2() {
    let x = five();
    println!("x is: {}", x);
}

#[allow(dead_code)]
fn plus_one(x: i32) -> i32 {
    x + 1 // 返回值不能带分号
}

#[allow(dead_code)]
fn test3() {
    let x = plus_one(5);
    println!("x is: {}", x);
}
