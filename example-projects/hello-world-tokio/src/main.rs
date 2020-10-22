use tokio::net::TcpStream;
use tokio::prelude::*;

#[tokio::main] // 该宏创建了一个tokio运行时环境
async fn main() {
    // === tcp 客户端 ===
    // 创建一个Tcp连接
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    println!("created stream");

    // 向Tcp连接中写入数据
    let result = stream.write(b"hello world\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());

    // 关闭Tcp连接
    if let Ok(()) = stream.shutdown(std::net::Shutdown::Write) {
        println!("close stream success");
    }
}