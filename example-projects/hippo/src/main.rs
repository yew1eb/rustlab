pub mod hippo;
#[allow(dead_code)]
#[warn(unused_imports)]
mod timer_task;

use hippo::config::Config;

use std::fs;
use std::process;

use env_logger;
use log::info;
use std::env;
use structopt::StructOpt;
use tokio::runtime::Runtime;

use futures::{
    pin_mut, select,
    stream::{self, SelectAll},
    StreamExt, TryStreamExt,
};
use tokio::{
    net::TcpListener,
    signal::unix::{signal, SignalKind},
    time::{delay_for, Duration},
};

#[derive(StructOpt, Debug, Clone)]
pub struct CliArgs {
    //#[structopt(short, long, default_value = "etc/config.toml")]
    #[structopt(short, long, default_value = "etc/dev_config.toml")]
    pub cfgurl: String,
}

fn main() {
    // 读取命令行执行参数
    let params = CliArgs::from_args();
    println!("{:#?}", params);

    let mut optConfig: Option<Config> = None;
    if params.cfgurl.contains("toml") {
        // 从本地读取配置
        let config_string = fs::read_to_string(&params.cfgurl).unwrap();
        optConfig = match toml::from_str::<Config>(&config_string) {
            Ok(cfg) => Some(cfg),
            Err(error) => {
                println!(
                    "ERROR: toml parse the file {}, error: {}",
                    params.cfgurl, error
                );
                process::abort();
                None
            }
        };
    } else {
        //从远端动态配置更新
        //TODO
    }
    let mut config = optConfig.unwrap();
    println!("{:#?}", &config);

    //TODO 配置自定义日志输出 https://rust-cookbook.budshome.com/development_tools/debugging.html
    env::set_var(
        "RUST_LOG",
        "diesel=debug,actix=debug,actix_web=debug,dts-rust=info",
    );
    env_logger::init();

    // Spawn a set of workers. TODO自定线程个数
    // This has to be in an async task to have access to the `tokio::spawn` primitive
    // 跟在async fn main 加 #[tokio::main]作用是一样的
    Runtime::new().unwrap().block_on(async {
        hippo::start(&config);

        //启动定时任务
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
                    process::abort();
                }
            }
        }
    });

    println!("Main thread close.");
}
