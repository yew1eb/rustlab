fn mt() {
    println!("Hello, world!");

    let x = 5;

    let y = if x == 5 {
        println!("hello test");
        10
    } else {
        15
    };

    match y {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    let x = 5;
    if let y = 5 {
        println!("{}", y);
    }

    for x in 0..10 {
        println!("{}", x);
    }

    for (i, j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }

    let mut x = 5;
    let mut done = false;

    while !done {
        x += x - 3;

        println!("x={}", x);

        if x % 5 == 0 {
            done = true;
        }
    }

    for x in 0..10 {
        if x % 2 == 0 {
            continue;
        }

        println!("x = {}", x);
    }

    let mut x = 5;
    //rust 在声明变量时，在变量前面加入 mut 关键字，变量就会成为可变绑定的变量。
    //通过可变绑定可以直接通过标识符修改内存中的变量的值。 在绑定后仍然可以重新修改绑定类型。
    loop {
        x += x - 3;

        println!("x = {}", x);

        if x % 5 == 0 {
            break;
        }
    }

    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 {
                continue 'outer;
            }
            if y % 2 == 0 {
                continue 'inner;
            }
            println!("x : {}, y : {}", x, y);
        }
    }

    let mut a: f64 = 1.0;
    let b = 2.0f32;
    //改变 a 的绑定
    a = 2.0;
    println!("{:?}", a);
    //重新绑定为不可变
    let a = a; //不能赋值
               //a = 3.0;
               //类型不匹配
               //assert_eq!(a, b);

    //Rust 的数组是被表示为[T;N]。其中 N 表示数组大小，并且这个大小一定是个 编译时就能获得的整数值，T 表示泛型类型，即任意类型。
    //和 Golang 一样，Rust 的数组中的 N(大小)也是类型的一部分，即[u8; 3] != [u8; 4]。
    //Rust 大小是固定的。
    let a = [8, 9, 10];
    let b: [u8; 3] = [8, 6, 5];
    print!("{}", a[0]);

    //Slice 从直观上讲，是对一个 Array 的切片，通过 Slice，你能获取到一个
    //Array 的部分或者全部的访问权限。和 Array 不同，Slice 是可以动态的，但是 呢，其范围是不能超过 Array 的大小，这点和 Golang 是不一样的。Golang slice 可以超出 Array 的大小是存在一些问题的。
    let arr = [1, 2, 3, 4, 5, 6];
    let slice_complete = &arr[..]; // 获取全部元素
    let slice_middle = &arr[1..4]; // 获取中间元素，最后取得的 Slice 为 [2, 3, 4] 。切片遵循左 闭右开原则。
    let slice_right = &arr[1..]; // 最后获得的元素为[2, 3, 4, 5, 6]，长度为 5。
    let slice_left = &arr[..3]; // 最后获得的元素为[1, 2, 3]，长度为 3。

    //动态Vec
    let mut v1: Vec<i32> = vec![1, 2, 3]; // 通过 vec!宏来声明
    let v2 = vec![0; 10]; // 声明一个初始长度为 10 的值全为 0 的动态数组
    println!("{}", v1[0]); // 通过下标来访问数组元素
    for i in &v1 {
        print!("{}", i); // &Vec<i32> 可以通过 Deref 转换成 &[i32]
    }
    println!("");
    for i in &mut v1 {
        *i = *i + 1;
        print!("{}", i); // 可变访问
    }
    println!("");

    //函数类型
    fn foo(x: i32) -> i32 {
        x + 1
    }
    let x: fn(i32) -> i32 = foo;
    assert_eq!(11, x(10));

    //枚举类型
    struct Student {
        name: String,
    }

    enum Message {
        School(String),
        Location { x: i32, y: i32 },
        ChangeColor(i32, i32, i32),
        Name(Student),
        ExitColor,
    }

    let m = Message::ExitColor;
    match m {
        Message::ExitColor => println!("{}", "exited color"),
        Message::ChangeColor(x, y, z) => println!("{}", x),
        Message::Name(s) => println!("{}", s.name),
        Message::School(s) => println!("{}", s),
        Message::Location { x, y } => println!("{}", x),
    }
    //与结构体一样，枚举中的元素默认不能使用关系运算符进行比较 (如
    //==, !=, >=)， 也不支持像+和*这样的双目运算符，需要自己实现，或者使 用 match 进行匹配。
    //枚举默认也是私有的，如果使用 pub 使其变为公有，则它的元素也都是默认公
    //有的。 这一点是与结构体不同的:即使结构体是公有的，它的域仍然是默认私 有的。
    //rust 枚举与其他语言的枚举不同的是在指定枚举元素时定义它元素是由什么组
    //成的。

    //字符串类型
    //Go 的字符串可以把它当成字节数组来用，并且是不可变的。
    //Rust 的字符串底层也是 Vec<u8>动态数组，这点和 Go 的字符串有点类似，不同 的是 Go 的字符串是定长的，无法修改的。

    //常用 rust 字符串类型为&str 和 String，前者是字符串的引用，后者是基于堆 创建的，可增长的字符串。
    //let s ="hello world";那 s 的类型就是&str，右边称为字符串字面量 literal， 程序编译成二进制文件后，这个字符串会被保存在文件内部，所以 s 是特定位置字符串的引用，这就是为什么 s 是&str 类型。
    //str 生命周期是 static，但是引用是有生命周期限制的。 可以在字符串字面量前加上 r 来避免转义

    //没有转义序列
    let d: &'static str = r"abc \n abc"; //等 价于
    let c: &'static str = "abc \\n abc";

    let x: &'static str = "hello";
    let mut y: String = x.to_string();
    println!("{}", y);
    y.push_str(", world");
    println!("{}", y);

    //那么如何将一个 String 重新变成&str 呢?用 &* 符号
    let s = "Hello".to_string();
    use_str(&*s);
    //&*是两个符号&和*的组合，按照 Rust 的运算顺序，先对 String 进行 Deref,也 就是*操作。

    test_str_opss()
}

