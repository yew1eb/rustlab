use crate::flow::Flow;
use crate::processor::Processor;
use crate::sinks::Sink;
use crate::stats::Stats;
use crate::Message;
use std::mem;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use uuid::Uuid;

// `Channels` holds the channel for receiving data from upstream DataSet
// as well as channels for transmitting data to downstream DataSets.
struct Channels<T: std::clone::Clone> {
    input_rx: Option<Receiver<Message<T>>>,
    input_txs: Vec<Sender<Message<T>>>,
}

/// DataSet represents a collection of elements which belong to the same type (T).
/// DataSets can be transformed to other DataSets by applying transformations.
pub struct DataSet<T: std::clone::Clone> {
    channels: Arc<Mutex<Channels<T>>>,
    threads: Vec<Option<thread::JoinHandle<()>>>,
    flow: Arc<Mutex<Flow>>,
    registered: bool,
    name: String,
    id: String,
}

impl<T: std::clone::Clone + std::marker::Send + 'static> DataSet<T> {
    /// Creates and returns a new DataSet.
    pub fn new(
        input_rx: Receiver<Message<T>>,
        flow: Arc<Mutex<Flow>>,
        from_id: String,
    ) -> DataSet<T> {
        let channels = Arc::new(Mutex::new(Channels {
            input_rx: Some(input_rx),
            input_txs: Vec::new(),
        }));

        let id = Uuid::new_v4().to_string();

        flow.lock().unwrap().add_edge((from_id, id.clone()));

        DataSet {
            channels,
            threads: Vec::new(),
            flow,
            registered: false,
            name: String::new(),
            id,
        }
    }

    /// Sets the name for the transformation applied to this DataSet.
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    /// Applies a `map` transformation to this DataSet.
    pub fn map<U: 'static, F: 'static>(&mut self, f: F) -> DataSet<U>
    where
        F: std::marker::Sync + std::marker::Send + Fn(T) -> U,
        Self: std::marker::Sized,
        U: std::clone::Clone + std::marker::Send,
    {
        let (input_tx, input_rx) = mpsc::channel::<Message<T>>();
        let (output_tx, output_rx) = mpsc::channel::<Message<U>>();

        let thread = thread::spawn(move || loop {
            let input = input_rx.recv().unwrap();
            match input {
                Message::Data(data) => {
                    let output = f(data);
                    if output_tx.send(Message::Data(output)).is_err() {
                        break;
                    }
                }
                Message::Terminate => {
                    output_tx.send(Message::Terminate).unwrap();
                    break;
                }
            }
        });

        self.channels.lock().unwrap().input_txs.push(input_tx);
        self.threads.push(Some(thread));

        if self.name.is_empty() {
            self.name = "Map Processor".to_string();
        }

        if !self.registered {
            self.register();
        }

        DataSet::new(output_rx, Arc::clone(&self.flow), self.id.clone())
    }

    /// Applies a `filter` transformation to this DataSet.
    pub fn filter<F: 'static>(&mut self, f: F) -> DataSet<T>
    where
        F: std::marker::Send + Fn(&T) -> bool,
        Self: std::marker::Sized,
    {
        let (input_tx, input_rx) = mpsc::channel::<Message<T>>();
        let (output_tx, output_rx) = mpsc::channel::<Message<T>>();
        let thread = thread::spawn(move || {
            loop {
                // receive data from input channel.
                let input = input_rx.recv().unwrap();
                match input {
                    Message::Data(data) => {
                        if f(&data) && output_tx.send(Message::Data(data)).is_err() {
                            break;
                        }
                    }
                    Message::Terminate => {
                        output_tx.send(Message::Terminate).unwrap();
                        break;
                    }
                }
            }
        });

        self.channels.lock().unwrap().input_txs.push(input_tx);
        self.threads.push(Some(thread));

        if self.name.is_empty() {
            self.name = "Filter Processor".to_string();
        }

        if !self.registered {
            self.register();
        }

        DataSet::new(output_rx, Arc::clone(&self.flow), self.id.clone())
    }

    /// Adds a sink to this DataSet.
    pub fn add_sink<S: 'static>(&mut self, sink: S)
    where
        S: std::marker::Send + Sink<T = T>,
    {
        let (input_tx, input_rx) = mpsc::channel::<Message<T>>();
        let (sink_input_tx, sink_input_rx) = mpsc::channel::<Message<T>>();
        let (stats_tx, stats_rx) = mpsc::channel::<Stats>();

        // TODO: Re-factor this (copied from `register`).
        let thread = thread::spawn(move || {
            loop {
                let input = input_rx.recv().unwrap();

                match input {
                    Message::Data(data) => {
                        let mut records_out = 0;
                        let bytes_in = mem::size_of_val(&data);
                        let mut bytes_out: usize = 0;

                        if sink_input_tx.send(Message::Data(data.clone())).is_err() {
                            break;
                        }
                        records_out += 1;
                        bytes_out += bytes_in;
                        // Send stats here.
                        if stats_tx
                            .send(Stats::new(1, records_out, bytes_in, bytes_out))
                            .is_err()
                        {
                            break;
                        }
                    }
                    Message::Terminate => {
                        sink_input_tx.send(Message::Terminate).unwrap();
                        // Send records_in = 0 to signal termination for stats.
                        stats_tx.send(Stats::new(0, 0, 0, 0)).unwrap();
                        break;
                    }
                }
            }
        });

        let sink_name = sink.name();
        let sink_thread = thread::spawn(move || {
            sink.start(sink_input_rx).expect("Error starting sink");
        });

        self.channels.lock().unwrap().input_txs.push(input_tx);
        self.threads.push(Some(thread));
        self.threads.push(Some(sink_thread));

        if self.name.is_empty() {
            self.name = "Sink Processor".to_string();
        }

        if !self.registered {
            self.register();
        }

        // Processor for sink.
        let id = Uuid::new_v4().to_string();
        let processor = Processor::new(id.clone(), sink_name, None, stats_rx);
        self.flow.lock().unwrap().add(processor);
        self.flow.lock().unwrap().add_edge((self.id.clone(), id));
    }

    fn register(&mut self) {
        let (signal_tx, signal_rx) = mpsc::channel::<()>();
        let (stats_tx, stats_rx) = mpsc::channel::<Stats>();
        let channels = Arc::clone(&self.channels);
        let thread = thread::spawn(move || {
            signal_rx.recv().unwrap();

            // Do some plumbing.
            let input_rx = channels.lock().unwrap().input_rx.take().unwrap();
            let input_txs = &channels.lock().unwrap().input_txs;
            loop {
                let input = input_rx.recv().unwrap();

                match input {
                    Message::Data(data) => {
                        let mut records_out = 0;
                        let bytes_in = mem::size_of_val(&data);
                        let mut bytes_out: usize = 0;

                        for input_tx in input_txs {
                            if input_tx.send(Message::Data(data.clone())).is_err() {
                                break;
                            }
                            records_out += 1;
                            bytes_out += bytes_in;
                        }
                        // Send stats here.
                        if stats_tx
                            .send(Stats::new(1, records_out, bytes_in, bytes_out))
                            .is_err()
                        {
                            break;
                        }
                    }
                    Message::Terminate => {
                        for input_tx in input_txs {
                            input_tx.send(Message::Terminate).unwrap();
                        }
                        // Send records_in = 0 to signal termination for stats.
                        stats_tx.send(Stats::new(0, 0, 0, 0)).unwrap();
                        break;
                    }
                }
            }
        });
        self.threads.push(Some(thread));

        let processor = Processor::new(
            self.id.clone(),
            self.name.clone(),
            Some(signal_tx),
            stats_rx,
        );
        self.flow.lock().unwrap().add(processor);

        self.registered = true;
    }
}

impl<T: std::clone::Clone> Drop for DataSet<T> {
    fn drop(&mut self) {
        for thread in &mut self.threads {
            if let Some(t) = thread.take() {
                t.join().unwrap();
            }
        }
    }
}
