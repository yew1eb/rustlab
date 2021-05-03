/*
read_file_contents 是一个函数，该函数接收 PathBuf 结构作为其单个输入并返回一个 Result<String, io::Error>。
 该函数按顺序执行以下操作：
新建一个空的可变 String。
尝试访问指定路径中的文件。
尝试使用 read_to_string 方法将该文件中的内容读取到你刚才创建的 String 中。
返回修改的 String.
*/
use std::fs::File;
use std::io::Error as IoError;
use std::io::Read;
use std::path::PathBuf;

fn read_file_contents(path: PathBuf) -> Result<String, IoError> {
    let mut string = String::new();

    // TODO #1: Handle this match expression.
    // --------------------------------------
    // Pass the variable to the `file` variable on success, or
    // Return from the function early if it is an error.
    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error),
    };

    // TODO #2: Handle this error.
    // ---------------------------
    // The success path is already filled in for you.
    // Return from the function early if it is an error.
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    };

    // TODO #3: Return the `string` variable as expected by this function signature.
    Ok(string)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_main() {
        assert!(read_file_contents(PathBuf::from("src/main.rs")).is_ok());
        assert!(read_file_contents(PathBuf::from("non-existent-file.txt")).is_err());
    }
}
