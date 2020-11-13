use avro_rs::types::Value;
use chrono::Local;
use dream::dataset::DataSet;
use dream::environment::Environment;
use dream::sinks::print::PrintSink;
use dream::sources::avro::AvroSource;

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

    // Extra step to convert data to String (required by print sink)
    let mut to_str: DataSet<String> = dataset
        .map(|row: Value| format!("{:?}", row))
        .name("To String");

    // Print.
    to_str.add_sink(PrintSink::new().to_stderr());

    // Run the pipeline.
    env.run();
}
