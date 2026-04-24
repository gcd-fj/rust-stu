use std::io;

fn main() {
    test_array3();
}

#[allow(dead_code)]
fn test1() {
    // println!("Hello, world!");
    // let guess: u32 = "42".parse().expect("not a number");
    // warning: unused variable: `guess`
    //  --> src/main.rs:3:9
    //   |
    // 3 |     let guess: u32= "42".parse().expect("not a number");
    //   |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_guess`
    //   |
    //   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

    // warning: `no_type_annotations` (bin "no_type_annotations") generated 1 warning (run `cargo fix --bin "no_type_annotations" -p no_type_annotations` to apply 1 suggestion)
    //     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

#[allow(dead_code)]
fn test2() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}

#[allow(dead_code)]
fn test3() {
    let t = true;
    let f: bool = false;
}

#[allow(dead_code)]
fn test_char() {
    let c = 'z';
    let z = 'z';
    let heart_eyed_cat = '😻';
}

// 元组类型
#[allow(dead_code)]
fn test_tup() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
#[allow(dead_code)]
fn test_tup2() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("the type value y is {}", y);
}
#[allow(dead_code)]
fn test_tup3() {
    let tup = (890, 1, 234.2);
    // 元组位置可以赋值
    let ba90 = tup.0;
}
#[allow(dead_code)]
fn test_array() {
    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ]; // 不过当你明确元素数量不需要改变时，数组会更有用。例如，如果你在程序中使用月份的名称，你很可能希望使用的是数组而不是 vector，因为你知道它始终包含 12 个元素：

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // 这里，i32 是每个元素的类型。分号之后，数字 5 表明该数组包含 5 个元素。
    let a = [3; 5]; // 变量名为 a 的数组将包含 5 个元素，这些元素的值初始化为 3。这种写法与 let a = [3, 3, 3, 3, 3]; 效果相同，但更简洁。
}
#[allow(dead_code)]
fn test_array2() {
    // 数组是可以在栈上分配的已知固定大小的单个内存块。可以使用索引访问数组的元素，如下所示：
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
#[allow(dead_code)]
fn test_array3() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
