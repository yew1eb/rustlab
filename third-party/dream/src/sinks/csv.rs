use crate::sinks::{Message, Receiver, Result, Sink};
use csv::{StringRecord, Writer};
use std::fs::File;
use std::io::{self, Write};

#[derive(Clone, Default)]
pub struct CSVSink {
    filename: Option<String>,
    headers: Option<StringRecord>,
}

impl Sink for CSVSink {
    type T = StringRecord;
    fn name(&self) -> String {
        "CSV Sink".to_owned()
    }

    fn start(self, rx: Receiver<Message<Self::T>>) -> Result<()> {
        let writer: Box<dyn Write> = if let Some(f) = self.filename {
            Box::new(File::create(f)?)
        } else {
            Box::new(io::stdout())
        };
        let mut wtr = Writer::from_writer(writer);

        if let Some(h) = self.headers {
            wtr.write_record(h.iter())?;
        }

        loop {
            let input = rx.recv().unwrap();
            match input {
                Message::Data(data) => {
                    wtr.write_record(data.iter())?;
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

impl CSVSink {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_filename(mut self, filename: &str) -> Self {
        self.filename = Some(filename.to_owned());
        self
    }

    pub fn with_headers(mut self, headers: StringRecord) -> Self {
        self.headers = Some(headers);
        self
    }
}
