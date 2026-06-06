use crate::address::AddressDecoder;
use crate::lru::LruTracker;
use crate::stats::Stats;

/// The result of a single cache access.
pub enum AccessResult {
    Hit,
    Miss,
}

/// Configuration for the cache.
pub struct CacheConfig {
    /// Total cache capacity in bytes. Must be a power of two.
    pub cache_size: usize,
    /// Number of ways (lines per set). Must be a power of two.
    pub associativity: usize,
    /// Size of each cache block in bytes. Must be a power of two.
    pub block_size: usize,
}

impl CacheConfig {
    /// Derive the number of sets from the config fields.
    ///
    /// num_sets = cache_size / (associativity * block_size)
    pub fn num_sets(&self) -> usize {
        todo!()
    }
}

/// A set-associative cache with LRU replacement.
///
/// Internally holds one `LruTracker` per set and uses an `AddressDecoder`
/// to split each address into (tag, set_index, block_offset).
pub struct Cache {
    config: CacheConfig,
    decoder: AddressDecoder,
    // TODO: storage for the sets (tags currently held in each way)
    // TODO: one LruTracker per set
    pub stats: Stats,
}

impl Cache {
    /// Build a cache from the given config.
    pub fn new(config: CacheConfig) -> Self {
        // TODO: validate that sizes are powers of two
        // TODO: initialise sets and LRU trackers
        todo!()
    }

    /// Simulate one memory access at `addr`.
    ///
    /// 1. Decode the address into (tag, set_index, offset).
    /// 2. Look up the tag in the correct set.
    /// 3. On a hit: update LRU order, record hit in stats.
    /// 4. On a miss: evict LRU line if the set is full, insert new tag,
    ///    record miss in stats.
    pub fn access(&mut self, addr: u64) -> AccessResult {
        todo!()
    }
}
