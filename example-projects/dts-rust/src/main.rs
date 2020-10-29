use std::fs;

use log::info;
use structopt::StructOpt;
use tokio::runtime::Runtime;

use config::CliArgs;

mod config;

fn main() {
    let cfg = CliArgs::from_args();
    println!("{:#?}", cfg);


    simple_logger::SimpleLogger::new()
        .with_level(cfg.log_level.to_level_filter())
        .init().expect("Could not init logger");

    info!("Logging initialized with level {}", cfg.log_level);

    // Spawn a set of workers.
    // This has to be in an async task to have access to the `tokio::spawn` primitive
    Runtime::new().unwrap().block_on(
        async {
            let futures =
                (0..cfg.parallel_operations)
                    .map(|_| {
                        tokio::spawn(
                            kafka::receive_messages(cfg.clone())
                        )
                    });

            // wait for all workers to finish
            join_all(futures).await;
        }
    );
}
