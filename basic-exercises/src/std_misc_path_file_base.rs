use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[cfg(test)]
mod test {
    use std::env;

    use super::*;

    #[test]
    fn tpath() {
        // 从 `&'static str` 创建一个 `Path`
        let path = Path::new(".");

        // `display` 方法返回一个可显示（showable）的结构体
        let display = path.display();

        // `join` 使用操作系统的特定分隔符来合并路径，并返回新的路径
        let new_path = path.join("a").join("b");

        // 将路径转换成一个字符串 slice
        match new_path.to_str() {
            None => panic!("new path is not a valid UTF-8 sequence"),
            Some(s) => println!("new path is {}", s),
        }
    }

    #[test]
    fn topen_file() {
        let dir = env::current_dir().unwrap();
        println!("current directory: {}", dir.display());

        // 给所需的文件创建一个路径
        let path = Path::new("./hello.txt");
        let display = path.display();

        // 以只读方式打开路径，返回 `io::Result<File>`
        let mut file = match File::open(&path) {
            // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        // 读取文件内容到一个字符串，返回 `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why.description()),
            Ok(_) => print!("{} contains:\n{}", display, s),
        }

        // `file` 离开作用域，并且 `hello.txt` 文件将被关闭。
    }

    #[test]
    fn create_file() {
        static LOREM_IPSUM: &'static str =
            "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";

        let path = Path::new("hello.txt");
        let display = path.display();

        // 以只写模式打开文件，返回 `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
        match file.write_all(LOREM_IPSUM.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}
