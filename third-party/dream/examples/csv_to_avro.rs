use avro_rs::to_value;
use avro_rs::types::Value;
use avro_rs::Schema;
use chrono::Local;
use csv::StringRecord;
use dream::dataset::DataSet;
use dream::environment::Environment;
use dream::sinks::avro::AvroSink;
use dream::sources::csv::CSVSource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UserData {
    registration_dttm: String,
    id: String,
    first_name: String,
    last_name: String,
    email: String,
    gender: String,
    ip_address: String,
    cc: String,
    country: String,
    birthdate: String,
    salary: String,
    title: String,
    comments: String,
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

    // Source CSV file: https://raw.githubusercontent.com/Teradata/kylo/master/samples/sample-data/csv/userdata1.csv

    // Add Source.
    let mut dataset: DataSet<StringRecord> = env
        .add_source(CSVSource::new().with_filename("data/in.csv"))
        .name("CSV Source");

    // Convert CSV `StringRecord` to Avro `Value`.
    let headers = StringRecord::from(vec![
        "registration_dttm",
        "id",
        "first_name",
        "last_name",
        "email",
        "gender",
        "ip_address",
        "cc",
        "country",
        "birthdate",
        "salary",
        "title",
        "comments",
    ]);
    let mut to_avro_value: DataSet<Value> = dataset
        .map(move |row: StringRecord| {
            let ud: UserData = row.deserialize(Some(&headers)).unwrap();
            to_value(ud).unwrap()
        })
        .name("To Avro Value");

    // Add Avro Sink.
    let raw_schema = r#"
        {
            "type": "record",
            "name": "userdata",
            "fields": [
                {"name": "registration_dttm", "type": "string"},
                {"name": "id", "type": "string"},
                {"name": "first_name", "type": "string"},
                {"name": "last_name", "type": "string"},
                {"name": "email", "type": "string"},
                {"name": "gender", "type": "string"},
                {"name": "ip_address", "type": "string"},
                {"name": "cc", "type": "string"},
                {"name": "country", "type": "string"},
                {"name": "birthdate", "type": "string"},
                {"name": "salary", "type": "string"},
                {"name": "title", "type": "string"},
                {"name": "comments", "type": "string"}
            ]
        }
    "#;
    let schema = Schema::parse_str(raw_schema).unwrap();
    to_avro_value.add_sink(AvroSink::new(schema).with_filename("data/userdata.avro"));

    // Run the pipeline.
    env.run();
}
