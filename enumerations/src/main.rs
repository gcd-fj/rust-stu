fn main() {
    // println!("Hello, world!");
    // test();
    test2()
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[allow(dead_code)]
fn test() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);


    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("{:#?}", home);

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:#?}", loopback);



}

#[allow(dead_code)]
fn route(ip_type: IpAddrKind) {
    println!("{:?}", ip_type);
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[allow(dead_code)]
fn test2() {
    let home = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum::V6(String::from("::1"));

    println!("home: {:?} loopback: {:?}",home,loopback)
}

struct Ipv4Addr {

}

struct Ipv6Addr {

}

enum IPADDR {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}