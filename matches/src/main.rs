fn main() {
    // println!("Hello, world!");
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// 如果调用 value_in_cents(Coin::Quarter(UsState::Alaska))，
// coin 将是 Coin::Quarter(UsState::Alaska)。
// 当将值与每个分支相比较时，没有分支会匹配，直到遇到 Coin::Quarter(state)。
// 这时，state 绑定的将会是值 UsState::Alaska。
// 接着就可以在 println! 表达式中使用这个绑定了，
// 像这样就可以获取 Coin 枚举的 Quarter 成员中内部的州的值。
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(dead_code)]
fn test() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    // let num = six + five;
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[allow(dead_code)]
fn test2() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}
#[allow(dead_code)]
fn add_fancy_hat() {}

#[allow(dead_code)]
fn remove_fancy_hat() {}

#[allow(dead_code)]
fn move_player(num_spaces: u8) {}

#[allow(dead_code)]
fn reroll() {}

#[allow(dead_code)]
fn test3() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

#[allow(dead_code)]
fn test4() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}