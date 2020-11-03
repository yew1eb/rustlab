use serde::{Deserialize, Serialize};
use std::sync::mpsc::SyncSender;

use crate::hippo::sources::Source;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log: LogConfig, 
    pub source: Box<dyn SourceConfig>,
    pub sink: Box<dyn SinkConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogConfig {
    pub level: String,
}
#[typetag::serde(tag = "type")]
pub trait SourceConfig: core::fmt::Debug  {

    fn build(&self, tx: SyncSender<Vec<u8>>) -> Box<dyn Source>;

    fn source_type(&self) -> &'static str;
}


#[typetag::serde(tag = "type")]
pub trait SinkConfig: core::fmt::Debug  {

    fn sink_type(&self) -> &'static str;
}


