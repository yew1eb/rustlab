use crate::processor::Processor;

/// Flow contains the processors and the connections/edges between them.
#[derive(Default)]
pub struct Flow {
    processors: Vec<Processor>,
    edges: Vec<(String, String)>,
}

impl Flow {
    /// Creates and returns a new Flow.
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds a processor to the flow.
    pub fn add(&mut self, processor: Processor) {
        self.processors.push(processor);
    }

    /// Adds an edge to the flow.
    pub fn add_edge(&mut self, edge: (String, String)) {
        self.edges.push(edge);
    }

    /// Stats the flow.
    pub fn start(&mut self) {
        // debug!("{:?}", self.edges);
        // Start in reverse order to ensure that downstream receivers
        // have been set up properly.
        self.processors.reverse();
        for p in &mut self.processors {
            p.start();
        }
    }
}
