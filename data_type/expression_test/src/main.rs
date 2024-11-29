fn main() {
    println!("Hello, world!");

    let y = {
        let x = 3;
        x + 1 // 返回值时不能带; 号
    };

    println!("The value of y is: {}",y)
}
