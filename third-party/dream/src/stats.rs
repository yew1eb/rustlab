/// Stats contains the statistics for a Processor.
#[derive(Debug)]
pub struct Stats {
    records_in: u64,
    records_out: u64,
    bytes_in: usize,
    bytes_out: usize,
}

impl Stats {
    /// Creates and returns a new Stats.
    pub fn new(records_in: u64, records_out: u64, bytes_in: usize, bytes_out: usize) -> Self {
        Stats {
            records_in,
            records_out,
            bytes_in,
            bytes_out,
        }
    }

    /// Updates this Stats with the provided Stats.
    pub fn update(&mut self, stats: Stats) -> bool {
        if stats.records_in == 0 {
            return false;
        }
        self.records_in += stats.records_in;
        self.records_out += stats.records_out;
        self.bytes_in += stats.bytes_in;
        self.bytes_out += stats.bytes_out;
        true
    }

    /// Returns the number of records in.
    pub fn get_records_in(&self) -> u64 {
        self.records_in
    }

    /// Returns the number of records out.
    pub fn get_records_out(&self) -> u64 {
        self.records_out
    }

    /// Returns the number of bytes in.
    pub fn get_bytes_in(&self) -> usize {
        self.bytes_in
    }

    /// Returns the number of bytes out.
    pub fn get_bytes_out(&self) -> usize {
        self.bytes_out
    }
}
