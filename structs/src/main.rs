struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
//也可以定义与元组（在第 3 章讨论过）类似的结构体，称为元组结构体（tuple struct）。元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的，这时像常规结构体那样为每个字段命名就显得多余和形式化了。
//
// 要定义元组结构体，以 struct 关键字和结构体名开头并后跟元组中的类型。例如，下面是两个分别叫做 Color 和 Point 元组结构体的定义和用法：
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 我们也可以定义一个没有任何字段的结构体！它们被称为类单元结构体（unit-like structs），
// 因为它们类似于 ()，即“元组类型”一节中提到的 unit 类型。
// 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
// 我们将在第 10 章介绍 trait。下面是一个声明和实例化一个名为 AlwaysEqual 的 unit 结构的例子。
struct AlwaysEqual;


fn main() {

}

#[allow(dead_code)]
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

#[allow(dead_code)]
fn test1() {
    let mut user1 = User {
        active: true,
        username: "someusername".to_string(),
        email: "".to_string(),
        sign_in_count: 1,
    };
    user1.email = String::from("newemail@example.com");
    println!("The new email address is: {}", user1.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {:?}", black.0);
    println!("origin: {:?}", origin.0);

    let sub = AlwaysEqual;
    println!("AlwaysEqual instance created");
}

