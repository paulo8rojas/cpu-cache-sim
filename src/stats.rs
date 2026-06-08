/// Accumulates hit/miss counts for a simulation run.
#[derive(Debug, Default)]
pub struct Stats {
    // TODO: fields for hits, misses, and any other metrics you want
    hit_count: u64,
    miss_count: u64,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            hit_count: 0,
            miss_count: 0,
        }
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
        self.hit_count + self.miss_count
    }

    /// Hit rate as a value in [0.0, 1.0]. Returns 0.0 if no accesses yet.
    pub fn hit_rate(&self) -> f64 {
        (self.hit_count as f64) / (self.total() as f64)
    }

    /// Pretty-print a summary to stdout.
    pub fn print_summary(&self) {
        // TODO: print hits, misses, total, hit rate
        println!("Hits: {}",self.hit_count);
        println!("Misses: {}",self.miss_count);
        println!("Total Accesses {}",self.total());
        println!("Hit Rate: {}",self.hit_rate());
    }
}
