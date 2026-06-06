/// Breaks a raw memory address into its cache-relevant components.
///
/// For a set-associative cache, every address splits into three fields:
///   [ tag | set index | block offset ]
///
/// The widths of each field depend on the cache configuration:
///   - block_offset bits = log2(block_size)
///   - set_index bits    = log2(num_sets)
///   - tag bits          = remaining high bits
pub struct AddressDecoder {
    // TODO: store bit widths / masks for offset, index, and tag fields
}

impl AddressDecoder {
    /// Create a decoder for a cache with the given geometry.
    /// `num_sets` and `block_size` must both be powers of two.
    pub fn new(num_sets: usize, block_size: usize) -> Self {
        // TODO: compute and store the bit masks and shift amounts
        todo!()
    }

    /// Extract the block offset from an address.
    pub fn offset(&self, addr: u64) -> u64 {
        todo!()
    }

    /// Extract the set index from an address.
    pub fn index(&self, addr: u64) -> u64 {
        todo!()
    }

    /// Extract the tag from an address.
    pub fn tag(&self, addr: u64) -> u64 {
        todo!()
    }
}
