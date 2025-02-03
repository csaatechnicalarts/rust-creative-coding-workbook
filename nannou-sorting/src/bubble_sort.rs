//! **crate:nannou_sorting::BubbleSort** implements the bubble sort algorithm in two versions:
//!
//! - A textbook version of algorithm.
//! - A step-wise implementation with step-forward and step-backward functionalities.
//!
//! Combined with GUI functionality provided by [Nannou,](https://github.com/nannou-org/nannou) we can graphically walk through
//! (or walk back) the steps of the bubble sort algorithm.

use std::fmt::Debug;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum BubbleSortError {
    EmptyVecToSort,
}

/// This type encapsulates the stream of data to sort. In the textbook version of the bubble sort algorithm, two nested loops drive the sorting process forward. For the step-wise implementation here, the BubbleSort type also extracts the two loop indices for keeping track of them globally.
///
/// BubbleSort::swap_events represents a swap action at a particular point in the bubble sort process. The components of the HashMap are as follows:
///
/// - Key - (u32, u32) where the first integer is the index of the bubble sort outer loop (self.outer_idx) and the second one is the inner index (self.inner_idx).
/// - Value - Some(T, T) stores the two swapped values of type T; a None means no swap occurred.

#[derive(Debug, PartialEq)]
pub struct BubbleSort<'a, T>
where
    T: PartialOrd + Debug + Clone,
{
    v: &'a mut Vec<T>,
    v_len: u32,
    outer_idx: u32,
    inner_idx: u32,
    sort_complete: bool,
    swap_events: HashMap<(u32, u32), Option<(T, T)>>,
}

impl<'a, T> BubbleSort<'a, T>
where
    T: PartialOrd + Debug + Clone,
{
    /// The BubbleSort constructor method.
    ///
    /// # Example Usage:
    ///
    /// ```
    /// let mut bs = BubbleSort::new(&mut v);
    /// match bs {
    ///     Ok(bubble_sort) => { // Run the bubble sort algorithm in this block.
    ///        ... 
    ///     },
    ///     _ => { // Handle errors here.
    ///        ... 
    ///     }
    /// }
    /// ```

    pub fn new(v: &'a mut Vec<T>) -> Result<Self, BubbleSortError> {
        if v.len() > 0 {
            let v_len = v.len() as u32;
            let bubble_sort = BubbleSort {
                v,
                v_len,
                outer_idx: 0,
                inner_idx: 0,
                sort_complete: false,
                /*swap_events: SwapEvent {
                    swap_event: HashMap::new(),
                },*/
                swap_events: HashMap::new(),
            };
            Ok(bubble_sort)
        } else {
            Err(BubbleSortError::EmptyVecToSort)
        }
    }

    pub fn get_vec(&'a self) -> &'a Vec<T> {
        self.v
    }

    pub fn is_sorted(&self) -> bool {
        return self.sort_complete;
    }

    /// A step-wise version of the bubble sort algorithm. It is the analog to BubbleSort::algo_prev().
    ///
    /// # Example Usage
    ///
    /// ```
    /// let mut bubble_sort = BubbleSort::new(&mut v); // v is a vector of data to sort
    /// ...
    /// loop {
    ///     if bubble_sort::algo_next() == true {
    ///         ...
    ///         break;
    ///     }
    /// }
    /// ```

    pub fn algo_next(&mut self) -> bool {
        if !self.sort_complete && (self.outer_idx < self.v_len) {
            if self.inner_idx < (self.v_len - 1 - self.outer_idx) {
                if self.v[self.inner_idx as usize] > self.v[self.inner_idx as usize + 1] {
                    self.v
                        .swap(self.inner_idx as usize, self.inner_idx as usize + 1);
                    self.swap_events.insert((self.outer_idx, self.inner_idx), 
                                            Some((self.v[self.inner_idx as usize].clone(), self.v[self.inner_idx as usize + 1].clone())));
                } else {
                    // No swap occured.
                    self.swap_events.insert((self.outer_idx, self.inner_idx), None);
                }
                self.inner_idx = self.inner_idx + 1;
            } else {
                self.inner_idx = 0;
                self.outer_idx = self.outer_idx + 1;
            }

            //self.sort_complete = false
        } else {
            self.sort_complete = true
        }

        self.sort_complete
    }
}

/// Textbook implementation of bubble sort, mainly for reference.
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
    fn test_empty_input() {
        let mut v0: Vec<u32> = Vec::new();
        let bs0 = BubbleSort::new(&mut v0);
        assert_eq!(bs0, Err(BubbleSortError::EmptyVecToSort));
    }

    #[test]
    fn test_bubble_sort_constructor() {
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

    #[test]
    fn test_bubble_sort_step() {
        let mut v = vec![4, 1, 2];
        let bs = BubbleSort::new(&mut v);
        match bs {
            Ok(mut bubble_sort) => {
                println!("\nUnsorted vector\t\t{:?}", bubble_sort.get_vec());
                println!("{:#?}", bubble_sort);

                bubble_sort.algo_next();
                assert_eq!(*bubble_sort.get_vec(), vec![1, 4, 2]);
                println!("{:#?}", bubble_sort);

                bubble_sort.algo_next();
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 4]);
                println!("{:#?}", bubble_sort);

                bubble_sort.algo_next();
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 4]);
                println!("{:#?}", bubble_sort);

                bubble_sort.algo_next();
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 4]);
                println!("{:#?}", bubble_sort);

                bubble_sort.algo_next();
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 4]);
                println!("{:#?}", bubble_sort);

                bubble_sort.algo_next();
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 4]);
                println!("{:#?}", bubble_sort);

                bubble_sort.algo_next();
                assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 4]);
                println!("{:#?}", bubble_sort);

                assert_eq!(bubble_sort.is_sorted(), true);

                println!("Step-wise bubble sort\t{:?}\n", bubble_sort.get_vec());
                println!("{:#?}", bubble_sort);
            }
            Err(BubbleSortError::EmptyVecToSort) => {
                println!("\nEmpty vector, nothing to sort!");
            }
        }
    }

    #[test]
    fn test_pre_sorted_input() {
        let mut v = vec![1, 2, 3];
        let bs = BubbleSort::new(&mut v);
        match bs {
            Ok(mut bubble_sort) => {
                println!("\nPre-sorted vector\t\t{:?}", bubble_sort.get_vec());
                loop {
                    if bubble_sort.algo_next() == true {
                        assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 3]);
                        println!("Step-wise bubble sort\t{:?}\n", bubble_sort.get_vec());

                        break;
                    }
                }
            },
            Err(BubbleSortError::EmptyVecToSort) => {
                println!("\nEmpty vector, nothing to sort!");
            }
        }
    }

    #[test]
    fn test_reverse_sorted_input() {
        let mut v = vec![3, 2, 1];
        let bs = BubbleSort::new(&mut v);
        match bs {
            Ok(mut bubble_sort) => {
                println!("\nReverse sorted vector\t\t{:?}", bubble_sort.get_vec());
                loop {
                    if bubble_sort.algo_next() == true {
                        assert_eq!(*bubble_sort.get_vec(), vec![1, 2, 3]);
                        println!("Step-wise bubble sort\t{:?}\n", bubble_sort.get_vec());

                        break;
                    }
                }
            },
            Err(BubbleSortError::EmptyVecToSort) => {
                println!("\nEmpty vector, nothing to sort!");
            }
        }
    }
}
