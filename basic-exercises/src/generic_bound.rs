use std::fmt::{Debug, Display};

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tmain() {
        let rectangle = Rectangle {
            length: 3.0,
            height: 4.0,
        };
        let _triangle = Triangle {
            length: 3.0,
            height: 4.0,
        };

        print_debug(&rectangle);
        println!("Area: {}", area(&rectangle));

        //print_debug(&_triangle);
        //println!("Area: {}", area(&_triangle));
        // ^ 试一试：将上述语句的注释去掉。
        // | 报错：未实现 `Debug` 或 `HasArea`。
    }
}

// ======= 空限定
struct Cardinal;

struct BlueJay;

struct Turkey;

trait Red {}

trait Blue {}

impl Red for Cardinal {}

impl Blue for BlueJay {}

// 这些函数只对实现了相应的 trait 的类型有效。实际情况中 trait 内部
// 是否为空都无所谓。
fn red<T: Red>(_: &T) -> &'static str {
    "red"
}

fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

#[cfg(test)]
mod testx {
    use super::*;

    #[test]
    fn tmain() {
        let cardinal = Cardinal;
        let blue_jay = BlueJay;
        let _turkey = Turkey;

        // 由于限定，`red()` 不能调用 blue_jay （蓝松鸟），
        // 反过来也一样。
        println!("A cardinal is {}", red(&cardinal));
        println!("A blue jay is {}", blue(&blue_jay));
        //println!("A turkey is {}", red(&_turkey));
        // ^ 试一试：将此行注释去掉。
    }
}

// ===== 多重限定
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: `{:?}`", t);
    println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: `{:?}", t);
    println!("u: `{:?}", u);
}

#[cfg(test)]
mod testxx {
    #[test]
    fn tmain() {
        use super::*;

        let string = "words";
        let array = [1, 2, 3];
        let vec = vec![1, 2, 3];

        compare_prints(&string);
        //compare_prints(&array);

        compare_types(&array, &vec);
    }
}
