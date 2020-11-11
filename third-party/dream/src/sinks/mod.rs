use crate::{Message, Result};
use std::sync::mpsc::Receiver;

pub mod avro;
pub mod csv;
pub mod print;

/// An interface for defining a data sink.
pub trait Sink: Clone {
    /// The type of the elements expected by the sink.
    type T;

    /// Returns the name of the sink.
    fn name(&self) -> String;

    /// Starts the sink.
    fn start(self, rx: Receiver<Message<Self::T>>) -> Result<()>
    where
        <Self as Sink>::T: std::clone::Clone;
}
