fn main() {
    // println!("Hello, world!");
    // assert!(0.1 + 0.2 == 0.3);
    // let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    // let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    // println!("abc (f32)");
    // println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    // println!("         0.3: {:x}", (abc.2).to_bits());
    // println!();

    // println!("xyz (f64)");
    // println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    // println!("         0.3: {:x}", (xyz.2).to_bits());
    // println!();

    // assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);
    // test1();
    for i in 'a'..='z' {
        println!("{}",i);
    }
    
}


fn test1(){

    // 无符号8位整数，二进制为00000010
    let a: u8 = 2; // 也可以写 let a: u8 = 0b_0000_0010;

    // 二进制为00000011
    let b: u8 = 3;

    // {:08b}：左高右低输出二进制01，不足8位则高位补0
    println!("a value is        {:08b}", a);

    println!("b value is        {:08b}", b);

    println!("(a & b) value is  {:08b}", a & b);

    println!("(a | b) value is  {:08b}", a | b);

    println!("(a ^ b) value is  {:08b}", a ^ b);

    println!("(!b) value is     {:08b}", !b);

    println!("(a << b) value is {:08b}", a << b);

    println!("(a >> b) value is {:08b}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {:08b}", a);
}