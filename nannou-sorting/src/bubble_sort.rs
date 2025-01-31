use std::fmt::Debug;
use std::fmt;

struct BubbleSort<'a, T: PartialOrd + Debug> {
    v: &'a mut Vec<T>,
    outer_max: u32,
    outer_idx: u32,
    inner_idx: u32,
}

impl<'a, T> BubbleSort<'a, T>
where T: PartialOrd + Debug 
{
    fn new(v: &'a mut Vec<T>) -> Self {
        let v_len = v.len() as u32;
        let ret_val = BubbleSort {
            v,
            outer_max: v_len,
            outer_idx: 0,
            inner_idx: 0,
        };

        ret_val
    }
}

impl<'a, T> fmt::Display for BubbleSort<'a, T>
where T: PartialOrd + Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}\nouter_max: {}\nouter_idx: {}\ninner_idx: {}\n",
               self.v,
               self.outer_max,
               self.outer_idx,
               self.inner_idx)
    }
}

// Big-O(n^2)
// Visualization - https://www.hackerearth.com/practice/algorithms/sorting/bubble-sort/visualize/
pub fn proto_bubble_sort <T: PartialOrd + Debug> (v: &mut [T]) {
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i+1] {
                // std::mem::swap() - Swaps the value at two mutable locations,
                // without deinitalizing either one.
                v.swap(i, i+1);
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
        /*let mut v0: Vec<u32> = Vec::new();
        let bs0 = BubbleSort::new(&mut v0);

        assert_eq!(bs0.v.get(0), None);
        assert_eq!(bs0.outer_max, 0);*/

        let mut v1 = vec![4, 2, 1];
        let bs1 = BubbleSort::new(&mut v1);

        assert_eq!(bs1.v.get(0), Some(&4));
        assert_eq!(bs1.v.get(1), Some(&2));
        assert_eq!(bs1.v.get(2), Some(&1));
        assert_eq!(bs1.v.get(3), None);

        assert_eq!(bs1.outer_max, 3);
        assert_eq!(bs1.outer_idx, 0);
        assert_eq!(bs1.inner_idx, 0);
    }
}
