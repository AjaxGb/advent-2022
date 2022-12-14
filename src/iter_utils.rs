use arrayvec::ArrayVec;
use std::cmp::Ordering;

pub trait IteratorUtils: Iterator + Sized {
    fn max_n<const N: usize>(self) -> ArrayVec<Self::Item, N>
    where
        Self::Item: Ord;

    fn max_n_by<F, const N: usize>(self, f: F) -> ArrayVec<Self::Item, N>
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering;

    fn max_n_by_key<B: Ord, F, const N: usize>(self, f: F) -> ArrayVec<Self::Item, N>
    where
        F: FnMut(&Self::Item) -> B;

    fn array_chunks_sep<const N: usize, const S: usize>(self) -> ArrayChunksSep<Self, N, S>;
}

#[inline]
fn find_less_than<T, F>(results: &impl AsRef<[T]>, new: &T, f: &mut F) -> Option<usize>
where
    F: FnMut(&T, &T) -> Ordering,
{
    results
        .as_ref()
        .iter()
        .enumerate()
        .find_map(|(i, old)| f(new, old).is_gt().then_some(i))
}

impl<T: Iterator> IteratorUtils for T {
    fn max_n<const N: usize>(self) -> ArrayVec<Self::Item, N>
    where
        Self::Item: Ord,
    {
        self.max_n_by(|a, b| a.cmp(b))
    }

    fn max_n_by_key<B: Ord, F, const N: usize>(self, mut f: F) -> ArrayVec<Self::Item, N>
    where
        F: FnMut(&Self::Item) -> B,
    {
        self.max_n_by(|a, b| f(a).cmp(&f(b)))
    }

    fn max_n_by<F, const N: usize>(mut self, mut f: F) -> ArrayVec<Self::Item, N>
    where
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        let mut results = ArrayVec::new();
        if N == 0 {
            return results;
        }

        // Before results is full
        while let Some(item) = self.next() {
            if let Some(i) = find_less_than(&results, &item, &mut f) {
                // Insert new item before max smaller value
                results.insert(i, item);
            } else {
                results.push(item);
            }

            if results.is_full() {
                break;
            }
        }

        // After results is full
        for item in self {
            if let Some(i) = find_less_than(&results, &item, &mut f) {
                // Drop smallest
                results.pop();
                // Insert new item before max smaller value
                results.insert(i, item);
            }
        }

        results
    }

    fn array_chunks_sep<const N: usize, const S: usize>(self) -> ArrayChunksSep<Self, N, S> {
        assert_ne!(N, 0);
        ArrayChunksSep(self)
    }
}

pub struct ArrayChunksSep<I: Iterator, const N: usize, const S: usize>(I);

impl<I: Iterator, const N: usize, const S: usize> Iterator for ArrayChunksSep<I, N, S> {
    type Item = [I::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        let chunk = self.0.next_chunk().ok()?;
        for _ in 0..S {
            if self.0.next().is_none() {
                break;
            }
        }
        Some(chunk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_n_basic() {
        let result = [7, 9, 18, 0, -1, 8, 8, 9].into_iter().max_n::<3>();
        assert_eq!(result.as_ref(), [18, 9, 9]);
        let result = [728, -19002, -1, -3, -289].into_iter().max_n::<3>();
        assert_eq!(result.as_ref(), [728, -1, -3]);
    }

    #[test]
    fn max_n_zero() {
        let result = [7, 9, 18, 0, -1, 8, 8, 9].into_iter().max_n::<0>();
        assert!(result.is_empty());
    }

    #[test]
    fn max_n_not_enough() {
        let mut values = [7, 9, 18, 0, -1, 8, 8, 9];
        let just_enough = values.iter().copied().max_n::<8>();
        let not_enough = values.iter().copied().max_n::<10>();
        values.sort_by(|a, b| a.cmp(b).reverse());
        assert_eq!(just_enough.as_ref(), values);
        assert_eq!(not_enough.as_ref(), values);
    }
}