fn use_str(s: &str) {
    println!("I am: {}", s);
}

//这几行定义了一个 Rust 函数。main 函数是一个特殊的函数：在可执行的 Rust 程序中，它总是最先运行的代码。第一行代码声明了一个叫做 main 的函数，它没有参数也没有返回值。如果有参数的话，它们的名称应该出现在小括号中，()。
//还须注意，函数体被包裹在花括号中，{}。Rust 要求所有函数体都要用花括号包裹起来。一般来说，将左花括号与函数声明置于同一行并以空格分隔，是良好的代码风格。

//println! 调用了一个 Rust 宏（macro）。如果是调用函数，则应输入 println（没有!）。我们将在第十九章详细讨论宏。现在你只需记住，当看到符号 ! 的时候，就意味着调用的是宏而不是普通函数。

//该行以分号结尾（;），这代表一个表达式的结束和下一个表达式的开始。大部分 Rust 代码行以分号结尾。

// 单行注释，注释内容直到行尾。
/* 块注释， 注释内容一直到结束分隔符。 */

//Rust 是一种 预编译静态类型（ahead-of-time compiled）语言，这意味着你可以编译程序，并将可执行文件送给其他人，他们甚至不需要安装 Rust 就可以运行。

fn test_str_opss() {
    //需要注意的主要就是:String 类型底层实现是 vec<u8>,unicode 类型，并且拿着引用 可以改变 String 内容。有点类似中在 go 做一个特殊的 String 类型，并且内部包着一个
    //byte数组。
    println!("Hello, world!\n");
    {
        println!("example 1");
        let mut str1 = String::from("你好 hello");
        str1.push_str(" str1");
        let mut str2 = &mut str1;
        str2.push_str(" test ");
        str2.push('a');
        println!("{:?}", str1);
        let mut str3 = &mut str1;
        str3.push_str(" test3 ");
        str3.push('3');
        println!("{:?}", str1);
        // compile error
        // error[E0502]: cannot borrow `str1` as immutable because it is also borrowed as mutable
        // -->src/main.rs:14:25 // |
        //11 | // | //... //14 | // | //...
        //   let mut str3 = &mut str1;
        //  --------- mutable borrow occurs here
        //  println!("{:?}",str1);
        // ^^^^ immutable borrow occurs here
        //17 | println!("{:?}",str3);
        // |
        // println!("{:?}",str3);
        //       ---- mutable borrow later used here
    }
    {
        println!("\nexample 2");
        let mut v1 = vec![1, 2, 3, 4];
        let mut v2 = &mut v1;
        v2.push(100);
        println!("{:?}", v2);
        let mut v3 = &mut v1;
        v3.push(20);
        //compile error
        println!("{:?}", v1);
        //error[E0502]: cannot borrow `v1` as immutable because it is also borrowed as
        //   mutable
        // -->src/main.rs:41:25
        // | //37 | // | //... //41 | // |
        //  let mut v3 = &mut v1;
        //  ------- mutable borrow occurs here
        //  println!("{:?}",v1);
        // ^^ immutable borrow occurs here
        //42 | println!("{:?}",v3);
        // |
        //  println!("{:?}", v3);
    }
    {
        println!("\nexample 3");
        let mut str1 = String::from("hello");
        //error[E0277]: the type `std::string::String` cannot be indexed by `{integer}`
        // -->src/main.rs:59:23
        // |
        //59 | let answer = &str1[0];
        // | ^^^^^^^ `std::string::String` cannot be indexed
        //   by `{integer}` // |
        // = help: the trait `std::ops::Index<{integer}>` is not implemented for `std::string::String`
        // compile error
        // let answer = &str1[0];
    }
    {
        println!("\nexample 4");
        //how to loop the string
        for c in "abc".chars() {
            println!("{}", c);
        }
    }
    {
        println!("\nexample 5");
        let mut a = String::from("testa 你好");
        let mut b = &a[0..4];
        println!("{}", b);
        //This will panic
        //thread 'main' panicked at 'byte index 6 is not a char boundary; it is inside '你' (bytes 5..8) of `testa 你好`', src/libcore/str/mod.rs:2027:5
        //let mut c = &a[0..6];
        //println!("{}",c); }
        {
            println!("\nexample 6");
            let mut a = String::from("testa 你好");
            println!("the length of a is {}", a.len());
        }
        {
            println!("\nexample 7");
            let mut a = String::from("test");
            let mut b = &mut a;
            b.push_str(" str");
            println!("{}", b);
            println!("{}", a);
            //compile error
            //error[E0502]: cannot borrow `a` as immutable because it is also borrowed as mutable
            // --> src/main.rs:102:23
            // |
            //99 | letmutb=&muta;
            // | ------ mutable borrow occurs here
            //...
            //102 |
            // |
            //...
            //106 |
            // |
            //
            //error[E0502]: cannot borrow `a` as immutable because it is also borrowed as
            //         println!("{}",a);
            //         ^ immutable borrow occurs here
            //         println!("{}",b);
            //        - mutable borrow later used here
            //         mutable
            // --> src/main.rs:105:23
            // |
            //99 | letmutb=&muta;
            // | ------ mutable borrow occurs here
            //...
            //105 | println!("{}",a);
            // | ^ immutable borrow occurs here
            //106 | println!("{}",b);
            // | - mutable borrow later used here
            // println!("{}",a);
            // println!("{}",b); }
            {
                println!("\nexample 8");
                let s1 = String::from("Hello, ");
                let s2 = String::from("world!");
                let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
                                   // write bug
                                   // error[E0369]: binary operation `+` cannot be applied to type `&std::string::String`
                                   //let s3 = &s1 + &s2; // 注意 s1 被移动了，不能继续使用
                println!("{}", s3);
                // error[E0382]: borrow of moved value: `s1`
                // --> src/main.rs:141:23
                // |
                //135 | let s1 = String::from("Hello, ");
                // | -- move occurs because `s1` has type `std::string::String`,
                //         which does not implement the `Copy` trait
                //136 | //137 | // | //...
                //        let s2 = String::from("world!");
                //        let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
                //        -- value moved here
                //141 | println!("{}",s1);
                // |            //       ^^ value borrowed here after move
                // compile error
                println!("{}", s2);
            }
        }
    }
}
