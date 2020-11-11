use crate::sinks::{Message, Receiver, Result, Sink};
use std::io::{self, BufWriter, Write};

#[derive(Clone, Default)]
pub struct PrintSink {
    stderr: bool,
}

impl Sink for PrintSink {
    type T = String;
    fn name(&self) -> String {
        "Prink Sink".to_owned()
    }

    fn start(self, rx: Receiver<Message<Self::T>>) -> Result<()> {
        let mut writer: Box<dyn Write> = if self.stderr {
            Box::new(BufWriter::new(io::stderr()))
        } else {
            Box::new(BufWriter::new(io::stdout()))
        };

        loop {
            let input = rx.recv().unwrap();
            match input {
                Message::Data(data) => {
                    writer.write_all(data.as_bytes())?;
                    writer.write("\n".as_bytes())?;
                }
                Message::Terminate => {
                    break;
                }
            }
        }

        Ok(())
    }
}

impl PrintSink {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn to_stderr(mut self) -> Self {
        self.stderr = true;
        self
    }
}
