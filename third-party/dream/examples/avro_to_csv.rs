use avro_rs::from_value;
use avro_rs::types::Value;
use chrono::Local;
use csv::StringRecord;
use dream::dataset::DataSet;
use dream::environment::Environment;
use dream::sinks::csv::CSVSink;
use dream::sources::avro::AvroSource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Weather {
    station: String,
    time: i64,
    temp: i8,
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

    // Add Avro Source.
    let mut dataset: DataSet<Value> = env
        .add_source(AvroSource::new().with_filename("data/weather.avro"))
        .name("Avro Source");

    // Convert Avro `Value` to CSV `StringRecord`.
    let mut to_csv_row: DataSet<StringRecord> = dataset
        .map(|val: Value| {
            let weather = from_value::<Weather>(&val).unwrap();
            let row: StringRecord = StringRecord::from(vec![
                weather.station,
                weather.time.to_string(),
                weather.temp.to_string(),
            ]);
            row
        })
        .name("To CSV Row");

    // Add CSV Sink.
    to_csv_row.add_sink(
        CSVSink::new()
            .with_filename("data/weather.csv")
            .with_headers(StringRecord::from(vec!["station", "time", "temp"])),
    );

    // Run the pipeline.
    env.run();
}
