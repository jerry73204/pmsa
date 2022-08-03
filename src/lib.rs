//! **P**arallel **M**ergeing of two **S**orted **A**rrays using rayon.
//!
//! This crate provides the highly parallelized algorithm to merge two
//! sorted slices in the following functions.
//!
//! - [par_merge]
//! - [par_merge_by]
//! - [par_merge_by_key]
//!
//! # Benchmark Results
//!
//! The benchmark runs the sequential and parallel versions on Intel
//! i7-10750H CPU (12 hyperthreads) using release profile. It is
//! tested on two sorted `u64` arrays. The parallel version is shown
//! to be ~6 times fater than the sequential counterpart.
//!
//! | per array len | sequential   | parallel     |
//! |---------------|--------------|--------------|
//! | 10^6          | 14.56987ms   | 2.219912ms   |
//! | 10^7          | 139.675856ms | 23.363867ms  |
//! | 10^8          | 1.427501286s | 325.121204ms |

mod seq;
pub use seq::*;

mod par;
pub use par::*;

mod utils;
