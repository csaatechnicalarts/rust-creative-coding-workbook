use std::fmt;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum BubbleSortError {
    EmptyVecToSort,
}

#[derive(Debug, PartialEq)]
pub struct BubbleSort<'a, T>
where
    T: PartialOrd + Debug,
{
    v: &'a mut Vec<T>,
    v_len: u32,
    outer_idx: u32,
    inner_idx: u32,
    sorted: bool,
}

impl<'a, T> BubbleSort<'a, T>
where
    T: PartialOrd + Debug,
{
    pub fn new(v: &'a mut Vec<T>) -> Result<Self, BubbleSortError> {
        if v.len() > 0 {
            let v_len = v.len() as u32;
            let bubble_sort = BubbleSort {
                v,
                v_len,
                outer_idx: 0,
                inner_idx: 0,
                sorted: false,
            };

            Ok(bubble_sort)
        } else {
            Err(BubbleSortError::EmptyVecToSort)
        }
    }

    pub fn get_vec(&'a self) -> &'a Vec<T> {
        self.v
    }

    pub fn algo_next(&mut self) -> bool {
        if !self.sorted && (self.outer_idx < self.v_len) {
            if self.inner_idx < (self.v_len - 1 - self.outer_idx) {
                let idx = self.inner_idx;
                if self.v[self.inner_idx as usize] > self.v[self.inner_idx as usize + 1] {
                    self.v.swap(idx as usize, idx as usize + 1);
                }
                self.inner_idx = self.inner_idx + 1;
            } else {
                self.inner_idx = 0;
                self.outer_idx = self.outer_idx + 1;
            }

            self.sorted = false
        } else {
            self.sorted = true
        }

        self.sorted
    }
}

impl<'a, T> fmt::Display for BubbleSort<'a, T>
where
    T: PartialOrd + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:#?}\nv_len: {}\nouter_idx: {}\ninner_idx: {}\n",
            self.v, self.v_len, self.outer_idx, self.inner_idx
        )
    }
}

// Big-O(n^2)
// Visualization - https://www.hackerearth.com/practice/algorithms/sorting/bubble-sort/visualize/
pub fn proto_bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i + 1] {
                // std::mem::swap() - Swaps the value at two mutable locations,
                // without deinitalizing either one.
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proto_bubble_sort() {
        let mut v = vec![2, 13, 4, 7, 8, 1, 5];
        proto_bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 4, 5, 7, 8, 13]);
    }

    #[test]
    fn test_proto_bubble_sort_contra() {
        let mut a = vec!['a', 'x', 'm', 'n', 'h', 'c'];
        proto_bubble_sort(&mut a);
        assert_ne!(a, vec!['a']);
    }

    #[test]
    fn test_bubble_sort_constructor() {
        let mut v0: Vec<u32> = Vec::new();
        let bs0 = BubbleSort::new(&mut v0);
        assert_eq!(bs0, Err(BubbleSortError::EmptyVecToSort));

        let mut v1 = vec![4, 2, 1];
        let bs1 = BubbleSort::new(&mut v1);
        assert_ne!(bs1, Err(BubbleSortError::EmptyVecToSort));

        let bs = bs1.unwrap();
        assert_eq!(bs.v, &vec![4, 2, 1]);
        assert_ne!(bs.v.get(0), Some(&u32::MAX));
        assert_eq!(bs.v.get(0), Some(&4));
        assert_eq!(bs.v.get(1), Some(&2));
        assert_eq!(bs.v.get(2), Some(&1));
        assert_eq!(bs.v.get(3), None);

        assert_eq!(bs.v_len, 3);
        assert_eq!(bs.outer_idx, 0);
        assert_eq!(bs.inner_idx, 0);
    }
}
