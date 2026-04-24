fn main() {
    // test1();
    // test2();
    // test_shadow();
    // test3()
    test4()
}

#[allow(dead_code)]
fn test1() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);
}

#[allow(dead_code)]
fn test2() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);
}

#[allow(dead_code)]
fn test_shadow() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}

#[allow(dead_code)]
fn test3() {
    let x = "             ";
    println!("The value of x is: {}", x);

    let x = x.len();
    println!("The value of x is: {}", x);
}

#[allow(dead_code)]
fn test4() {
    let mut x = "       ";
    println!("The value of x is: {}", x);
    let x = x.len(); // 不允许这样操作， 可变变量不允许重新设置类型
    println!("The value of x is: {}", x);
}

// $ cargo check
// warning: variable does not need to be mutable
//   --> src/main.rs:49:9
//    |
// 49 |     let mut x = "       ";
//    |         ----^
//    |         |
//    |         help: remove this `mut`
//    |
//    = note: `#[warn(unused_mut)]` (part of `#[warn(unused)]`) on by default
//
// warning: `variables` (bin "variables") generated 1 warning (run `cargo fix --bin "variables" -p variables` to apply 1 suggestion)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s