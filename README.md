# cpu-cache-sim

A set-associative CPU cache simulator with LRU replacement, written in Rust.

## Project Layout

```
src/
├── main.rs       # Entry point and CLI
├── lib.rs        # Module declarations
├── address.rs    # Address decoder (tag / set index / block offset)
├── lru.rs        # LRU eviction tracker (per set)
├── cache.rs      # Set-associative cache + access logic
└── stats.rs      # Hit/miss counters and reporting
tests/
└── cache_tests.rs
```

## Getting Started

```bash
cargo build
cargo test
cargo run -- --help
```

## Roadmap

- [ ] CLI: accept cache size, associativity, block size, trace file path
- [ ] Implement `AddressDecoder` (address.rs)
- [ ] Implement `LruTracker` (lru.rs)
- [ ] Implement `Cache::access` (cache.rs)
- [ ] Implement `Stats` (stats.rs)
- [ ] Fill in integration tests (tests/cache_tests.rs)
- [ ] Parse a memory access trace file (e.g. Valgrind lackey format)
- [ ] Write-policy support (write-back vs write-through)
- [ ] Multi-level cache (L1 → L2 → L3)
- [ ] Visualise hit/miss patterns

## References

- [Computer Organization and Design (Patterson & Hennessy) — Chapter 5](https://www.elsevier.com/books/computer-organization-and-design-risc-v-edition/patterson/978-0-12-820331-6)
- [What Every Programmer Should Know About Memory — Ulrich Drepper](https://people.freebsd.org/~lstewart/articles/cpumemory.pdf)

## License

MIT
