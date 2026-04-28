fn main() {
    // // println!("Hello, world!");
    // let width1 = 30;
    // let height1 = 50;
    // println!("The area of the rectangle is {} square pixels.", area(width1, height1));

    // test1();
    test2();
}

#[allow(dead_code)]
fn area(width: u32, height: u32) -> u32 {
    width * height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
fn test1() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", rect.width * rect.height);
    println!("The area of the rectangle is {} square pixels.", area2(&rect));
    println!("rect: {:?}", rect);
    println!("rect: {:#?}", rect);
}

fn area2(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

#[allow(dead_code)]
fn test2() {
    let scale = 2;
    let rect = Rectangle{
        width: dbg!(30*scale),
        height: 50,
    };

    dbg!(&rect);
}