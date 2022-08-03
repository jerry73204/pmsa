use std::cmp::{Ordering, Ordering::*};

/// Sequentially merges two sorted slices.
pub fn seq_merge<T, E>(lslice: &[T], rslice: &[T], tgt: &mut E)
where
    T: Clone + Ord,
    E: Extend<T>,
{
    seq_merge_by(lslice, rslice, tgt, |lhs, rhs| lhs.cmp(rhs))
}

/// Sequentially merges two sorted slices with a custom key function.
pub fn seq_merge_by_key<T, E, K, F>(lslice: &[T], rslice: &[T], tgt: &mut E, key: F)
where
    T: Clone,
    E: Extend<T>,
    F: Fn(&T) -> K,
    K: Ord,
{
    seq_merge_by(lslice, rslice, tgt, |lhs, rhs| key(lhs).cmp(&key(rhs)))
}

/// Sequentially merges two sorted slices with a custom comparison function.
pub fn seq_merge_by<T, E, F>(mut lslice: &[T], mut rslice: &[T], tgt: &mut E, compare: F)
where
    T: Clone,
    E: Extend<T>,
    F: Fn(&T, &T) -> Ordering,
{
    loop {
        if lslice.is_empty() {
            while !rslice.is_empty() {
                tgt.extend([rslice[0].clone()]);
                rslice = &rslice[1..];
            }

            break;
        } else if rslice.is_empty() {
            while !lslice.is_empty() {
                tgt.extend([lslice[0].clone()]);
                lslice = &lslice[1..];
            }

            break;
        } else {
            let litem = &lslice[0];
            let ritem = &rslice[0];

            match compare(litem, ritem) {
                Less | Equal => {
                    tgt.extend([litem.clone()]);
                    lslice = &lslice[1..];
                }
                Greater => {
                    tgt.extend([ritem.clone()]);
                    rslice = &rslice[1..];
                }
            }
        }
    }
}
