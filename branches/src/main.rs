fn main() {
    // println!("Hello, world!");
    // test_if();
    // test_if_else_if();
    // test_let_if();
    // test_loop();
    test_loop_label();
}

#[allow(dead_code)]
fn test_if() {
    let number = 5;
    if number < 10 {
        println!("number is less than 10");
    } else {
        println!("number is greater than or equal to 10");
    }
}

#[allow(dead_code)]
fn test_if_else_if() {
    let number = 6;
    // if number < 10 {
    //     println!("number is less than 10");
    // } else if number < 20 {
    //     println!("number is less than 20");
    // } else {
    //     println!("number is greater than or equal to 20");
    // }
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4 or 3 or 2");
    }
}

#[allow(dead_code)]
fn test_let_if() {
    let condition = true;

    let number = if condition {
        5
    } else {
        10 // if else 值类型必须相同
    };

    println!("number: {}", number);
}

#[allow(dead_code)]
fn test_loop() {
    let mut count = 0;
    loop {
        count += 1;
        if count == 10 {
            break;
        }
    }
    println!("count: {}", count);
}

#[allow(dead_code)]
fn test_loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count: {}", count);

        let mut remaining = 10;

        loop {
            println!("remaining: {}", remaining);

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("count: {}", count);
}
