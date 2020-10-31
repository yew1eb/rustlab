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

pub async fn receive_messages() {
    let kafka_ip = &env::var("KAFKA_IP").context("`KAFKA_IP` must be set to run this consumer")?;
    let kafka_topic = &env::var("KAFKA_TOPIC").context("`KAFKA_TOPIC` must be set to run this consumer")?;
    
    println!("kafka receive_messages");

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
}

async fn generate_sql(json_string: String) -> anyhow::Result<String> {

    let v: Value = serde_json::from_str(&json_string)?;

    let mut colums: Vec<String> = vec![];
    let mut values: Vec<String> = vec![];

    for (key, value) in v["data"].as_object().unwrap() {
        colums.push(key.to_string());
        values.push(value.to_string());
    }

    let sql_keys = colums.join(",");
    let sql_values = values.join(",").replace("\"", "'");

    let sql = format!("INSERT INTO users ({}) VALUES ({});", sql_keys, sql_values);

    Ok(sql)
}
