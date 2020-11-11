use crate::sources::{Message, Result, Sender, Source};
use csv::{Reader, StringRecord};
use std::fs::File;
use std::io::{self, BufReader, Read};

#[derive(Clone, Default)]
pub struct CSVSource {
    filename: Option<String>,
}

impl Source for CSVSource {
    type T = StringRecord;
    fn name(&self) -> String {
        "CSV Source".to_owned()
    }

    fn start(self, tx: Sender<Message<Self::T>>) -> Result<()> {
        let reader: Box<dyn Read> = if let Some(f) = self.filename {
            Box::new(BufReader::new(File::open(f)?))
        } else {
            Box::new(BufReader::new(io::stdin()))
        };
        let mut rdr = Reader::from_reader(reader);
        for result in rdr.records() {
            match result {
                Err(_) => break,
                Ok(record) => {
                    if tx.send(Message::Data(record)).is_err() {
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

impl CSVSource {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_filename(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_owned());
        self
    }
}
