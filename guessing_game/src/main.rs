use std::io;

// 猜数字游戏
fn main() {
    // println!("Hello, world!");
    println!("Guessing the number");

    println!("Please input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("Your guessed: {guess}");
}
