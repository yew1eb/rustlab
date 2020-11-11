use crate::{Message, Result};
use std::sync::mpsc::Sender;

pub mod avro;
pub mod csv;
pub mod stdin;

/// An interface for defining a data source.
pub trait Source: Clone {
    /// The type of the elements returned by the data source.
    type T;

    /// Gets the name of the data source.
    fn name(&self) -> String;

    /// Starts the data source.
    fn start(self, tx: Sender<Message<Self::T>>) -> Result<()>
    where
        <Self as Source>::T: std::clone::Clone;
}
