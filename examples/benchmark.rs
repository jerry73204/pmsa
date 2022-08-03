use anyhow::Result;
use clap::Parser;
use itertools::chain;
use pmsa::{par_merge, seq_merge};
use rand::prelude::*;
use rayon::prelude::*;
use std::time::Instant;

#[derive(Parser)]
struct Opts {
    pub num_values: usize,
    #[clap(long)]
    pub par_only: bool,
    #[clap(long)]
    pub seq_only: bool,
    #[clap(long)]
    pub no_check: bool,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    assert!(
        !(opts.par_only && opts.seq_only),
        "--par-only and --seq-only cannot be specified in the mean time"
    );

    let llist = generate_sorted_vec(opts.num_values);
    let rlist = generate_sorted_vec(opts.num_values);

    if !opts.par_only {
        let mut tgt = Vec::with_capacity(llist.len() + rlist.len());
        let since = Instant::now();
        seq_merge(&llist, &rlist, &mut tgt);
        println!("seq {:?}", since.elapsed());

        if !opts.no_check {
            check(&llist, &rlist, &tgt);
        }
    }

    if !opts.seq_only {
        let since = Instant::now();
        let tgt = par_merge(&llist, &rlist);
        println!("par {:?}", since.elapsed());

        if !opts.no_check {
            check(&llist, &rlist, &tgt);
        }
    }

    Ok(())
}

fn generate_sorted_vec(len: usize) -> Vec<u64> {
    let incs: Vec<_> = (0..len)
        .into_par_iter()
        .map_init(rand::thread_rng, |rng, _| rng.gen_range(0..=8))
        .collect();

    let values: Vec<_> = incs
        .iter()
        .scan(0, |acc, &curr| {
            let out = *acc;
            *acc += curr;
            Some(out)
        })
        .collect();

    values
}

fn check(llist: &[u64], rlist: &[u64], tgt: &[u64]) {
    let mut values: Vec<_> = chain!(llist, rlist).cloned().collect();
    values.par_sort_unstable();
    assert_eq!(values, tgt);
}
