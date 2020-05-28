enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrV2 {
    V4(String),
    V6(String),
}

enum IpAddrV3 {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let homeV2 = IpAddrV2::V4(String::from("127.0.0.1"));
    let loopbackV2 = IpAddrV2::V6(String::from("::1"));

    let homeV3 = IpAddrV3::V4(127, 0, 0, 1);
    let loopbackV3 = IpAddrV3::V6(String::from("::1");
}
