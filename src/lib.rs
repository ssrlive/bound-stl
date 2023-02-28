#![doc = include_str!("../readme.md")]

use std::collections::*;
use std::{cmp::Ordering, result::Result};

/// find first index where arr[idx] >= v; assume arr is sorted.
/// it is a encapsulation of `position` method, like `position(|e| e.cmp(x) != Ordering::Less)`,
/// but it returns `Err` when all elements are less than `x`.
pub trait LowerBound {
    type Item;
    fn lower_bound(&self, x: &Self::Item) -> Result<usize, usize>;

    fn lower_bound_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering;

    fn lower_bound_by_key<'a, K, F>(&'a self, k: &K, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord;
}

/// find first index where arr[idx] > v; assume arr is sorted.
/// it is a encapsulation of `position` method, like `position(|e| e.cmp(x) == Ordering::Greater)`,
/// but it returns `Err` when all elements are less than or equal to `x`.
pub trait UpperBound {
    type Item;
    fn upper_bound(&self, x: &Self::Item) -> Result<usize, usize>;

    fn upper_bound_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering;

    fn upper_bound_by_key<'a, K, F>(&'a self, k: &K, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord;
}

impl<T: Ord> LowerBound for [T] {
    type Item = T;
    fn lower_bound(&self, x: &Self::Item) -> Result<usize, usize> {
        self.lower_bound_by(|y| y.cmp(x))
    }

    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        // self.iter().position(|e| f(e) != Ordering::Less).ok_or(self.len())

        let mut left = 0;
        let len = self.len();
        let mut right = len;
        while left < right {
            let mid = left + (right - left) / 2;
            match f(&self[mid]) {
                Ordering::Less => left = mid + 1,
                _ => right = mid,
            }
        }
        assert_eq!(left, right);
        if left == len {
            Err(left)
        } else {
            Ok(left)
        }
    }

    fn lower_bound_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.lower_bound_by(|e| f(e).cmp(k))
    }
}

impl<T: Ord> UpperBound for [T] {
    type Item = T;
    fn upper_bound(&self, x: &Self::Item) -> Result<usize, usize> {
        self.upper_bound_by(|y| y.cmp(x))
    }

    fn upper_bound_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        // self.iter().position(|e| f(e) == Ordering::Greater).ok_or(self.len())

        let mut left = 0;
        let len = self.len();
        let mut right = len;
        while left < right {
            let mid = left + (right - left) / 2;
            match f(&self[mid]) {
                Ordering::Greater => right = mid,
                _ => left = mid + 1,
            }
        }
        assert_eq!(left, right);
        if left == len {
            Err(left)
        } else {
            Ok(left)
        }
    }

    fn upper_bound_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.upper_bound_by(|e| f(e).cmp(k))
    }
}

impl<T: Ord> LowerBound for Vec<T> {
    type Item = T;
    fn lower_bound(&self, x: &Self::Item) -> Result<usize, usize> {
        self.as_slice().lower_bound(x)
    }

    fn lower_bound_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        self.as_slice().lower_bound_by(f)
    }

    fn lower_bound_by_key<'a, K, F>(&'a self, k: &K, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.as_slice().lower_bound_by_key(k, f)
    }
}

impl<T: Ord> UpperBound for Vec<T> {
    type Item = T;
    fn upper_bound(&self, x: &Self::Item) -> Result<usize, usize> {
        self.as_slice().upper_bound(x)
    }

    fn upper_bound_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        self.as_slice().upper_bound_by(f)
    }

    fn upper_bound_by_key<'a, K, F>(&'a self, k: &K, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.as_slice().upper_bound_by_key(k, f)
    }
}

/// To use `lower_bound` and `upper_bound` on `VecDeque`,
/// you need to call `VecDeque::make_contiguous` first and sort it.
/// ```
/// # use bound_stl::{LowerBound, UpperBound};
/// # use std::collections::VecDeque;
/// let mut v = VecDeque::new();
/// v.push_back(1);
/// v.push_back(2);
/// v.push_back(3);
/// assert_eq!(v.lower_bound(&2), Ok(1));
/// assert_eq!(v.upper_bound(&2), Ok(2));
/// assert_eq!(v.upper_bound(&0), Ok(0));
/// assert_eq!(v.lower_bound(&4), Err(3));
/// ```
impl<T: Ord> LowerBound for VecDeque<T> {
    type Item = T;
    fn lower_bound(&self, x: &Self::Item) -> Result<usize, usize> {
        self.as_slices().0.lower_bound(x)
    }

    fn lower_bound_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        self.as_slices().0.lower_bound_by(f)
    }

    fn lower_bound_by_key<'a, K, F>(&'a self, k: &K, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.as_slices().0.lower_bound_by_key(k, f)
    }
}

impl<T: Ord> UpperBound for VecDeque<T> {
    type Item = T;
    fn upper_bound(&self, x: &Self::Item) -> Result<usize, usize> {
        self.as_slices().0.upper_bound(x)
    }

    fn upper_bound_by<'a, F>(&'a self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        self.as_slices().0.upper_bound_by(f)
    }

    fn upper_bound_by_key<'a, K, F>(&'a self, k: &K, f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.as_slices().0.upper_bound_by_key(k, f)
    }
}

impl<T: Ord> LowerBound for BinaryHeap<T> {
    type Item = T;
    fn lower_bound(&self, x: &Self::Item) -> Result<usize, usize> {
        self.lower_bound_by(|y| y.cmp(x))
    }

    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        self.iter().position(|y| f(y) != Ordering::Less).ok_or(self.len())
    }

    fn lower_bound_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.lower_bound_by(|y| f(y).cmp(k))
    }
}

