fn main() {

    let v: Vec<i32> = Vec::new();
    print!("{:?}", v);
    let v = vec![1, 2, 3];
    print!("{:?}", v);
    let mut v = Vec::new();
    v.push(1);

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);


    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }


    // let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    // // println!("{:?}", does_not_exist);

    // let does_not_exist = v.get(100);
    // // print does_not_exist
    // println!("{:?}", does_not_exist);



    println!("Hello, world!");
}
