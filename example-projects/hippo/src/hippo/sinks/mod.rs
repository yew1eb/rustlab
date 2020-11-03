pub mod clickhouse;
pub mod pulsar;
pub mod stdout;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
pub trait Sink {
    fn start(&self, arx: Arc<Mutex<Receiver<Vec<u8>>>>) {}
}
