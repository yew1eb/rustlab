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

use std::sync::mpsc::SyncSender;

pub async fn receive_messages(config: &Config) {
    println!("kafka receive_messages");

    /*
    let kafka_ip = &env::var("KAFKA_IP").context("`KAFKA_IP` must be set to run this consumer")?;
    let kafka_topic = &env::var("KAFKA_TOPIC").context("`KAFKA_TOPIC` must be set to run this consumer")?;



    // 消费者群组如果没有环境变量则随机生成
    let consumer_group: String;
    if let Ok(val) = env::var("CONSUMER_GROUP") {
        consumer_group = val.to_owned();
    } else {
        let rand_number = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .collect::<String>();
        consumer_group = "consumer-group-".to_owned() + &rand_number;
    }
    println!("consumer-group: {}", &consumer_group);

    // 初始化Kafka消费者
    let mut consumer =
        Consumer::from_hosts(vec!(kafka_ip.to_owned() + ":9092"))
        .with_topic(kafka_topic.to_owned())
        //.with_topic_partitions("test".to_owned(), &[0, 1])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group(consumer_group.to_owned())
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();


        loop {
            for ms in consumer.poll().unwrap().iter() {
                for m in ms.messages() {
                    let start = Instant::now();
                    let json_data = str::from_utf8(&m.value).expect("transfer failed");

                    let sql_string = generate_sql(json_data.to_string()).await?;
                    println!("{}", sql_string);

                    let _inc = save_data(&pool, sql_string).await?;
                    println!("time cost: {:?} ms", start.elapsed().as_millis());// ms
                }
                match consumer.consume_messageset(ms) {
                    Err(e) => println!("{:?}", e),
                    _ => ()
                }
            }
            consumer.commit_consumed().unwrap();
        }
            */
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct KafkaSourceConfig {
    bootstrap_servers: String,
    topics: Vec<String>,
    group_id: String,

    #[serde(default = "default_auto_offset_reset")]
    auto_offset_reset: String,
    #[serde(default = "default_session_timeout_ms")]
    session_timeout_ms: u64,
    #[serde(default = "default_socket_timeout_ms")]
    socket_timeout_ms: u64,
    #[serde(default = "default_fetch_wait_max_ms")]
    fetch_wait_max_ms: u64,
    #[serde(default = "default_commit_interval_ms")]
    commit_interval_ms: u64,
}

fn default_session_timeout_ms() -> u64 {
    10000 // default in librdkafka
}

fn default_socket_timeout_ms() -> u64 {
    60000 // default in librdkafka
}

fn default_fetch_wait_max_ms() -> u64 {
    100 // default in librdkafka
}

fn default_commit_interval_ms() -> u64 {
    5000 // default in librdkafka
}

fn default_auto_offset_reset() -> String {
    "largest".into() // default in librdkafka
}

#[typetag::serde(name = "kafka")]
impl SourceConfig for KafkaSourceConfig {
    fn build(&self, tx: SyncSender<Vec<u8>>) -> Box<dyn Source> {
        Box::new(KafkaSource::new(self, tx)) as Box<dyn Source>
    }

    fn source_type(&self) -> &'static str {
        "kafka"
    }
}

pub struct KafkaSource {}

impl Source for KafkaSource {
    fn start(&self) {
        //TODO
    }
}
impl KafkaSource {
    pub fn new(kafkaConfig: &KafkaSourceConfig, tx: SyncSender<Vec<u8>>) -> Self {
        KafkaSource {}
    }
}
