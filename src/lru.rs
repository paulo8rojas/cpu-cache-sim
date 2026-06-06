/// LRU eviction tracker for a single cache set.
///
/// Tracks the access order of `capacity` tags so the least-recently-used
/// tag can be identified and evicted when the set is full.
///
/// Hint: a VecDeque works well here — move a tag to the back on every
/// access and pop from the front when you need to evict.
pub struct LruTracker {
    // TODO: store access order and capacity
}

impl LruTracker {
    /// Create a tracker for a set with `capacity` ways.
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    /// Record an access to `tag`. If the tag is already tracked, move it
    /// to most-recently-used. If it is new and the set is full, evict the
    /// least-recently-used tag and return it.
    ///
    /// Returns `Some(evicted_tag)` when an eviction occurred, `None` otherwise.
    pub fn access(&mut self, tag: u64) -> Option<u64> {
        todo!()
    }

    /// Remove a specific tag from the tracker (e.g. on an explicit eviction).
    pub fn remove(&mut self, tag: u64) {
        todo!()
    }

    /// Return the least-recently-used tag without evicting it.
    pub fn peek_lru(&self) -> Option<u64> {
        todo!()
    }
}
