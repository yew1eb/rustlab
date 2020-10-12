/**
 * Hyper 中有 Service 的概念。它是一个实现了 Service trait 的类型，有一个 call 函数，
 * 接收一个表示解析过的 HTTP 请求的 hyper::Request 对象作为参数。
 * 对于一个异步服务来说，这个函数必须返回一个 Future。
 * 
 */

extern crate hyper;
extern crate futures;


#[macro_use]
extern crate log;
extern crate env_logger;

use hyper::server::{Request, Response, Service};

use futures::future::Future;


struct Microservice;

impl Service for Microservice {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        info!("Microservice received a request: {:?}", request);
        Box::new(futures::future::ok(Response::new()))
      }
}


fn main() {
    env_logger::init();
    let address = "127.0.0.1:8080".parse().unwrap();
    let server = hyper::server::Http::new()
      .bind(&address, || Ok(Microservice {}))
      .unwrap();
    info!("Running microservice at {}", address);
    server.run().unwrap();
}
/*
上面的代码，hyper 会在 localhost:8080 开始监听 HTTP 请求，
解析并将其转发到我们的 Microservice 类。请注意，每次有新请求到来，
都会创建一个新的实例。我们现在可以启动服务器，用 curl 发来一些请求！
我们在终端中启动服务器：
RUST_LOG="microservice=debug" cargo run

注意到在上面的命令中，我将 RUST_LOG="microservice=debug" 添加到了 cargo run 中。
由于 env_logger 会搜索这个特定的环境变量，我们通过这种方式控制它的行为。
这个环境变量（"microservice=debug"）的第一部分指定了我们希望启动的日志的根模块，
第二部分（= 后面的部分）指定了可见的最小日志级别。默认情况下，只有 error! 会被记录。


TODO

参考链接： https://doc.xuwenliang.com/docs/rust/2588


*/


