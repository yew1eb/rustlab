mod config;
mod server;

#[macro_use]
extern crate serde_derive;

extern crate env_logger;
extern crate log;

extern crate failure;
extern crate toml;
use failure::Error;

use std::env;
use std::fs;
use std::process::exit;

fn main() -> Result<(), Error> {
    //解析配置文件
    let config_string = fs::read_to_string(config::CONFIG_FILENAME)?;
    let config: config::Config = toml::from_str(&config_string)?;
    println!("{:#?}", config);

    //设置日志
    env::set_var(
        "RUST_LOG",
        format!(
            "actix_web={},webapp={}",
            config.log.actix_web, config.log.webapp
        ),
    );
    env_logger::init();

    let server = server::Server::new(&config)?;

    server.start();

    Ok(())
}
