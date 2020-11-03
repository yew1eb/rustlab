macro_rules! say_hello {
    // `()` 表示此宏不接受任何参数。
    () => {
        // 此宏将会展开成这个代码块里面的内容。
        println!("Hello!");
    };
}

macro_rules! create_function {
    // 此宏接受一个 `ident` 指示符参数，并创建一个名为 `$func_name`
    // 的函数。
    // `ident` 指示符用于变量名或函数名
    ($func_name:ident) => {
        fn $func_name() {
            // `stringify!` 宏把 `ident` 转换成字符串。
            println!("You called {:?}()", stringify!($func_name))
        }
    };
}

// 借助上述宏来创建名为 `foo` 和 `bar` 的函数。

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // 此宏接受一个 `expr` 类型的表达式，将它转换成一个字符串，
    // 并伴随着表达式的结果。
    // `expr` 指示符用于表达式。
    ($expression:expr) => {
        // `stringify!` 把表达式转换成一个字符串，正如 stringify
        // （意思为“字符串化”） 所表达的意思那样。
        println!("{:?} = {:?}", stringify!($expression), $expression)
    };
}

// `test!` 将以不同的方式来比较 `$left` 和 `$right`，
// 根据所调用的情况确定。
macro_rules! test {
    // 参数不需要使用逗号隔开。
    // 可以使用任意模板（原文：Any template can be used!）！
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        )
    };
    // ^ 每个分支都必须以分号结束。
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        )
    };
}

// `min!` 将求出任意数量的参数的最小值。
macro_rules! find_min {
    // 基本情形：
    ($x:expr) => ($x);
    // `$x` 后面跟着至少一个 `$y,`
    ($x:expr, $($y:expr),+) => (
        // 对尾部的 `$y` 调用 `find_min!`
        std::cmp::min($x, find_min!($($y),+))
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tmain() {
        say_hello!()
    }

    #[test]
    fn designators() {
        foo();
        bar();

        print_result!(1u32 + 1);

        // 回想一下，代码块也是表达式！
        print_result!({
            let x = 1u32;

            x * x + 2 * x - 1
        });
    }

    #[test]
    fn overload() {
        test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
        test!(true; or false);
    }

    #[test]
    fn repeat() {
        //宏在参数列表中可以使用 + 来表示一个参数可能出现一次或多次，使用 * 来表示该参数可能出现零次或多次。
        println!("{}", find_min!(1u32));
        println!("{}", find_min!(1u32 + 2, 2u32));
        println!("{}", find_min!(5u32, 2u32 * 3, 4u32));
    }
}

/*
这里列出全部指示符：
block
expr 用于表达式
ident 用于变量名或函数名
item
pat (模式 pattern)
path
stmt (语句 statement)
tt (令牌树 token tree)
ty (类型 type)
 */
