use avro_rs::types::Value;
use avro_rs::Schema;
use avro_rs::{from_value, to_value};
use chrono::Local;
use dream::dataset::DataSet;
use dream::environment::Environment;
use dream::sinks::avro::AvroSink;
use dream::sources::avro::AvroSource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Weather {
    station: String,
    time: i64,
    temp: i8,
    #[serde(default)]
    is_cold: bool,
}

fn main() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stderr())
        .apply()
        .unwrap();
    let mut env = Environment::new("My Pipeline");

    // Source AVRO file: https://github.com/apache/avro/blob/master/share/test/data/weather.avro

    // Add Source.
    let mut dataset: DataSet<Value> = env
        .add_source(AvroSource::new().with_filename("data/weather.avro"))
        .name("Avro Source");

    // Specify if the temperature is "cold" (temp <= 0).
    let mut with_coldness: DataSet<Value> = dataset
        .map(|val: Value| {
            let mut weather = from_value::<Weather>(&val).unwrap();
            if weather.temp <= 0 {
                weather.is_cold = true;
            }
            to_value(weather).unwrap()
        })
        .name("With Coldness");

    // Add Avro Sink.
    let raw_schema = r#"
        {
            "type": "record",
            "name": "weather",
            "fields": [
                {"name": "station", "type": "string"},
                {"name": "time", "type": "long"},
                {"name": "temp", "type": "int"},
                {"name": "is_cold", "type": "boolean"}
            ]
        }
    "#;
    let schema = Schema::parse_str(raw_schema).unwrap();
    with_coldness.add_sink(AvroSink::new(schema).with_filename("data/result.avro"));

    // Run the pipeline.
    env.run();
}
