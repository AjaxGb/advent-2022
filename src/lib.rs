use arrayvec::ArrayVec;

pub trait IteratorUtils: Iterator + Sized {
    fn max_n<const N: usize>(self) -> ArrayVec<Self::Item, N>
    where
        Self::Item: Ord;
}

#[inline]
fn find_less_than<T: Ord>(results: &impl AsRef<[T]>, new: &T) -> Option<usize> {
    results
        .as_ref()
        .iter()
        .enumerate()
        .find_map(|(i, old)| new.gt(old).then_some(i))
}

impl<T: Iterator> IteratorUtils for T {
    fn max_n<const N: usize>(mut self) -> ArrayVec<Self::Item, N>
    where
        Self::Item: Ord,
    {
        let mut results = ArrayVec::new();
        if N == 0 {
            return results;
        }

        // Before results is full
        while let Some(item) = self.next() {
            if let Some(i) = find_less_than(&results, &item) {
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
            if let Some(i) = find_less_than(&results, &item) {
                // Drop smallest
                results.pop();
                // Insert new item before max smaller value
                results.insert(i, item);
            }
        }

        results
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
