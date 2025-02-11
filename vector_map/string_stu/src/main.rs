fn main() {
    let data = "initial contents";

    let s = data.to_string();
    println!("{}", s);
    // 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();
    println!("{}", s);


    // 也可以使用 String::from 函数来从字符串字面值创建 String：
    let s = String::from("initial contents");

    // 打印s
    println!("{}", s);

    let hello = String::from("السلام عليكم");
    println!("{}, world!", hello);
    let hello = String::from("Dobrý den");
    println!("{}, world!", hello);
    let hello = String::from("Hello");
    println!("{}, world!", hello);
    let hello = String::from("שלום");
    println!("{}, world!", hello);
    let hello = String::from("नमस्ते");
    println!("{}, world!", hello);
    let hello = String::from("こんにちは");
    println!("{}, world!", hello);
    let hello = String::from("안녕하세요");
    println!("{}, world!", hello);
    let hello = String::from("你好");
    println!("{}, world!", hello);
    let hello = String::from("Olá");
    println!("{}, world!", hello);
    let hello = String::from("Здравствуйте");
    println!("{}, world!", hello);
    let hello = String::from("Hola");
    println!("{}, world!", hello);

    // 打印hello、S
    println!("{}, world!", hello);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);



    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);

    // error
    // let s1 = String::from("hello");
    // let h = s1[0];


    for c in "Зд".chars() {
        println!("{c}");
    }
    

    println!("Hello, world!");
}
