use crate::dataset::DataSet;
use crate::flow::Flow;
use crate::sources::Source;
use crate::Message;
use log::debug;
use std::sync::mpsc::{self};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

/// Environment sets up and executes the data processing pipeline.
pub struct Environment {
    name: String,
    source_runners: Vec<Option<SourceRunner>>,
    source_threads: Vec<Option<thread::JoinHandle<()>>>,
    flow: Arc<Mutex<Flow>>,
}

struct SourceRunner(Box<dyn FnOnce() + std::marker::Send + 'static>);

impl Environment {
    /// Creates and returns a new Environment.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            source_runners: Vec::new(),
            source_threads: Vec::new(),
            flow: Arc::new(Mutex::new(Flow::new())),
        }
    }

    /// Adds a data source to the environment.
    pub fn add_source<S: 'static>(&mut self, source: S) -> DataSet<S::T>
    where
        S: std::marker::Send + Source,
        <S as Source>::T: std::marker::Send,
        <S as Source>::T: std::clone::Clone,
    {
        let (source_tx, source_rx) = mpsc::channel::<Message<S::T>>();
        let name = source.name();

        let x = SourceRunner(Box::new(move || {
            source.start(source_tx).expect("Error starting source");
        }));

        self.source_runners.push(Some(x));

        DataSet::new(source_rx, Arc::clone(&self.flow), "ROOT".to_owned())
            .name(format!("{} Processor", name).as_str())
    }

    /// Signals the environment to run the data processing pipeline.
    pub fn run(&mut self) {
        debug!("Starting {}.", self.name);

        // Start the flow.
        self.flow.lock().unwrap().start();

        // Start sources.
        for source_runner in &mut self.source_runners {
            let runner = source_runner.take();
            let thread = thread::spawn(move || {
                runner.unwrap().0();
            });
            self.source_threads.push(Some(thread));
        }
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        debug!("Closing execution environment.");
        for thread in &mut self.source_threads {
            if let Some(thread) = thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
