use std::ops::Mul;

fn square<T: Mul<T, Output=T>>(x: T, y: T) -> T {
    x * y
}


fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { return a;}
    return gcd(b, a % b);
}


#[derive(Debug)]
struct User {
    name: &'static str,
    avatar_url: &'static str,
}

impl User {
    fn show(&self) {
        println!("name: {:?} ", self.name);
        println!("avatar: {:?} ", self.avatar_url);
    }
}

fn main() {
    let g = gcd(60, 40);
    assert_eq!(20, g);

    let a: i32 = square(37, 41);
    let b: f64 = square(37.2, 41.1);

    
    let user = User {
        name: "alex",
        avatar_url: "https://xxxx"
    };
    user.show();
}
