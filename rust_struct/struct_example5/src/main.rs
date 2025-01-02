struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };

    println!(
        "the area of the rectangle is {} suqare pixels",
        rect1.area()
    );
    println!("Hello, world!");
}
