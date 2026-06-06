/// Accumulates hit/miss counts for a simulation run.
#[derive(Debug, Default)]
pub struct Stats {
    // TODO: fields for hits, misses, and any other metrics you want
    hit_count: u32,
    miss_count: u32,
}

impl Stats {
    pub fn new() -> Self {
        todo!()
    }

    pub fn record_hit(&mut self) {
        self.hit_count += 1;
        println!("cache hit!")
    }

    pub fn record_miss(&mut self) {
        self.miss_count += 1;
        println!("cache miss!")
    }

    /// Total number of accesses (hits + misses).
    pub fn total(&self) -> u64 {
        todo!()
    }

    /// Hit rate as a value in [0.0, 1.0]. Returns 0.0 if no accesses yet.
    pub fn hit_rate(&self) -> f64 {
        todo!()
    }

    /// Pretty-print a summary to stdout.
    pub fn print_summary(&self) {
        // TODO: print hits, misses, total, hit rate
        todo!()
    }
}
