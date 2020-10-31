#[allow(dead_code)]
#[warn(unused_imports)]
pub mod config;
pub mod kafka;
pub mod timer_task;

use std::fs;
use std::process::exit;

use config::CliArgs;
use env_logger;
use log::info;
use std::env;
use structopt::StructOpt;
use tokio::runtime::Runtime;

use tokio::{
    net::TcpListener,
    signal::unix::{signal, SignalKind},
    time::{delay_for, Duration},
};
use futures::{
    pin_mut,
    select,
    stream::{
        self,
        SelectAll,
    },
    StreamExt,
    TryStreamExt,
};
fn main() {
    // 读取命令行执行参数
    let args = CliArgs::from_args();
    println!("{:#?}", args);

    if args.cfgurl.contains("toml") {
        // 从本地读取配置
        let config_string = fs::read_to_string(args.cfgurl).unwrap();
        //let config = toml::from_str::<config::Config>(&config_string).unwrap();
        let config = match toml::from_str::<config::Config>(&config_string) {
            Ok(config) => config,
            Err(err) => {
                println!("toml file parse error: \n{:#?}", err);
                exit(1);
            }
        };
    } else {
        //从远端动态配置更新
        //TODO
    }

    //TODO 配置自定义日志输出 https://rust-cookbook.budshome.com/development_tools/debugging.html
    env::set_var(
        "RUST_LOG",
        "diesel=debug,actix=debug,actix_web=debug,dts-rust=info",
    );
    env_logger::init();

    // Spawn a set of workers.
    // This has to be in an async task to have access to the `tokio::spawn` primitive
    // 跟在async fn main 加 #[tokio::main]作用是一样的
    Runtime::new().unwrap().block_on(async {
        kafka::receive_messages();

        tokio::spawn(timer_task::execute());

        //优雅退出
        let mut signals = SelectAll::new();
        signals.push(
            signal(SignalKind::interrupt()).expect("failed to register the interrupt signal"),
        );
        signals.push(signal(SignalKind::quit()).expect("failed to register the quit signal"));
        signals.push(
            signal(SignalKind::terminate()).expect("failed to register the terminate signal"),
        );
        // ignore SIGPIPE
        let _sigpipe = signal(SignalKind::pipe()).expect("failed to register the pipe signal");
        loop {
            select! {
                _ = signals.select_next_some() => {
                    println!("shutting down...");
                    exit(1);
                }
            }
        }
    });

    println!("Main thread close.");
}
