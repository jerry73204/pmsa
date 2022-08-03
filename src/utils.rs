use std::mem::MaybeUninit;

pub fn partition_by_indices<'a, T, I>(slice: &'a [T], indices: I) -> (Vec<&'a [T]>, &'a [T])
where
    I: IntoIterator<Item = usize>,
{
    let lens = indices.into_iter().scan(0, |prev, curr| {
        let len = curr - *prev;
        *prev = curr;
        Some(len)
    });
    partition_by_lens(slice, lens)
}

pub fn partition_by_lens<'a, T, I>(mut slice: &'a [T], lens: I) -> (Vec<&'a [T]>, &'a [T])
where
    I: IntoIterator<Item = usize>,
{
    let mut chunks = vec![];

    for len in lens {
        let (lslice, rslice) = slice.split_at(len);
        chunks.push(lslice);
        slice = rslice;
    }
    let remaining = slice;

    (chunks, remaining)
}

#[allow(dead_code)]
pub fn partition_by_indices_mut<'a, T, I>(
    slice: &'a mut [T],
    indices: I,
) -> (Vec<&'a mut [T]>, &'a mut [T])
where
    I: IntoIterator<Item = usize>,
{
    let lens = indices.into_iter().scan(0, |prev, curr| {
        let len = curr - *prev;
        *prev = curr;
        Some(len)
    });
    partition_by_lens_mut(slice, lens)
}

pub fn partition_by_lens_mut<'a, T, I>(
    mut slice: &'a mut [T],
    lens: I,
) -> (Vec<&'a mut [T]>, &'a mut [T])
where
    I: IntoIterator<Item = usize>,
{
    let mut chunks = vec![];

    for len in lens {
        let (lslice, rslice) = slice.split_at_mut(len);
        chunks.push(lslice);
        slice = rslice;
    }

    let remaining = slice;

    (chunks, remaining)
}

#[repr(transparent)]
pub struct MutSliceWriter<'a, T>(Option<&'a mut [MaybeUninit<T>]>);

impl<'a, T> MutSliceWriter<'a, T> {
    pub fn new(slice: &'a mut [MaybeUninit<T>]) -> Self {
        Self(Some(slice))
    }

    // pub fn take_remaining(self) -> &'a mut [MaybeUninit<T>] {
    //     self.0.unwrap()
    // }
}

impl<'a, T> Extend<T> for MutSliceWriter<'a, T> {
    fn extend<A>(&mut self, iter: A)
    where
        A: IntoIterator<Item = T>,
    {
        let mut slice = self.0.take().unwrap();

        for item in iter {
            let (first, remain) = slice.split_first_mut().expect("The slice is depleted");
            slice = remain;
            first.write(item);
        }

        self.0 = Some(slice);
    }
}

pub fn even_division_lens(len: usize, num_parts: usize) -> Vec<usize> {
    let quot = len / num_parts;
    let rem = len % num_parts;
    let mut lens: Vec<_> = (0..rem).map(|_| quot + 1).collect();
    if quot > 0 {
        lens.extend((rem..num_parts).map(|_| quot));
    }
    lens
}
