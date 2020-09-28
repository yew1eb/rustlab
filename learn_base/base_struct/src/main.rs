struct Dog {
    name: String,
    weight: f32,
    height: f32,
}

fn main() {
    //struct_fn();

    struct_with_function();
}

fn struct_fn() {
    #[derive(Debug)]
    struct User {
        name: String,
        count: String,
        nonce: u64,
        active: bool,
    }

    let xiaoming = User {
        name: String::from("xiaoming"),
        count: String::from("8000100"),
        nonce: 10000,
        active: true,
    };

    let mut xiaohuang = User {
        name: String::from("xiaohuang"),
        count: String::from("80001001"),
        nonce: 10000,
        active: true,
    };

    xiaohuang.nonce = 20000;

    //参数名字和字段名字同名可以简写构造


    //从其他结构体创建实例
    let user2 = User {
        name: String::from("user2"),
        ..xiaohuang
    };

    println!("user2.name = {}", user2.name);
    println!("user2.nonce = {}", user2.nonce);
    println!("xiaohuang.nonce = {}", xiaohuang.nonce);


    // 元组结构体
    struct Point(i32, i32);
    let a = Point(10, 20);
    let b = Point(30, 11);

    println!("a.x = {}, a.y = {}", a.0, a.1);

    struct A {};

    //打印结构体
    println!("xiaoming = {:?}", xiaoming);
    println!("xiaoming = {:#?}", xiaoming);
}


fn struct_with_function() {
    #[derive(Debug)]
    struct Dog {
        name: String,
        weight: f32,
        height: f32,
    }

    impl Dog {
        fn get_name(&self) -> &str {
            &(self.name[..])
        }

        fn get_weight(&self) -> f32 {
            self.weight
        }

        fn get_height(&self) -> f32 {
            self.height
        }

        fn show() {
            println!("oh oh oh");
        }
    }

    let dog1 = Dog {
        name: String::from("wangcai"),
        weight: 100.0,
        height: 70.0,
    };
    println!("dog1 = {:#?}", dog1);
    println!("dog1.name = {}", dog1.get_name());

    Dog::show();
}