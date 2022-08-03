use rayon::prelude::*;
use std::{cmp::Ordering, mem};

use crate::{
    seq_merge_by,
    utils::{
        even_division_lens, partition_by_indices, partition_by_lens, partition_by_lens_mut,
        MutSliceWriter,
    },
};

pub fn par_merge<T>(lslice: &[T], rslice: &[T]) -> Vec<T>
where
    T: Clone + Ord + Sync + Send,
{
    par_merge_by(lslice, rslice, |lhs, rhs| lhs.cmp(rhs))
}

pub fn par_merge_by_key<T, K, F>(lslice: &[T], rslice: &[T], key: F) -> Vec<T>
where
    T: Clone + Sync + Send,
    F: Fn(&T) -> K + Sync,
    K: Ord,
{
    par_merge_by(lslice, rslice, |lhs, rhs| key(lhs).cmp(&key(rhs)))
}

pub fn par_merge_by<'a, T, F>(mut lslice: &'a [T], mut rslice: &'a [T], compare: F) -> Vec<T>
where
    T: Clone + Sync + Send,
    F: Fn(&T, &T) -> Ordering + Sync,
    &'a [T]: IntoParallelIterator<Item = &'a T>,
{
    if lslice.is_empty() {
        return rslice.to_vec();
    }

    if rslice.is_empty() {
        return lslice.to_vec();
    }

    if lslice.len() > rslice.len() {
        mem::swap(&mut lslice, &mut rslice);
    }

    let num_parts = rayon::current_num_threads();
    let tgt_len = lslice.len() + rslice.len();
    let mut tgt: Vec<T> = Vec::with_capacity(tgt_len);

    let lchunks: Vec<&[T]> = {
        let lens: Vec<usize> = even_division_lens(lslice.len(), num_parts);
        let (lchunks, _) = partition_by_lens(lslice, lens);
        lchunks
    };

    let (rchunks, rremaining): (Vec<&[T]>, &[T]) = {
        let rindices: Vec<usize> = lchunks
            .par_iter()
            .map(|lchunk| {
                let lval = lchunk.last().unwrap();
                let result = rslice.binary_search_by(|rval| compare(rval, lval));
                match result {
                    Ok(idx) => idx,
                    Err(idx) => idx,
                }
            })
            .collect();

        partition_by_indices(rslice, rindices)
    };
    assert_eq!(lchunks.len(), rchunks.len());

    let (tchunks, tremaining) = {
        let lens = lchunks
            .iter()
            .zip(&rchunks)
            .map(|(lchunk, rchunk)| lchunk.len() + rchunk.len());
        let (tchunks, tremaining) = partition_by_lens_mut(tgt.spare_capacity_mut(), lens);
        assert_eq!(lchunks.len(), tchunks.len());
        (tchunks, tremaining)
    };

    assert_eq!(tchunks.len(), lchunks.len());
    assert_eq!(tremaining.len(), rremaining.len());

    tchunks
        .into_par_iter()
        .zip(lchunks)
        .zip(rchunks)
        .for_each(|args| {
            let ((tblock, lblock), rblock) = args;
            let mut writer = MutSliceWriter::new(tblock);
            seq_merge_by(lblock, rblock, &mut writer, &compare);
        });

    // Copy remaining parts
    {
        let lens: Vec<usize> = even_division_lens(tremaining.len(), num_parts);
        let (r_remaining_chunks, _) = partition_by_lens(rremaining, lens.iter().cloned());
        let (t_remaining_chunks, _) = partition_by_lens_mut(tremaining, lens);

        t_remaining_chunks
            .into_par_iter()
            .zip(r_remaining_chunks)
            .for_each(|(tblock, rblock)| {
                let mut writer = MutSliceWriter::new(tblock);
                writer.extend(rblock.iter().cloned());
            });
    }

    unsafe {
        tgt.set_len(tgt_len);
    }

    tgt
}
