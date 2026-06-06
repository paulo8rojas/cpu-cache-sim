use cpu_cache_sim::cache::{Cache, CacheConfig};

// A simple helper to build a small cache for tests.
// 4 sets, 2-way, 16-byte blocks = 128 bytes total.
fn small_cache() -> Cache {
    Cache::new(CacheConfig {
        cache_size: 128,
        associativity: 2,
        block_size: 16,
    })
}

#[test]
fn cold_miss_on_first_access() {
    // TODO: access any address and assert the result is a Miss
    todo!()
}

#[test]
fn hit_on_repeated_access() {
    // TODO: access the same address twice; second access should be a Hit
    todo!()
}

#[test]
fn lru_eviction_order() {
    // TODO: fill a set completely, then access a new address.
    // The tag that was least recently used should be evicted.
    todo!()
}

#[test]
fn addresses_map_to_correct_sets() {
    // TODO: verify that two addresses differing only in their set-index bits
    // land in different sets (no interference).
    todo!()
}

#[test]
fn stats_count_correctly() {
    // TODO: run a known sequence of hits and misses, then assert
    // stats.total(), stats.hit_rate() match expected values.
    todo!()
}
