struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User{
        active:true,
        username:String::from("cui"),
        email:String::from("dasdad@gmail.com"),
        sign_in_count:1,
    };

    
}