impl<T: Ord> UpperBound for BinaryHeap<T> {
    type Item = T;
    fn upper_bound(&self, x: &Self::Item) -> Result<usize, usize> {
        self.upper_bound_by(|y| y.cmp(x))
    }

    fn upper_bound_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        self.iter().position(|y| f(y) == Ordering::Greater).ok_or(self.len())
    }

    fn upper_bound_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.upper_bound_by(|y| f(y).cmp(k))
    }
}

impl<T: Ord, V> LowerBound for BTreeMap<T, V> {
    type Item = T;
    fn lower_bound(&self, x: &Self::Item) -> Result<usize, usize> {
        self.lower_bound_by(|y| y.cmp(x))
    }

    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        self.iter().position(|(y, _)| f(y) != Ordering::Less).ok_or(self.len())
    }

    fn lower_bound_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.lower_bound_by(|y| f(y).cmp(k))
    }
}

impl<T: Ord, V> UpperBound for BTreeMap<T, V> {
    type Item = T;
    fn upper_bound(&self, x: &Self::Item) -> Result<usize, usize> {
        self.upper_bound_by(|y| y.cmp(x))
    }

    fn upper_bound_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        self.iter()
            .position(|(y, _)| f(y) == Ordering::Greater)
            .ok_or(self.len())
    }

    fn upper_bound_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.upper_bound_by(|y| f(y).cmp(k))
    }
}

impl<T: Ord> LowerBound for BTreeSet<T> {
    type Item = T;
    fn lower_bound(&self, x: &Self::Item) -> Result<usize, usize> {
        self.lower_bound_by(|y| y.cmp(x))
    }

    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        self.iter().position(|y| f(y) != Ordering::Less).ok_or(self.len())
    }

    fn lower_bound_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.lower_bound_by(|y| f(y).cmp(k))
    }
}

impl<T: Ord> UpperBound for BTreeSet<T> {
    type Item = T;
    fn upper_bound(&self, x: &Self::Item) -> Result<usize, usize> {
        self.upper_bound_by(|y| y.cmp(x))
    }

    fn upper_bound_by<'a, F>(&'a self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        self.iter().position(|y| f(y) == Ordering::Greater).ok_or(self.len())
    }

    fn upper_bound_by_key<'a, K, F>(&'a self, k: &K, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&'a Self::Item) -> K,
        K: Ord,
    {
        self.upper_bound_by(|y| f(y).cmp(k))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    use bound_stl::{LowerBound, UpperBound};

    #[test]
    fn test_lower_bound() {
        let v = Vec::<i32>::new();
        assert_eq!(v.lower_bound(&0), Err(0));
        assert_eq!(v.lower_bound(&1), Err(0));

        let v = vec![1, 2, 4, 5, 5, 6, 6];
        assert_eq!(v.lower_bound(&0), Ok(0));
        assert_eq!(v.lower_bound(&1), Ok(0));
        assert_eq!(v.lower_bound(&2), Ok(1));
        assert_eq!(v.lower_bound(&3), Ok(2));
        assert_eq!(v.lower_bound(&4), Ok(2));
        assert_eq!(v.lower_bound(&5), Ok(3));
        assert_eq!(v.lower_bound(&6), Ok(5));
        assert_eq!(v.lower_bound(&7), Err(7));
        assert_eq!(v.lower_bound(&8), Err(7));
        assert_eq!(v.lower_bound(&9), Err(7));
    }

    #[test]
    fn test_upper_bound() {
        let v = Vec::<i32>::new();
        assert_eq!(v.upper_bound(&0), Err(0));
        assert_eq!(v.upper_bound(&1), Err(0));

        let v = vec![-1, 1, 2, 4, 5, 5, 6, 6];
        assert_eq!(v.upper_bound(&0), Ok(1));
        assert_eq!(v.upper_bound(&1), Ok(2));
        assert_eq!(v.upper_bound(&2), Ok(3));
        assert_eq!(v.upper_bound(&3), Ok(3));
        assert_eq!(v.upper_bound(&4), Ok(4));
        assert_eq!(v.upper_bound(&5), Ok(6));
        assert_eq!(v.upper_bound(&6), Err(8));
        assert_eq!(v.upper_bound(&7), Err(8));
        assert_eq!(v.upper_bound(&8), Err(8));
        assert_eq!(v.upper_bound(&9), Err(8));
    }

    #[test]
    fn test_lower_bound_btreeset() {
        use std::collections::BTreeSet;
        let v = BTreeSet::<i32>::new();
        assert_eq!(v.lower_bound(&0), Err(0));
        assert_eq!(v.lower_bound(&1), Err(0));

        let v = vec![1, 2, 4, 5, 5, 6, 6].into_iter().collect::<BTreeSet<_>>();
        assert_eq!(format!("{v:?}"), "{1, 2, 4, 5, 6}");
        assert_eq!(v.lower_bound(&0), Ok(0));
        assert_eq!(v.lower_bound(&1), Ok(0));
        assert_eq!(v.lower_bound(&2), Ok(1));
        assert_eq!(v.lower_bound(&3), Ok(2));
        assert_eq!(v.lower_bound(&4), Ok(2));
        assert_eq!(v.lower_bound(&5), Ok(3));
        assert_eq!(v.lower_bound(&6), Ok(4));
        assert_eq!(v.lower_bound(&7), Err(5));
        assert_eq!(v.lower_bound(&8), Err(5));
        assert_eq!(v.lower_bound(&9), Err(5));
    }
}
