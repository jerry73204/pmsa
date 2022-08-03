use anyhow::Result;
use clap::Parser;
use itertools::chain;
use par_merge_sorted_array::{par_merge, seq_merge};
use rand::prelude::*;
use rayon::prelude::*;
use std::time::Instant;

#[derive(Parser)]
struct Opts {
    pub num_values: usize,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    let llist = generate_sorted_vec(opts.num_values);
    let rlist = generate_sorted_vec(opts.num_values);

    {
        let mut tgt = Vec::with_capacity(llist.len() + rlist.len());
        let since = Instant::now();
        seq_merge(&llist, &rlist, &mut tgt);
        println!("seq {:?}", since.elapsed());
        check(&llist, &rlist, &tgt);
    }

    {
        let since = Instant::now();
        let tgt = par_merge(&llist, &rlist);
        println!("par {:?}", since.elapsed());
        check(&llist, &rlist, &tgt);
    }

    Ok(())
}

fn generate_sorted_vec(len: usize) -> Vec<usize> {
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

fn check(llist: &[usize], rlist: &[usize], tgt: &[usize]) {
    let mut values: Vec<_> = chain!(llist, rlist).cloned().collect();
    values.par_sort_unstable();
    assert_eq!(values, tgt);
}
