use crate::sinks::{Message, Receiver, Result, Sink};
use avro_rs::types::Value;
use avro_rs::{Schema, Writer};
use std::fs::File;
use std::io::{self, Write};

#[derive(Clone)]
pub struct AvroSink {
    schema: Schema,
    filename: Option<String>,
}

impl Sink for AvroSink {
    type T = Value;
    fn name(&self) -> String {
        "Avro Sink".to_owned()
    }

    fn start(self, rx: Receiver<Message<Self::T>>) -> Result<()> {
        let writer: Box<dyn Write> = if let Some(f) = self.filename {
            Box::new(File::create(f)?)
        } else {
            Box::new(io::stdout())
        };
        let mut wtr = Writer::new(&self.schema, writer);

        loop {
            let input = rx.recv().unwrap();
            match input {
                Message::Data(data) => {
                    wtr.append(data)?;
                }
                Message::Terminate => {
                    break;
                }
            }
        }

        wtr.flush()?;

        Ok(())
    }
}

impl AvroSink {
    pub fn new(schema: Schema) -> Self {
        Self {
            schema,
            filename: None,
        }
    }

    pub fn with_filename(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_owned());
        self
    }
}
