use crate::sources::{Message, Result, Sender, Source};
use avro_rs::Reader;
use std::fs::File;
use std::io::{self, BufReader, Read};

#[derive(Clone, Default)]
pub struct AvroSource {
    filename: Option<String>,
}

impl Source for AvroSource {
    type T = avro_rs::types::Value;
    fn name(&self) -> String {
        "Avro Source".to_owned()
    }

    fn start(self, tx: Sender<Message<Self::T>>) -> Result<()> {
        let reader: Box<dyn Read> = if let Some(f) = self.filename {
            Box::new(BufReader::new(File::open(f)?))
        } else {
            Box::new(BufReader::new(io::stdin()))
        };
        let rdr = Reader::new(reader)?;
        for value in rdr {
            match value {
                Err(_) => break,
                Ok(v) => {
                    if tx.send(Message::Data(v)).is_err() {
                        break;
                    }
                }
            }
        }

        // no more data.
        let _ = tx.send(Message::Terminate);
        Ok(())
    }
}

impl AvroSource {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_filename(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_owned());
        self
    }
}
