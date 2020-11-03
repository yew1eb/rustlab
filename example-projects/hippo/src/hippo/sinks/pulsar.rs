use std::time::Instant; // timer
use std::str;
use std::env;

use rand::distributions::Alphanumeric;
use rand::Rng;
use anyhow::Context;
use dotenv::dotenv;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use sqlx::postgres::PgPool;
use serde_json::{Value};
use structopt::StructOpt;
use serde::{Deserialize, Serialize};


use crate::hippo::config::Config;
use crate::hippo::config::SourceConfig;
use crate::hippo::sources::Source;