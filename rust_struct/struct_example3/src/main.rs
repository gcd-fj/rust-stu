struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the area of the rectangle is {} square pixels",area(&rect1));

    println!("Hello, world!");
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
