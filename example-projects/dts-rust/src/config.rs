use structopt::StructOpt;
use serde_derive::Deserialize;


#[derive(StructOpt, Debug, Clone)]
pub struct CliArgs {
    #[structopt(short, long, default_value = "config.toml")]
    pub cfgurl: String,
}


#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub log: LogConfig, 
    pub kafkaConf: KafkaConfig,
    pub ClickhouseConf: ClickhouseConfig,
    

}

#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    pub level: String,
}




