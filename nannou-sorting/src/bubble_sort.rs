use std::fmt::Debug;

// Big-O(n^2)
// Visualization - https://www.hackerearth.com/practice/algorithms/sorting/bubble-sort/visualize/
pub fn bubble_sort <T: PartialOrd + Debug> (v: &mut [T]) {
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
    fn test_bubble_sort() {
        let mut v = vec![2, 13, 4, 7, 8, 1, 5];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 2, 4, 5, 7, 8, 13]);
    }

    #[ignore]
    #[test]
    fn test_bubble_sort_contra() {
        let mut a = vec!['a', 'x', 'm', 'n', 'h', 'c'];
        bubble_sort(&mut a);
        assert_ne!(a, vec!['a']);
    }
}
