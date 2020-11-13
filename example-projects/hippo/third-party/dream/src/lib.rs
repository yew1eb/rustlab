pub mod dataset;
pub mod environment;
pub mod flow;
pub mod processor;
pub mod sinks;
pub mod sources;
pub mod stats;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Message encapsulates the data that flows through the system.
pub enum Message<T: Clone> {
    Data(T),
    Terminate,
}

#[cfg(test)]
mod test {}
