mod kv;
mod kv_handler;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// curl http://localhost:8080/
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

use actix_web::get;

// curl http://localhost:8080/hello
// 使用宏解析
#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

//actix web 使用文档： https://www.rectcircle.cn/posts/rust-actix/

#[actix_rt::main] //#[actix_rt::main] 用于生成异步函数运行时，需要引入 actix-rt = "1.0" 依赖
async fn main() -> std::io::Result<()> {

    use actix_web:: {
        middleware::Logger,
        web::{scope, resource, get, post, delete},
        App,
        HttpServer
    };

    let db: kv::RocksDB = kv::KVStore::init("/tmp/rocksdb/actix-db");

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
        //actix_web 提供了 web::Data API 用来在程序间共享状态
        //线程级别共享，共享的类型不用实现 线程交换安全，只能用于只读，如全局配置。通过 .data(T) 初始化
            .data(db.clone()) 
            .wrap(Logger::default())
            .route("/", get().to(index))
            .service(index3)
            .service(
                scope("/api") //一个 App 可以通过 scope，为路由添加统一的前缀。
                .service(
                    resource("/{key}")
                        .route(get().to(kv_handler::get))
                        .route(post().to(kv_handler::post))
                        .route(delete().to(kv_handler::delete)),
                ),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
    //actix_web::App 是 actix_web 的核心，所有的路由、服务、共享数据都围绕 App 构建。
    //在 actix_web 中，每个线程持有一个 App 实例。在创建 Server 时，需要传递一个 App 的 工厂函数

}