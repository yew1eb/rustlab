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

use super::Sink;
use crate::hippo::config::Config;
use crate::hippo::config::SinkConfig;
