use std::env;
use std::str;
use std::time::Instant; // timer

use anyhow::Context;
use dotenv::dotenv;
use rand::distributions::Alphanumeric;
use rand::Rng;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::postgres::PgPool;
use structopt::StructOpt;

use crate::hippo::config::Config;
use crate::hippo::config::SourceConfig;
use crate::hippo::sources::Source;
use std::io::{stderr, stdin, BufRead, BufReader, ErrorKind, Read, Write};
use std::sync::mpsc::SyncSender;

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields, default)]
pub struct StdinConfig {}

#[typetag::serde(name = "stdin")]
impl SourceConfig for StdinConfig {
    fn build(&self, tx: SyncSender<Vec<u8>>) -> Box<dyn Source> {
        Box::new(StdinSource::new(self, tx)) as Box<dyn Source>
    }

    fn source_type(&self) -> &'static str {
        "stdin"
    }
}

pub struct StdinSource {
    tx: SyncSender<Vec<u8>>,
}

impl Source for StdinSource {
    fn start(&self) {
        println!("start stdin source");

        let reader = BufReader::new(stdin());

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(e) => match e.kind() {
                    ErrorKind::Interrupted => continue,
                    ErrorKind::WouldBlock => {
                        let _ = writeln!(
                            stderr(),
                            "Client hasn't sent any data for a while - Closing \
                             idle connection"
                        );
                        return;
                    }
                    _ => return,
                },
            };

            self.tx.send(line.into_bytes()).unwrap();
        }
    }
}

impl StdinSource {
    pub fn new(stdinConfig: &StdinConfig, tx: SyncSender<Vec<u8>>) -> Self {
        StdinSource { tx: tx }
    }
}
