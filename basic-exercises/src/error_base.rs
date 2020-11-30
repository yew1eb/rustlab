// 不可恢复错误是bug的同义词，rust中通过panic!来实现
// 添加调试信息输出 RUST_BACKTRACE=1 cargo run

//什么时候用painc!, 什么时候用Result
//(1)示例、代码原型、测试用painc!\unwrap\expect ?
//(2)实际项目中应该用Result
// Option和Result

use std::fs::File;
use std::io;
use std::io::Read;

fn fn_error2() {
    let r = read_username_from_file3();
    match r {
        Ok(s) => println!("s = {}", s),
        Err(e) => println!("err = {:?}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tmain() {
        fn_error2();
    }

    #[test]
    #[should_panic] // or  #[should_panic(expected = "AAAaaaaa!!!!")]
    fn t_panic() {
        let gift = "snake";
        if gift == "snake" {
            panic!("AAAaaaaa!!!!");
        }
    }
}
