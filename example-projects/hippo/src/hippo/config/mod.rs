use serde::{Deserialize, Serialize};
use std::sync::mpsc::SyncSender;

use crate::hippo::sinks::Sink;
use crate::hippo::sources::Source;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
#[derive(Debug, Deserialize)]
pub struct Config {
    pub global: GlobalOptions,
    pub source: Box<dyn SourceConfig>,
    pub sink: Box<dyn SinkConfig>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct GlobalOptions {
    pub log: LogConfig,
    pub db: MysqlConfig,
    pub kafka: KafkaConfig,
    pub clickhouse: ClickhouseConfig,
    pub pulsar: PulsarConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogConfig {
    pub level: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MysqlConfig {
    pub addr: String,
    pub idle: i32,
    pub max: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct KafkaConfig {
    pub brokers: String,
    pub groupId: String,
    pub consumerOffset: String,
    pub timeout: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClickhouseConfig {
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub dsnParams: String,
    pub inputBufferSize: i32,
    pub flushInterval: i32,
    pub flushParallel: i32,
    pub flushLimitMB: i32,
    pub flushLimitSize: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PulsarConfig {
    pub addr: String,
    pub token: String,
    pub timeout: i32,
}

#[typetag::serde(tag = "type")]
pub trait SourceConfig: core::fmt::Debug {
    fn build(&self, tx: SyncSender<Vec<u8>>) -> Box<dyn Source>;

    fn source_type(&self) -> &'static str;
}

#[typetag::serde(tag = "type")]
pub trait SinkConfig: core::fmt::Debug {
    fn build(&self) -> Box<dyn Sink>;

    fn sink_type(&self) -> &'static str;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn missing_key() {}
}
