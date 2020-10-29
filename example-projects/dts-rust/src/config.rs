use log::Level;
use structopt::StructOpt;

pub const CONFIG_FILENAME: &str = "config.toml";

#[derive(Debug, Clone)]
pub struct Config {
    pub loglevel: String,
}


#[derive(StructOpt, Debug, Clone)]
pub struct CliArgs {
    #[structopt(short, long, default_value = "info")]
    pub log_level: Level,
}