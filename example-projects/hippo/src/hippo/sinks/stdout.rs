use anyhow::Context;
use dotenv::dotenv;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::io::{stdout, Write};
use std::str;
use std::time::Instant; // timer

use super::Sink;
use crate::hippo::config::Config;
use crate::hippo::config::SinkConfig;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;
use structopt::StructOpt;
use tokio::{
    net::TcpListener,
    signal::unix::{signal, SignalKind},
    time::{delay_for, Duration},
};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StdoutConfig {}

#[typetag::serde(name = "stdout")]
impl SinkConfig for StdoutConfig {
    fn build(&self) -> Box<dyn Sink> {
        Box::new(StdoutSink::new(self)) as Box<dyn Sink>
    }

    fn sink_type(&self) -> &'static str {
        "stdout"
    }
}

pub struct StdoutSink {}

impl Sink for StdoutSink {
    fn start(&self, arx: Arc<Mutex<Receiver<Vec<u8>>>>) {
        println!("start stdout sink");

        thread::spawn(move || loop {
            let mut bytes = match { arx.lock().unwrap().recv() } {
                Ok(line) => line,
                Err(_) => return,
            };

            let out = String::from_utf8_lossy(&bytes);
            println!("[stdout] sink: {}", out);
            let _ = stdout().flush();
        });
    }
}

impl StdoutSink {
    pub fn new(stdinConfig: &StdoutConfig) -> Self {
        StdoutSink {}
    }
}
