# PMSA: Parallel Mergeing of Two Sorted Arrays in Rust

\[ [docs.rs](https://docs.rs/pmsa/) | [crates.io](https://crates.io/crates/pmsa) \]

It implements the **P**arallel **M**ergeing of two **S**orted
**A**rrays algorithm in Rust using rayon. The following functions are
provided in this crate.

- `par_merge`
- `par_merge_by`
- `par_merge_by_key`

## Benchmark

To test the parallel algoirhtm against the sequential one,

```rust
cargo run --release --example benchmark -- 1000000
```

The benchmark runs the sequential and parallel versions on Intel
i7-10750H CPU (12 hyperthreads) using release profile. It is tested on
two sorted `u64` arrays. The parallel version is shown to be ~6 times
fater than the sequential counterpart.

| per array len | sequential   | parallel     |
|---------------|--------------|--------------|
| 10^6          | 14.56987ms   | 2.219912ms   |
| 10^7          | 139.675856ms | 23.363867ms  |
| 10^8          | 1.427501286s | 325.121204ms |

## License

MIT license.
