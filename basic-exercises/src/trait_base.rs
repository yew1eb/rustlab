//类似于接口
pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> &u32;
}

pub struct Student {
    pub name: String,
    pub age: u32,
}

impl GetInformation for Student {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_age(&self) -> &u32 {
        &self.age
    }
}

struct Empty;
struct Null;

// 用到 `T` 的trait 泛型。
trait DoubleDrop<T> {
    // 定义一个关于调用者的方法，接受一个额外的单一参量 `T`，
    // 且没有任何操作。
    fn double_drop(self, _: T);
}

// 针对泛型参量 `T` 和调用者 `U` 实现了 `DoubleDrop<T>` 。
impl<T, U> DoubleDrop<T> for U {
    // 此方法获得了两个传入参数的所有权，并释放这两个参数。
    fn double_drop(self, _: T) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tmain() {
        let s = Student {
            name: "xiaoming".to_string(),
            age: 10,
        };

        println!("student, name = {}, age = {}", s.get_name(), s.get_age());
        println!("Hello, world!");
    }

    #[test]
    fn tmainx() {
        let empty = Empty;
        let null = Null;

        empty.double_drop(null);
    }
}
