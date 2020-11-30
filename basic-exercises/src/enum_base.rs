enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Change(i32, i32, i32),
}

impl Message {
    fn prt(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move x= {}, y = {}", x, y),
            Message::Change(a, b, c) => println!("Change a = {}, b = {}, c = {}", a, b, c),
            //expected struct `std::string::String`, found reference
            // help: you can probably remove the explicit borrow: `s`
            //Message::Write(&s) => println!("{}", s),
            _ => println!("Write"),
        }
    }
}

enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tmain() {
        enum IpAddrKind {
            V4,
            V6,
        }

        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let i1 = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
        let i2 = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

        enum IpAddr2 {
            V4(String),
            V6(String),
        }

        let i1 = IpAddr2::V4(String::from("127.0.0.1"));
        let i2 = IpAddr2::V6(String::from("::1"));

        enum IpAddr3 {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let i1 = IpAddr3::V4(127, 0, 0, 1);
        let i2 = IpAddr3::V6(String::from("::1"));
    }

    #[test]
    fn t_message_enum() {
        let msg1 = Message::Quit;
        msg1.prt();

        let msg2 = Message::Move { x: 10, y: 30 };
        msg2.prt();
    }

    #[test]
    fn t_star_enum() {
        use Star::*;
        let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant];
        for star in starvec {
            match star as u32 {
                size if size <= 80 => println!("Not the biggest star."),
                size if size >= 80 => println!("This is a good-sized star."),
                _ => println!("That star is pretty big!"),
            }
        }
    }
}
