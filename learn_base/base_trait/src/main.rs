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

fn main() {
    let s = Student { name: "xiaoming".to_string(), age: 10 };

    println!("student, name = {}, age = {}", s.get_name(), s.get_age());
    println!("Hello, world!");
}
