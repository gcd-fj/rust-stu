
mod front_of_house {
    pub mod hosting {
       pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant1() {
    hosting::add_to_waitlist();
}

pub fn eat_at_restaurant2() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant3() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

pub fn eat_at_restaurant4() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant5() {
    add_to_waitlist();
}

use std::collections::HashMap;

#[allow(dead_code)]
fn test1() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::fmt;
use std::io;


// fn function1() -> fmt::Result {
//
// }

// fn function2() -> io::Result<()> {}
// use crate::a::b1;

pub mod a;
use crate::a::b1;
